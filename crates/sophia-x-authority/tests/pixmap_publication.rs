use sophia_protocol::{
    BufferHandle, DRM_FORMAT_ARGB8888, DmaBufDescriptor, DmaBufPlaneDescriptor, NamespaceId, Rect,
    Region, Size, TransactionId,
};
use sophia_x_authority::*;
use std::fs::File;

const NS: NamespaceId = NamespaceId::from_raw(71);
const OTHER: NamespaceId = NamespaceId::from_raw(72);
const SIZE: Size = Size {
    width: 4,
    height: 2,
};
fn id(raw: u64) -> XResourceId {
    XResourceId::new(raw, 1)
}
fn runtime() -> XAuthorityRuntime {
    let mut r = XAuthorityRuntime::new();
    r.set_pixmap_textures_supported(true);
    r
}
fn create(r: &mut XAuthorityRuntime, namespace: NamespaceId, drawable: XResourceId) {
    r.create_pixmap(namespace, drawable, SIZE, 32, 1).unwrap();
}
fn draw(
    r: &mut XAuthorityRuntime,
    namespace: NamespaceId,
    drawable: XResourceId,
    rect: Rect,
    marker: u8,
) {
    let bytes = vec![marker; (rect.width * rect.height * 4) as usize];
    let response = r.apply_put_image(
        TransactionId::from_raw(1),
        namespace,
        drawable,
        Region::single(rect),
        Some(&bytes),
        None,
    );
    assert_eq!(response.outcome, XAuthorityResponseOutcome::Accepted);
}
fn allocation(request: XServerFrontendPixmapAllocation) -> XServerFrontendAllocatedPixmap {
    XServerFrontendAllocatedPixmap {
        descriptor: DmaBufDescriptor {
            handle: BufferHandle::from_raw(request.handle),
            size: request.size,
            format: if request.depth == 32 {
                DRM_FORMAT_ARGB8888
            } else {
                sophia_protocol::DRM_FORMAT_XRGB8888
            },
            modifier: 0,
            plane_count: 1,
            planes: [
                Some(DmaBufPlaneDescriptor {
                    offset: 0,
                    stride: request.size.width as u32 * 4,
                }),
                None,
                None,
                None,
            ],
        },
        plane_fds: vec![File::open("/dev/null").unwrap().into()],
    }
}
fn reserve(
    r: &mut XAuthorityRuntime,
    namespace: NamespaceId,
    drawable: XResourceId,
) -> (XPixmapExportToken, XServerFrontendPixmapAllocation) {
    match r.prepare_pixmap_export(namespace, drawable).unwrap() {
        XPixmapExportPreparation::Allocate { token, request } => (token, request),
        other => panic!("expected reservation, got {other:?}"),
    }
}
fn export(
    r: &mut XAuthorityRuntime,
    namespace: NamespaceId,
    drawable: XResourceId,
) -> XPixmapExportToken {
    let (token, request) = reserve(r, namespace, drawable);
    assert!(
        r.finish_pixmap_export_allocation(token, Some(allocation(request)))
            .unwrap()
    );
    token
}
fn settle(r: &mut XAuthorityRuntime, namespace: NamespaceId) {
    let prefix = r.capture_pixmap_publication_prefix(namespace).unwrap();
    for target in &prefix {
        if let Some(update) = r.take_pixmap_publication_update(*target).unwrap() {
            r.finish_pixmap_publication_update(update.handle, update.revision, true);
        }
        assert!(r.pixmap_publication_target_settled(*target));
    }
    r.release_pixmap_publication_prefix(prefix);
}

