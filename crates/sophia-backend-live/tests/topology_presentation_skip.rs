#![cfg(all(feature = "libdrm-events", feature = "gbm-probe"))]

//! Forcing presentation quiescence for a topology change.
//!
//! The owner waits for every present owner to settle before applying a new
//! output topology, but it cannot make clients stop drawing, and the owners it
//! waits on advance only while it waits. When that wait expires it skips what
//! is outstanding rather than reporting a stall it never tried to clear. Every
//! present given up that way still owes its client feedback.

use sophia_backend_live::{
    LivePresentBufferDisposition, LivePresentProtocolFeedback, LiveProductionAuthorityBatch,
    LiveProductionAuthorityGroup, LiveProductionCursorPresentation, LiveProductionCycleRequest,
    LiveProductionDmaBufRegistration, LiveProductionFenceRegistration,
    LiveProductionPresentDisposition, LiveProductionPresentSubmission, LiveProductionVisualRuntime,
};
use sophia_engine::HeadlessOutput;
use sophia_protocol::{
    AuthorityKind, BufferHandle, BufferSource, DRM_FORMAT_MOD_INVALID, DmaBufDescriptor,
    DmaBufPlaneDescriptor, FenceHandle, LayerSnapshot, OutputId, Rect, Region,
    ResizeSyncCapability, Size, SurfaceId, SurfaceTransaction, SurfaceTransactionReadiness,
    TransactionId, Transform,
};
use sophia_renderer_live::{LIVE_RENDERER_SCANOUT_FORMAT_XRGB8888, LiveProductionCpuScene};
use std::fs::File;
use std::os::fd::OwnedFd;
use std::sync::Arc;

const SIZE: Size = Size {
    width: 64,
    height: 48,
};

fn geometry() -> Rect {
    Rect {
        x: 0,
        y: 0,
        width: SIZE.width,
        height: SIZE.height,
    }
}

fn layer(surface: SurfaceId) -> LayerSnapshot {
    LayerSnapshot {
        input_region: None,
        translation: None,
        output: None,
        surface,
        authority_local_id: None,
        namespace: None,
        stack_rank: 0,
        geometry: geometry(),
        source_size: Size {
            width: (geometry()).width,
            height: (geometry()).height,
        },
        source: BufferSource::None,
        damage: Region::single(geometry()),
        opacity: 1.0,
        crop: None,
        transform: Transform::IDENTITY,
        generation: 0,
        resize_sync: ResizeSyncCapability::ImplicitOnly,
    }
}

fn group(
    transaction: TransactionId,
    surface: SurfaceId,
    handle: BufferHandle,
    disposition: LiveProductionPresentDisposition,
    acquire_fence: Option<FenceHandle>,
) -> LiveProductionAuthorityGroup {
    LiveProductionAuthorityGroup {
        transaction,
        transactions: vec![SurfaceTransaction {
            input_region: None,
            transaction,
            authority: AuthorityKind::SophiaX,
            surface,
            namespace: None,
            target_geometry: geometry(),
            presentation_extent: Size {
                width: (geometry()).width,
                height: (geometry()).height,
            },
            content: sophia_protocol::SurfaceContentSet::singleton(
                BufferSource::DmaBuf {
                    handle: handle.raw(),
                },
                SIZE,
            ),
            damage: Region::single(geometry()),
            readiness: SurfaceTransactionReadiness::Ready,
            timeout_msec: 250,
            previous_committed_generation: 0,
        }],
        cpu_buffer_updates: Vec::new(),
        removed_surfaces: Vec::new(),
        present_submissions: vec![LiveProductionPresentSubmission {
            transaction,
            surface,
            buffer: handle,
            x_offset: 0,
            y_offset: 0,
            acquire_fence,
            idle_fence: None,
            layout_disposition: disposition,
        }],
        software_present_submissions: Vec::new(),
    }
}

