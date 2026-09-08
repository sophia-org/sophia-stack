#![cfg(all(feature = "gbm-platform", target_os = "linux"))]

use std::{
    fs::{File, OpenOptions},
    os::{fd::AsFd, unix::fs::MetadataExt},
    path::{Path, PathBuf},
};

use sophia_renderer_native_egl::{
    NativeDmaBufImportFormat, NativeDmaBufPlane, NativeMultiPlaneDmaBufFrame,
    NativePixmapImportProbe, native_dmabuf_cpu_write_access, query_native_dmabuf_import_formats,
};

const WIDTH: u32 = 4;
const HEIGHT: u32 = 3;
const FORMATS: [gbm::Format; 2] = [gbm::Format::Argb8888, gbm::Format::Xrgb8888];

fn open_node(path: &Path) -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .unwrap_or_else(|error| panic!("open render node {}: {error}", path.display()))
}

fn nodes() -> [PathBuf; 2] {
    let paths = ["SOPHIA_TEST_RENDER_NODE", "SOPHIA_TEST_OTHER_RENDER_NODE"].map(|name| {
        PathBuf::from(
            std::env::var_os(name).unwrap_or_else(|| panic!("set {name} to a render node")),
        )
    });
    let ids = paths.each_ref().map(|path| {
        open_node(path)
            .metadata()
            .expect("read render-node identity")
            .rdev()
    });
    assert_ne!(ids[0], ids[1], "select two distinct render nodes");
    paths
}

fn inventory(path: &Path) -> Vec<NativeDmaBufImportFormat> {
    query_native_dmabuf_import_formats(open_node(path))
        .unwrap_or_else(|error| panic!("query import capabilities on {}: {error}", path.display()))
}

fn modifiers(inventory: &[NativeDmaBufImportFormat], format: gbm::Format) -> &[u64] {
    inventory
        .iter()
        .find(|entry| entry.format == format as u32)
        .map_or(&[], |entry| entry.modifiers.as_slice())
}

fn allocate(
    device: &gbm::Device<File>,
    format: gbm::Format,
    modifier: u64,
) -> std::io::Result<gbm::BufferObject<()>> {
    device.create_buffer_object_with_modifiers2(
        WIDTH,
        HEIGHT,
        format,
        std::iter::once(gbm::Modifier::from(modifier)),
        gbm::BufferObjectFlags::RENDERING,
    )
}

fn expected_rgba() -> Vec<u8> {
    (0..HEIGHT)
        .flat_map(|y| {
            (0..WIDTH).flat_map(move |x| {
                [
                    (17 * x + 11) as u8,
                    (29 * y + 13) as u8,
                    (31 * (x + y) + 7) as u8,
                    255,
                ]
            })
        })
        .collect()
}