#[test]
fn allocations_reserve_distinct_handles_before_either_completion() {
    let mut r = runtime();
    create(&mut r, NS, id(10));
    create(&mut r, NS, id(11));
    let (a, ar) = reserve(&mut r, NS, id(10));
    assert_eq!(
        r.prepare_pixmap_export(NS, id(10)).unwrap(),
        XPixmapExportPreparation::Pending(a)
    );
    let (b, br) = reserve(&mut r, NS, id(11));
    assert_ne!(a.handle, b.handle);
    assert!(r.next_dma_buf_handle() > b.handle.raw());
    assert!(
        r.finish_pixmap_export_allocation(b, Some(allocation(br)))
            .unwrap()
    );
    assert!(
        r.finish_pixmap_export_allocation(a, Some(allocation(ar)))
            .unwrap()
    );
    assert_eq!(r.pixmap_export_buffers(a).unwrap().0.handle, a.handle);
}

#[test]
fn partial_publication_is_ordered_retryable_and_has_a_finite_namespace_prefix() {
    let mut r = runtime();
    create(&mut r, NS, id(20));
    create(&mut r, OTHER, id(21));
    let full = Rect {
        x: 0,
        y: 0,
        width: 4,
        height: 2,
    };
    draw(&mut r, NS, id(20), full, 1);
    export(&mut r, NS, id(20));
    settle(&mut r, NS);
    draw(&mut r, OTHER, id(21), full, 9);
    let foreign = export(&mut r, OTHER, id(21));
    let a = Rect {
        x: 0,
        y: 0,
        width: 1,
        height: 1,
    };
    let b = Rect {
        x: 2,
        y: 0,
        width: 1,
        height: 1,
    };
    draw(&mut r, NS, id(20), a, 2);
    let first = r.capture_pixmap_publication_prefix(NS).unwrap();
    assert_eq!(first.len(), 1);
    assert_ne!(first[0].handle, foreign.handle);
    let update = r.take_pixmap_publication_update(first[0]).unwrap().unwrap();
    assert_eq!(update.patches.len(), 1);
    assert_eq!(update.patches[0].rect, a);
    assert_eq!(update.patches[0].bytes, vec![2; 4]);
    draw(&mut r, NS, id(20), b, 3);
    let second = r.capture_pixmap_publication_prefix(NS).unwrap();
    assert!(r.pixmap_publication_update_pending(second[0]));
    assert!(
        r.take_pixmap_publication_update(second[0])
            .unwrap()
            .is_none()
    );
    r.finish_pixmap_publication_update(update.handle, update.revision, true);
    assert!(r.pixmap_publication_target_settled(first[0]));
    assert!(!r.pixmap_publication_target_settled(second[0]));
    let failed = r
        .take_pixmap_publication_update(second[0])
        .unwrap()
        .unwrap();
    r.finish_pixmap_publication_update(failed.handle, failed.revision, false);
    assert!(!r.pixmap_publication_update_pending(second[0]));
    assert!(!r.pixmap_publication_target_settled(second[0]));
    let retry = r
        .take_pixmap_publication_update(second[0])
        .unwrap()
        .unwrap();
    assert!(retry.revision > failed.revision);
    r.finish_pixmap_publication_update(failed.handle, failed.revision, true);
    assert!(
        r.pixmap_publication_update_pending(second[0]),
        "a stale completion cannot settle its replacement"
    );
    assert!(!r.pixmap_publication_target_settled(second[0]));
    assert_eq!(retry.patches, failed.patches);
    draw(&mut r, NS, id(20), a, 4);
    r.finish_pixmap_publication_update(retry.handle, retry.revision, true);
    assert!(
        r.pixmap_publication_target_settled(second[0]),
        "future writes cannot extend a captured prefix"
    );
    r.release_pixmap_publication_prefix(first);
    r.release_pixmap_publication_prefix(second);
    assert_eq!(r.capture_pixmap_publication_prefix(OTHER).unwrap().len(), 1);
    settle(&mut r, NS);
}