fn registration(handle: BufferHandle) -> LiveProductionDmaBufRegistration {
    LiveProductionDmaBufRegistration {
        descriptor: DmaBufDescriptor {
            handle,
            size: SIZE,
            format: LIVE_RENDERER_SCANOUT_FORMAT_XRGB8888,
            modifier: DRM_FORMAT_MOD_INVALID,
            plane_count: 1,
            planes: [
                Some(DmaBufPlaneDescriptor {
                    offset: 0,
                    stride: 256,
                }),
                None,
                None,
                None,
            ],
        },
        plane_fds: vec![Arc::new(OwnedFd::from(
            File::open("/dev/null").expect("DMA-BUF fixture FD"),
        ))],
    }
}

struct Fixture {
    runtime: LiveProductionVisualRuntime,
    scene: LiveProductionCpuScene,
    output: HeadlessOutput,
    layout: [LayerSnapshot; 1],
}

fn fixture(surface: SurfaceId, handles: &[BufferHandle]) -> Fixture {
    let output = HeadlessOutput {
        id: OutputId::from_raw(1),
        size: SIZE,
        scale: 1,
    };
    let mut runtime = LiveProductionVisualRuntime::new(&[output], None).unwrap();
    let mut scene = LiveProductionCpuScene::new(SIZE);
    let layout = [layer(surface)];
    let registrations = LiveProductionAuthorityBatch {
        groups: Vec::new(),
        dma_buf_registrations: handles.iter().copied().map(registration).collect(),
        fence_registrations: Vec::new(),
        released_dma_bufs: Vec::new(),
        released_fences: Vec::new(),
    };
    runtime
        .run_gpu_production_cycle(LiveProductionCycleRequest {
            batch: &registrations,
            scene: &mut scene,
            raised_surface: None,
            focused_surface: None,
            cursor_presentation: LiveProductionCursorPresentation::Software(None),
            defer_frame: false,
            output_descriptors: &[output],
            native_scanout: None,
            wm_update: None,
            presentation_layout: &layout,
            geometry_routed_surfaces: &[],
            chrome_surfaces: &[],
            indicator_publication: None,
            staged_cpu_buffer_handles: &[],
        })
        .unwrap();
    Fixture {
        runtime,
        scene,
        output,
        layout,
    }
}

impl Fixture {
    fn run(&mut self, batch: &LiveProductionAuthorityBatch) {
        self.runtime
            .run_gpu_production_cycle(LiveProductionCycleRequest {
                batch,
                scene: &mut self.scene,
                raised_surface: None,
                focused_surface: None,
                cursor_presentation: LiveProductionCursorPresentation::Software(None),
                defer_frame: false,
                output_descriptors: &[self.output],
                native_scanout: None,
                wm_update: None,
                presentation_layout: &self.layout,
                geometry_routed_surfaces: &[],
                chrome_surfaces: &[],
                indicator_publication: None,
                staged_cpu_buffer_handles: &[],
            })
            .unwrap();
    }

    #[expect(
        dead_code,
        reason = "kept beside the fixture it belongs to; the \
current tests build their batches inline, and removing it would make the \
next one that needs a batch reinvent it"
    )]
    fn batch(&self, group: LiveProductionAuthorityGroup) -> LiveProductionAuthorityBatch {
        LiveProductionAuthorityBatch {
            groups: vec![group],
            dma_buf_registrations: Vec::new(),
            fence_registrations: Vec::new(),
            released_dma_bufs: Vec::new(),
            released_fences: Vec::new(),
        }
    }
}