fn prove_pixels(
    source: &Path,
    destination: &Path,
    mut buffer: gbm::BufferObject<()>,
    format: gbm::Format,
    modifier: u64,
) {
    assert_eq!(buffer.width(), WIDTH);
    assert_eq!(buffer.height(), HEIGHT);
    assert_eq!(buffer.format(), format);
    assert_eq!(
        u64::from(buffer.modifier()),
        modifier,
        "allocation must honor the negotiated modifier"
    );
    let count = buffer.plane_count() as usize;
    assert!((1..=4).contains(&count), "invalid plane count {count}");
    let plane_fds = (0..count)
        .map(|index| {
            buffer
                .fd_for_plane(index as i32)
                .unwrap_or_else(|error| panic!("export plane {index}: {error}"))
        })
        .collect::<Vec<_>>();
    let expected = expected_rgba();

    // GBM maps the BO; balance exporter access even when mapping fails.
    native_dmabuf_cpu_write_access(&plane_fds[0], false).expect("begin CPU write access");
    let written = buffer.map_mut(0, 0, WIDTH, HEIGHT, |mapping| {
        let stride = mapping.stride() as usize;
        let target = mapping.buffer_mut();
        if stride < WIDTH as usize * 4 || target.len() < stride * HEIGHT as usize {
            return Err("mapped extent does not cover the requested pixels");
        }
        target.fill(0);
        for y in 0..HEIGHT as usize {
            for x in 0..WIDTH as usize {
                let input = (y * WIDTH as usize + x) * 4;
                let output = y * stride + x * 4;
                target[output..output + 4].copy_from_slice(&[
                    expected[input + 2],
                    expected[input + 1],
                    expected[input],
                    expected[input + 3],
                ]);
            }
        }
        Ok(())
    });
    let ended = native_dmabuf_cpu_write_access(&plane_fds[0], true);
    written
        .expect("map source BO")
        .expect("write source pixels");
    ended.expect("end CPU write access");

    let frame = NativeMultiPlaneDmaBufFrame {
        width: WIDTH,
        height: HEIGHT,
        format: format as u32,
        modifier,
        plane_count: count as u8,
        planes: std::array::from_fn(|index| {
            plane_fds.get(index).map(|fd| NativeDmaBufPlane {
                fd: fd.as_fd(),
                offset: buffer.offset(index as i32),
                stride: buffer.stride_for_plane(index as i32),
            })
        }),
    };
    let imported =
        NativePixmapImportProbe::new(open_node(destination), frame).unwrap_or_else(|error| {
            panic!(
                "import {} -> {}, format={:#x} modifier={modifier:#x}: {error:?}",
                source.display(),
                destination.display(),
                format as u32,
            )
        });
    assert_eq!(
        imported.read_rgba().expect("read imported pixels"),
        expected,
        "{} -> {}, format={:#x} modifier={modifier:#x}",
        source.display(),
        destination.display(),
        format as u32,
    );
    eprintln!(
        "negotiated_import source={} destination={} format={:#x} modifier={modifier:#x} planes={count} exact_pixels=true",
        source.display(),
        destination.display(),
        format as u32,
    );
}

#[test]
#[ignore = "requires two explicitly selected render nodes; creates no windows"]
fn measured_linear_buffers_import_with_exact_pixels_in_both_directions() {
    let paths = nodes();
    let inventories = paths.each_ref().map(|path| inventory(path));
    let common = FORMATS
        .into_iter()
        .filter(|format| {
            inventories
                .iter()
                .all(|table| modifiers(table, *format).contains(&0))
        })
        .collect::<Vec<_>>();
    assert!(
        !common.is_empty(),
        "no measured common LINEAR XR24/AR24 import pair"
    );
    for (source, destination) in [(0, 1), (1, 0)] {
        let device =
            gbm::Device::new(open_node(&paths[source])).expect("initialize source GBM device");
        for format in common.iter().copied() {
            let buffer = allocate(&device, format, 0).unwrap_or_else(|error| {
                panic!(
                    "allocate negotiated LINEAR {format:?} on {}: {error}",
                    paths[source].display()
                )
            });
            prove_pixels(&paths[source], &paths[destination], buffer, format, 0);
        }
    }
}

#[test]
#[ignore = "requires two render nodes with allocatable explicit non-linear XR24/AR24 modifiers"]
fn measured_non_linear_buffers_import_on_the_allocating_device() {
    for path in nodes() {
        let inventory = inventory(&path);
        let device = gbm::Device::new(open_node(&path)).expect("initialize source GBM device");
        let mut attempts = Vec::new();
        let mut selected = None;
        for format in FORMATS {
            for modifier in modifiers(&inventory, format).iter().copied() {
                if matches!(modifier, 0 | u64::MAX) || modifier == u64::from(gbm::Modifier::Invalid)
                {
                    continue;
                }
                match allocate(&device, format, modifier) {
                    Ok(buffer) => {
                        selected = Some((buffer, format, modifier));
                        break;
                    }
                    Err(error) => attempts.push((format, modifier, error.to_string())),
                }
            }
            if selected.is_some() {
                break;
            }
        }
        let (buffer, format, modifier) = selected.unwrap_or_else(|| {
            panic!(
                "no measured non-linear candidate allocated on {}: {attempts:?}",
                path.display()
            )
        });
        prove_pixels(&path, &path, buffer, format, modifier);
    }
}