#[test]
fn a_glx_referent_keeps_allocation_completion_off_a_reused_xid() {
    let mut r = runtime();
    create(&mut r, NS, id(30));
    r.create_glx_pixmap(
        NS,
        id(31),
        x_glx_fb_config(2, true).unwrap(),
        id(30),
        None,
        None,
        None,
    )
    .unwrap();
    let (old, request) = reserve(&mut r, NS, id(30));
    assert_eq!(r.free_pixmap(NS, id(30)).unwrap(), None);
    create(&mut r, NS, id(30));
    let replacement = export(&mut r, NS, id(30));
    assert!(
        r.finish_pixmap_export_allocation(old, Some(allocation(request)))
            .unwrap()
    );
    assert_eq!(
        r.dri3_pixmap_buffers(NS, id(31)).unwrap().0.handle,
        old.handle
    );
    assert_eq!(
        r.dri3_pixmap_buffers(NS, id(30)).unwrap().0.handle,
        replacement.handle
    );
    r.destroy_glx_pixmap(NS, id(31)).unwrap();
    assert_eq!(r.take_pending_backing_releases(), vec![old.handle]);
    assert_eq!(r.take_retired_pixmap_registrations(NS), vec![old.handle]);
    assert!(r.take_retired_pixmap_registrations(OTHER).is_empty());
}

#[test]
fn an_unreferenced_late_allocation_is_released_without_adopting_the_replacement() {
    let mut r = runtime();
    create(&mut r, NS, id(40));
    let (old, request) = reserve(&mut r, NS, id(40));
    r.free_pixmap(NS, id(40)).unwrap();
    create(&mut r, NS, id(40));
    let replacement = export(&mut r, NS, id(40));
    assert!(
        !r.finish_pixmap_export_allocation(old, Some(allocation(request)))
            .unwrap()
    );
    assert_eq!(r.take_pending_backing_releases(), vec![old.handle]);
    assert_eq!(
        r.dri3_pixmap_buffers(NS, id(40)).unwrap().0.handle,
        replacement.handle
    );
    assert!(r.pixmap_export_buffers(old).is_err());
}

#[test]
fn invalid_allocation_metadata_cannot_release_another_live_handle() {
    let mut r = runtime();
    create(&mut r, NS, id(50));
    create(&mut r, NS, id(51));
    let live = export(&mut r, NS, id(50));
    let (bad, request) = reserve(&mut r, NS, id(51));
    let mut result = allocation(request);
    result.descriptor.handle = live.handle;
    assert!(
        r.finish_pixmap_export_allocation(bad, Some(result))
            .is_err()
    );
    assert_eq!(r.take_pending_backing_releases(), vec![bad.handle]);
    assert_eq!(r.pixmap_export_buffers(live).unwrap().0.handle, live.handle);
}

#[test]
fn free_waits_for_the_update_and_its_exact_prefix_pin() {
    let mut r = runtime();
    create(&mut r, NS, id(60));
    draw(
        &mut r,
        NS,
        id(60),
        Rect {
            x: 0,
            y: 0,
            width: 4,
            height: 2,
        },
        8,
    );
    let token = export(&mut r, NS, id(60));
    let prefix = r.capture_pixmap_publication_prefix(NS).unwrap();
    let target = prefix[0];
    let update = r.take_pixmap_publication_update(target).unwrap().unwrap();
    r.free_pixmap(NS, id(60)).unwrap();
    assert!(r.take_pending_backing_releases().is_empty());
    assert!(
        r.capture_pixmap_publication_prefix(NS).unwrap().is_empty(),
        "new requests must not pin a dead backing"
    );
    r.finish_pixmap_publication_update(update.handle, update.revision, true);
    assert!(r.pixmap_publication_target_settled(target));
    assert!(r.take_pending_backing_releases().is_empty());
    r.release_pixmap_publication_prefix(prefix);
    assert_eq!(r.take_pending_backing_releases(), vec![token.handle]);
    r.release_pixmap_publication_prefix([target]);
    assert!(r.take_pending_backing_releases().is_empty());
    assert!(!r.pixmap_publication_target_settled(target));
}