#[test]
fn topology_skip_settles_runnable_presents_and_reaches_quiescence() {
    let surface = SurfaceId::new(310, 1);
    let handle = BufferHandle::from_raw(311);
    let transaction = TransactionId::from_raw(312);
    let mut fixture = fixture(surface, &[handle]);

    // An acquire fence that never triggers holds the present queued. Its layout
    // state stays runnable, which is precisely what a topology wait blocks on.
    let acquire_handle = FenceHandle::from_raw(313);
    let batch = LiveProductionAuthorityBatch {
        groups: vec![group(
            transaction,
            surface,
            handle,
            LiveProductionPresentDisposition::Immediate,
            Some(acquire_handle),
        )],
        dma_buf_registrations: Vec::new(),
        fence_registrations: vec![LiveProductionFenceRegistration {
            handle: acquire_handle,
            initially_triggered: false,
            fd: Arc::new(sophia_xshmfence::allocate().unwrap()),
        }],
        released_dma_bufs: Vec::new(),
        released_fences: Vec::new(),
    };
    fixture.run(&batch);
    assert!(
        !fixture.runtime.topology_rebind_quiescent(),
        "a queued present is exactly what blocks a rebind"
    );

    let report = fixture.runtime.skip_presentations_for_topology(None);

    assert!(!report.is_empty());
    assert_eq!(
        report
            .skipped_queued
            .saturating_add(usize::from(report.skipped_in_flight.is_some())),
        1
    );
    assert!(
        fixture.runtime.topology_rebind_quiescent(),
        "skipping is only worth doing if it actually reaches quiescence: {}",
        fixture.runtime.topology_rebind_quiescence_report()
    );

    // The client is told its buffer will never reach a screen, rather than
    // being left waiting on feedback that can no longer arrive.
    let mut feedback = Vec::new();
    fixture
        .runtime
        .drain_present_feedback_into(&mut feedback)
        .unwrap();
    let settled = feedback
        .iter()
        .flat_map(|outcome| outcome.feedback.iter())
        .filter(|entry| {
            matches!(
                entry,
                LivePresentProtocolFeedback::Complete {
                    transaction: settled,
                    disposition: LivePresentBufferDisposition::Skipped,
                    ..
                } if *settled == transaction
            )
        })
        .count();
    assert_eq!(settled, 1, "the skipped present owes its client feedback");
    assert_eq!(
        fixture
            .runtime
            .diagnostics()
            .topology_escalation_present_rejections,
        1
    );
}

/// Nothing is skipped when the wait could simply have proceeded.
#[test]
fn topology_skip_is_empty_when_nothing_is_outstanding() {
    let surface = SurfaceId::new(320, 1);
    let handle = BufferHandle::from_raw(321);
    let mut fixture = fixture(surface, &[handle]);

    assert!(fixture.runtime.topology_rebind_quiescent());
    let report = fixture.runtime.skip_presentations_for_topology(None);

    assert!(report.is_empty());
    assert_eq!(
        fixture
            .runtime
            .diagnostics()
            .topology_escalation_present_rejections,
        0
    );
    assert!(fixture.runtime.topology_rebind_quiescent());
}

