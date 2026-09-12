// An opaque modifier leaves plane geometry to its vendor, so the descriptors a
// client sends are what bounds its planes. Driven over a real socket with real
// `SCM_RIGHTS` descriptors, including the shape a compression plane arrives in:
// one allocation duplicated across two planes.
#[cfg(unix)]
#[test]
fn opaque_dri3_planes_are_bounded_by_their_descriptors_before_any_allocation() {
    use std::fs::OpenOptions;
    use std::io::{IoSlice, Read, Seek, SeekFrom, Write};
    use std::mem::MaybeUninit;
    use std::net::Shutdown;
    use std::os::fd::{AsFd, BorrowedFd};
    use std::thread;
    use std::time::{SystemTime, UNIX_EPOCH};

    const WIDTH: u16 = 640;
    const HEIGHT: u16 = 360;
    const DEPTH: u8 = 32;
    const COLOR_STRIDE: u16 = 3072;
    const AUX_STRIDE: u32 = 256;
    const COLOR_BYTES: u32 = COLOR_STRIDE as u32 * HEIGHT as u32;
    const ALLOCATION: u32 = 1_572_864;
    const AUX_ALLOCATION: u32 = 4096;
    const PIXMAP: u32 = 0x330911;
    // AMD's vendor byte over an opaque value. Only that it is neither linear nor
    // INVALID decides which path it takes.
    const OPAQUE_MODIFIER: u64 = 0x0200_0000_0000_001b;
    // Every descriptor is handed over at a non-zero position, which bounding it
    // must leave where the client left it.
    const POSITION: u64 = 41;

    #[derive(Clone, Copy)]
    enum Aux {
        SharedWithFirst,
        Own(u64),
        // Reports no length at all.
        Lengthless,
    }

    #[derive(Clone, Copy)]
    enum Expect {
        Admitted,
        Refused(u32),
    }

    fn unique(tag: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "sophia-x-dri3-planes-{tag}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
    }

    fn backing_file(path: &std::path::Path, len: u64) -> std::fs::File {
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .read(true)
            .write(true)
            .open(path)
            .unwrap();
        file.set_len(len).unwrap();
        file.seek(SeekFrom::Start(POSITION)).unwrap();
        file
    }

    fn get_geometry_request(drawable: u32) -> Vec<u8> {
        let mut out = vec![0u8; 8];
        out[0] = 14;
        out[2..4].copy_from_slice(&2u16.to_le_bytes());
        out[4..8].copy_from_slice(&drawable.to_le_bytes());
        out
    }

    fn send_with_fds(stream: &std::os::unix::net::UnixStream, bytes: &[u8], fds: &[BorrowedFd]) {
        let mut space = [MaybeUninit::uninit();
            rustix::cmsg_space!(ScmRights(sophia_protocol::DMA_BUF_MAX_PLANES))];
        let mut ancillary = rustix::net::SendAncillaryBuffer::new(&mut space);
        if !fds.is_empty() {
            assert!(ancillary.push(rustix::net::SendAncillaryMessage::ScmRights(fds)));
        }
        let sent = rustix::net::sendmsg(
            stream,
            &[IoSlice::new(bytes)],
            &mut ancillary,
            rustix::net::SendFlags::empty(),
        )
        .unwrap();
        assert_eq!(sent, bytes.len());
    }

    fn drive(aux: Aux, offsets: [u32; 2], expect: Expect, tag: &str) {
        let socket = unique(&format!("{tag}.sock"));
        let config = XServerFrontendConfig::new(&socket, NamespaceId::from_raw(824)).unwrap();
        let mut frontend = XServerFrontend::bind(config).unwrap();
        let server = thread::spawn(move || frontend.serve_next());

        wait_for_socket(&socket);
        let mut stream = connect_x_socket(&socket);
        stream
            .write_all(&setup_request(XByteOrder::LittleEndian, 11, 0, b"", b""))
            .unwrap();
        read_setup_success(&mut stream, XByteOrder::LittleEndian);

        let color_path = unique(&format!("{tag}.color"));
        let mut color = backing_file(&color_path, u64::from(ALLOCATION));
        let aux_path = unique(&format!("{tag}.aux"));
        let mut aux_file = match aux {
            Aux::SharedWithFirst => None,
            Aux::Own(len) => Some(backing_file(&aux_path, len)),
            Aux::Lengthless => Some(OpenOptions::new().read(true).open("/dev/null").unwrap()),
        };
        // Duplicating the colour descriptor is how one allocation carrying two
        // planes reaches the server.
        let aux_fd = aux_file
            .as_ref()
            .map_or_else(|| color.as_fd(), |file| file.as_fd());

        let mut strides = [0u32; sophia_protocol::DMA_BUF_MAX_PLANES];
        strides[0] = u32::from(COLOR_STRIDE);
        strides[1] = AUX_STRIDE;
        let mut plane_offsets = [0u32; sophia_protocol::DMA_BUF_MAX_PLANES];
        plane_offsets[0] = offsets[0];
        plane_offsets[1] = offsets[1];

        send_with_fds(
            &stream,
            &dri3_pixmap_from_buffers_request(
                XByteOrder::LittleEndian,
                PIXMAP,
                X_SETUP_DEFAULT_ROOT,
                2,
                WIDTH,
                HEIGHT,
                strides,
                plane_offsets,
                DEPTH,
                32,
                OPAQUE_MODIFIER,
            ),
            &[color.as_fd(), aux_fd],
        );
        send_with_fds(&stream, &get_geometry_request(PIXMAP), &[]);

        // A refusal is followed by an ordinary import at the same XID, which
        // only succeeds if the refused one allocated nothing.
        let legacy_path = unique(&format!("{tag}.legacy"));
        let legacy = matches!(expect, Expect::Refused(_))
            .then(|| backing_file(&legacy_path, u64::from(ALLOCATION)));
        if let Some(legacy) = legacy.as_ref() {
            let mut followup = dri3_pixmap_from_buffer_request(
                XByteOrder::LittleEndian,
                PIXMAP,
                X_SETUP_DEFAULT_ROOT,
                COLOR_BYTES,
                WIDTH,
                HEIGHT,
                COLOR_STRIDE,
                DEPTH,
                32,
            );
            followup.extend_from_slice(&get_geometry_request(PIXMAP));
            send_with_fds(&stream, &followup, &[legacy.as_fd()]);
        }

        stream.shutdown(Shutdown::Write).unwrap();
        let mut answer = Vec::new();
        stream.read_to_end(&mut answer).unwrap();
        server.join().unwrap().unwrap();

        assert_eq!(
            answer.len() % 32,
            0,
            "{tag}: answer must be whole 32-byte records, got {} bytes",
            answer.len(),
        );
        let records = answer.chunks_exact(32).collect::<Vec<_>>();
        let errors = records
            .iter()
            .filter(|record| record[0] == 0)
            .collect::<Vec<_>>();
        let replies = records
            .iter()
            .filter(|record| record[0] == 1)
            .collect::<Vec<_>>();
        let codes = errors.iter().map(|record| record[1]).collect::<Vec<_>>();

        // The pixmap answers the geometry it was imported with, whether it was
        // admitted outright or created again after a refusal.
        let assert_pixmap_answers = |reply: &[u8]| {
            assert_eq!(reply[1], DEPTH, "{tag}: queried depth");
            assert_eq!(
                u16::from_le_bytes([reply[16], reply[17]]),
                WIDTH,
                "{tag}: queried width",
            );
            assert_eq!(
                u16::from_le_bytes([reply[18], reply[19]]),
                HEIGHT,
                "{tag}: queried height",
            );
        };

        match expect {
            Expect::Admitted => {
                assert_eq!(
                    codes,
                    Vec::<u8>::new(),
                    "{tag}: planes inside their descriptors are admitted",
                );
                assert_eq!(replies.len(), 1, "{tag}: the imported pixmap answers once");
                assert_pixmap_answers(replies[0]);
            }
            Expect::Refused(offset) => {
                assert_eq!(
                    errors.len(),
                    2,
                    "{tag}: the refusal and the absent drawable are the only errors, got {codes:?}",
                );
                let refusal = errors[0];
                assert_eq!(
                    refusal[1],
                    XErrorCode::BadValue.wire_code(),
                    "{tag}: an out-of-bounds plane is a value error",
                );
                assert_eq!(
                    u16::from_le_bytes([refusal[2], refusal[3]]),
                    1,
                    "{tag}: the refusal carries the sequence of the request it refused",
                );
                assert_eq!(
                    u32::from_le_bytes([refusal[4], refusal[5], refusal[6], refusal[7]]),
                    offset,
                    "{tag}: the error names the offset that no descriptor contains",
                );
                assert_eq!(
                    u16::from_le_bytes([refusal[8], refusal[9]]),
                    u16::from(X_DRI3_PIXMAP_FROM_BUFFERS_MINOR_OPCODE),
                    "{tag}: correlated to the minor it refused",
                );
                assert_eq!(
                    refusal[10], X_DRI3_MAJOR_OPCODE,
                    "{tag}: correlated to DRI3"
                );
                // BadDrawable: the refused id was to be a pixmap, and
                // GetGeometry asks about drawables. Reporting a window error
                // would say the id is the wrong kind of thing rather than that
                // nothing was ever created under it.
                assert_eq!(
                    errors[1][1],
                    XErrorCode::BadDrawable.wire_code(),
                    "{tag}: a refused import leaves its XID uncreated",
                );
                assert_eq!(
                    u32::from_le_bytes([errors[1][4], errors[1][5], errors[1][6], errors[1][7]]),
                    PIXMAP,
                    "{tag}: and it is that XID which is still unknown",
                );
                assert_eq!(
                    replies.len(),
                    1,
                    "{tag}: the connection stays usable and the XID stays free",
                );
                assert_pixmap_answers(replies[0]);
            }
        }

        assert_eq!(
            color.stream_position().unwrap(),
            POSITION,
            "{tag}: bounding a plane must not move the position shared with the client",
        );
        if let Some(file) = aux_file.as_mut()
            && matches!(aux, Aux::Own(_))
        {
            assert_eq!(
                file.stream_position().unwrap(),
                POSITION,
                "{tag}: the auxiliary descriptor keeps its position too",
            );
        }

        std::fs::remove_file(socket).unwrap();
        std::fs::remove_file(color_path).unwrap();
        if legacy.is_some() {
            std::fs::remove_file(legacy_path).unwrap();
        }
        if matches!(aux, Aux::Own(_)) {
            std::fs::remove_file(aux_path).unwrap();
        }
    }

    // A compression plane at a second offset into the colour plane's own
    // allocation, well inside it.
    drive(
        Aux::SharedWithFirst,
        [0, COLOR_BYTES],
        Expect::Admitted,
        "shared-inside",
    );
    // An offset must be strictly inside its descriptor, so the end is refused.
    drive(
        Aux::SharedWithFirst,
        [0, ALLOCATION],
        Expect::Refused(ALLOCATION),
        "shared-at-end",
    );
    drive(
        Aux::SharedWithFirst,
        [0, ALLOCATION + 4096],
        Expect::Refused(ALLOCATION + 4096),
        "shared-past-end",
    );
    // A plane with its own allocation is bounded by that allocation. This offset
    // sits far inside the colour plane and exactly at the end of the auxiliary
    // one, so only a plane-specific length refuses it.
    drive(
        Aux::Own(u64::from(AUX_ALLOCATION)),
        [0, AUX_ALLOCATION],
        Expect::Refused(AUX_ALLOCATION),
        "own-at-end",
    );
    drive(
        Aux::Own(u64::from(AUX_ALLOCATION)),
        [0, 0],
        Expect::Admitted,
        "own-inside",
    );
    // A descriptor with no length bounds nothing, so its plane is refused even
    // at offset zero.
    drive(Aux::Lengthless, [0, 0], Expect::Refused(0), "no-length");
    // So is one larger than the protocol's cap, sparse though it is.
    drive(
        Aux::Own(sophia_protocol::DMA_BUF_MAX_BYTES + 1),
        [0, 0],
        Expect::Refused(0),
        "over-cap",
    );
}