#[test]
fn imported_backings_never_enter_the_provider_upload_or_release_queue() {
    let mut r = runtime();
    let imported = r
        .create_dri3_pixmap(NS, id(70), 1, 32, 4, 2, 16, 32, 32)
        .unwrap();
    r.attach_dri3_plane_fds(
        NS,
        id(70),
        vec![std::sync::Arc::new(File::open("/dev/null").unwrap().into())],
    )
    .unwrap();
    assert_eq!(
        r.prepare_pixmap_export(NS, id(70)).unwrap(),
        XPixmapExportPreparation::Ready
    );
    draw(
        &mut r,
        NS,
        id(70),
        Rect {
            x: 0,
            y: 0,
            width: 1,
            height: 1,
        },
        7,
    );
    assert!(r.capture_pixmap_publication_prefix(NS).unwrap().is_empty());
    assert_eq!(r.free_pixmap(NS, id(70)).unwrap(), Some(imported.handle));
    assert!(r.take_pending_backing_releases().is_empty());
    assert_eq!(
        r.take_retired_pixmap_registrations(NS),
        vec![imported.handle]
    );
}

#[test]
fn a_stateless_provider_keeps_blank_exports_but_refuses_cpu_uploads() {
    let mut r = XAuthorityRuntime::new();
    create(&mut r, NS, id(80));
    let token = export(&mut r, NS, id(80));
    r.free_pixmap(NS, id(80)).unwrap();
    assert!(r.take_pending_backing_releases().is_empty());
    assert_eq!(r.take_retired_pixmap_registrations(NS), vec![token.handle]);
    create(&mut r, NS, id(81));
    draw(
        &mut r,
        NS,
        id(81),
        Rect {
            x: 0,
            y: 0,
            width: 1,
            height: 1,
        },
        7,
    );
    assert!(r.prepare_pixmap_export(NS, id(81)).is_err());
}

#[test]
fn captured_prefixes_and_allocation_reservations_are_bounded() {
    let mut r = runtime();
    create(&mut r, NS, id(90));
    draw(
        &mut r,
        NS,
        id(90),
        Rect {
            x: 0,
            y: 0,
            width: 1,
            height: 1,
        },
        1,
    );
    export(&mut r, NS, id(90));
    let mut targets = Vec::new();
    for _ in 0..4096 {
        targets.extend(r.capture_pixmap_publication_prefix(NS).unwrap());
    }
    assert!(r.capture_pixmap_publication_prefix(NS).is_err());
    r.release_pixmap_publication_prefix(targets);
    assert_eq!(r.capture_pixmap_publication_prefix(NS).unwrap().len(), 1);
    let mut r = runtime();
    for index in 0..1024 {
        create(&mut r, NS, id(1000 + index));
        reserve(&mut r, NS, id(1000 + index));
    }
    create(&mut r, NS, id(2024));
    assert!(r.prepare_pixmap_export(NS, id(2024)).is_err());
}

#[test]
fn repeated_damage_keeps_rectangles_and_copied_bytes_bounded() {
    let mut r = runtime();
    let drawable = id(100);
    r.create_pixmap(
        NS,
        drawable,
        Size {
            width: 64,
            height: 1,
        },
        32,
        1,
    )
    .unwrap();
    export(&mut r, NS, drawable);
    for x in 0..40 {
        draw(
            &mut r,
            NS,
            drawable,
            Rect {
                x,
                y: 0,
                width: 1,
                height: 1,
            },
            x as u8,
        );
    }
    let prefix = r.capture_pixmap_publication_prefix(NS).unwrap();
    let update = r
        .take_pixmap_publication_update(prefix[0])
        .unwrap()
        .unwrap();
    assert!(update.patches.len() <= 32);
    assert!(
        update
            .patches
            .iter()
            .map(|patch| patch.bytes.len())
            .sum::<usize>()
            <= 256
    );
    for x in 0..40 {
        let patch = update
            .patches
            .iter()
            .find(|patch| patch.rect.x <= x && x < patch.rect.x + patch.rect.width)
            .unwrap();
        assert_eq!(patch.bytes[((x - patch.rect.x) * 4) as usize], x as u8);
    }
}