#[test]
fn skipping_an_escaped_pre_admission_present_settles_it_and_frees_its_surface() {
    // A frame presented before its window mapped is owned by production with no
    // admission claim on it. Skipping it has to do more than drop it from the
    // queue: the client is waiting on that buffer, and a successor for the same
    // surface is waiting behind its content ownership.
    let surface = SurfaceId::new(320, 1);
    let escaped_handle = BufferHandle::from_raw(321);
    let next_handle = BufferHandle::from_raw(322);
    let escaped = TransactionId::from_raw(323);
    let successor = TransactionId::from_raw(324);
    let mut fixture = fixture(surface, &[escaped_handle, next_handle]);

    // An acquire fence that never triggers is only how this fixture keeps a
    // real present queued without a display; the parked case is covered in the
    // scheduler tests.
    let acquire_handle = FenceHandle::from_raw(325);
    let batch = LiveProductionAuthorityBatch {
        groups: vec![group(
            escaped,
            surface,
            escaped_handle,
            LiveProductionPresentDisposition::Immediate,
            Some(acquire_handle),
        )],
        dma_buf_registrations: Vec::new(),
        fence_registrations: vec![LiveProductionFenceRegistration {
            handle: acquire_handle,
            initially_triggered: false,
            fd: Arc::new(sophia_xshmfence::allocate().unwrap()),
        }],
        released_dma_bufs: Vec::new(),
        released_fences: Vec::new(),
    };
    fixture.run(&batch);

    // A second frame for the same surface queues behind the first's content
    // ownership, which is what a client's redraw after the skip looks like.
    let successor_batch = LiveProductionAuthorityBatch {
        groups: vec![group(
            successor,
            surface,
            next_handle,
            LiveProductionPresentDisposition::Immediate,
            None,
        )],
        dma_buf_registrations: Vec::new(),
        fence_registrations: Vec::new(),
        released_dma_bufs: Vec::new(),
        released_fences: Vec::new(),
    };
    fixture.run(&successor_batch);

    // Before the skip the successor owes nothing: it is waiting behind the
    // escaped frame's content ownership, which is the state the skip has to
    // clear. Establishing this first is what makes the assertion afterwards
    // evidence rather than coincidence.
    let mut before = Vec::new();
    fixture
        .runtime
        .drain_present_feedback_into(&mut before)
        .unwrap();
    assert_eq!(
        before
            .iter()
            .flat_map(|outcome| outcome.feedback.iter())
            .filter(|entry| feedback_names(entry, successor))
            .count(),
        0,
        "the successor is blocked behind the escaped frame, not settled"
    );

    let key = sophia_protocol::DmaBufPresentKey {
        transaction: escaped,
        surface,
        buffer: escaped_handle,
    };
    // Removed and settled, not merely removed.
    assert_eq!(fixture.runtime.skip_escaped_pre_admission(key), Some(true));

    let mut feedback = Vec::new();
    fixture
        .runtime
        .drain_present_feedback_into(&mut feedback)
        .unwrap();
    let entries = feedback
        .iter()
        .flat_map(|outcome| outcome.feedback.iter())
        .collect::<Vec<_>>();
    // The client is told the buffer will never reach a screen, and told it is
    // free again. Without both it waits on a completion that cannot arrive --
    // which is the failure this whole path exists to end.
    assert_eq!(
        entries
            .iter()
            .filter(|entry| matches!(
                entry,
                LivePresentProtocolFeedback::Complete {
                    transaction: settled,
                    disposition: LivePresentBufferDisposition::Skipped,
                    ..
                } if *settled == escaped
            ))
            .count(),
        1,
        "the skipped present owes its client a completion"
    );
    assert_eq!(
        entries
            .iter()
            .filter(|entry| matches!(
                entry,
                LivePresentProtocolFeedback::Idle { transaction } if *transaction == escaped
            ))
            .count(),
        1,
        "and owes it the buffer back"
    );

    // The surface is free again. Releasing the buffer is only half the point --
    // the frame waiting behind it has to be able to run, or a client that
    // redraws after the skip is blocked by the frame it just replaced.
    let idle_batch = LiveProductionAuthorityBatch {
        groups: Vec::new(),
        dma_buf_registrations: Vec::new(),
        fence_registrations: Vec::new(),
        released_dma_bufs: Vec::new(),
        released_fences: Vec::new(),
    };
    fixture.run(&idle_batch);
    let mut after = Vec::new();
    fixture
        .runtime
        .drain_present_feedback_into(&mut after)
        .unwrap();
    assert!(
        after
            .iter()
            .flat_map(|outcome| outcome.feedback.iter())
            .any(|entry| feedback_names(entry, successor)),
        "skipping the owner has to let its successor run, not just free a buffer"
    );

    // Asking again is a miss rather than a second settlement, so a request the
    // caller has not yet consumed cannot double-settle the same frame.
    assert_eq!(fixture.runtime.skip_escaped_pre_admission(key), None);
    let mut repeat = Vec::new();
    fixture
        .runtime
        .drain_present_feedback_into(&mut repeat)
        .unwrap();
    assert!(
        repeat.is_empty(),
        "a missed skip must not manufacture feedback"
    );
}

/// Whether one feedback entry concerns this transaction, whatever its outcome.
fn feedback_names(entry: &LivePresentProtocolFeedback, transaction: TransactionId) -> bool {
    match entry {
        LivePresentProtocolFeedback::Complete {
            transaction: named, ..
        }
        | LivePresentProtocolFeedback::Idle {
            transaction: named, ..
        } => *named == transaction,
    }
}