#[test]
fn externally_mutable_shm_pixmaps_cannot_export_an_untracked_blank_backing() {
    let mut r = runtime();
    let segment = id(90);
    let pixmap = id(91);
    let glx = id(92);
    let (mapping, _fd) = sophia_sysv_shm::DescriptorMapping::create_sealed(4096).unwrap();
    let mapping = std::sync::Arc::new(sophia_sysv_shm::ClientMapping::Descriptor(mapping));
    r.attach_shm_descriptor_segment(NS, segment, mapping, false, 1)
        .unwrap();
    r.create_shm_pixmap(NS, pixmap, SIZE, 32, 1, segment, 0)
        .unwrap();
    r.create_glx_pixmap(
        NS,
        glx,
        x_glx_fb_config(2, true).unwrap(),
        pixmap,
        None,
        None,
        None,
    )
    .unwrap();
    let next = r.next_dma_buf_handle();
    assert_eq!(
        r.prepare_pixmap_export(NS, pixmap),
        Err(XAuthorityRuntimeError::InvalidResource)
    );
    assert_eq!(
        r.prepare_pixmap_export(NS, glx),
        Err(XAuthorityRuntimeError::InvalidResource)
    );
    r.free_pixmap(NS, pixmap).unwrap();
    assert_eq!(
        r.prepare_pixmap_export(NS, glx),
        Err(XAuthorityRuntimeError::InvalidResource)
    );
    assert_eq!(
        r.next_dma_buf_handle(),
        next,
        "refusal reserves no allocation"
    );
    assert!(r.capture_pixmap_publication_prefix(NS).unwrap().is_empty());
    assert!(r.take_pending_backing_releases().is_empty());
}

#[test]
fn failed_provider_cleanup_keeps_its_capacity_until_the_debt_is_drained() {
    let mut r = runtime();
    for index in 0..1024 {
        let pixmap = id(5000 + index);
        create(&mut r, NS, pixmap);
        export(&mut r, NS, pixmap);
        r.free_pixmap(NS, pixmap).unwrap();
    }
    create(&mut r, NS, id(7000));
    assert!(r.prepare_pixmap_export(NS, id(7000)).is_err());
    let debt = r.take_pending_backing_releases();
    assert_eq!(debt.len(), 1024);
    assert!(
        r.prepare_pixmap_export(NS, id(7000)).is_err(),
        "dequeued releases still own capacity"
    );
    r.restore_pending_backing_releases(debt);
    assert!(r.prepare_pixmap_export(NS, id(7000)).is_err());
    let debt = r.take_pending_backing_releases();
    assert_eq!(debt.len(), 1024);
    r.finish_pixmap_backing_release(debt[0]);
    reserve(&mut r, NS, id(7000));
    create(&mut r, NS, id(7001));
    assert!(
        r.prepare_pixmap_export(NS, id(7001)).is_err(),
        "one successful release frees exactly one slot"
    );
}

#[test]
fn a_pbuffer_destroy_during_allocation_cannot_install_into_a_reused_name() {
    let mut r = runtime();
    let pbuffer = id(9000);
    r.create_glx_pbuffer(NS, pbuffer, 2, SIZE).unwrap();
    let (old, request) = reserve(&mut r, NS, pbuffer);
    r.destroy_glx_pbuffer(NS, pbuffer).unwrap();
    r.create_glx_pbuffer(NS, pbuffer, 2, SIZE).unwrap();
    let fresh = export(&mut r, NS, pbuffer);
    assert!(
        !r.finish_pixmap_export_allocation(old, Some(allocation(request)))
            .unwrap()
    );
    assert_eq!(
        r.dri3_pixmap_buffers(NS, pbuffer).unwrap().0.handle,
        fresh.handle
    );
    assert_eq!(r.take_pending_backing_releases(), vec![old.handle]);
    assert_eq!(r.take_retired_pixmap_registrations(NS), vec![old.handle]);
}
