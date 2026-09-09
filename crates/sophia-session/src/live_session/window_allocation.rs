use super::*;

#[derive(Default)]
pub(super) struct LiveWindowAllocationPublisher {
    generation: u64,
    next_check: Option<Instant>,
    applied: Option<sophia_x_authority::XWindowAllocationPreferences>,
    pending: Option<(
        sophia_x_authority::XWindowAllocationPreferences,
        Receiver<sophia_x_authority::XWindowAllocationUpdate>,
    )>,
}

fn contains_rect(outer: Rect, inner: Rect) -> bool {
    inner.width > 0
        && inner.height > 0
        && inner.x >= outer.x
        && inner.y >= outer.y
        && i64::from(inner.x) + i64::from(inner.width)
            <= i64::from(outer.x) + i64::from(outer.width)
        && i64::from(inner.y) + i64::from(inner.height)
            <= i64::from(outer.y) + i64::from(outer.height)
}

pub(super) fn window_allocation_rows(
    surfaces: &[sophia_protocol::CommittedSurfaceState],
    mapped: &BTreeSet<SurfaceId>,
    bounds: &[(sophia_protocol::OutputId, Rect)],
    preferences: &[sophia_backend_live::LiveOutputAllocationPreference],
) -> Vec<sophia_x_authority::XWindowAllocationPreference> {
    use sophia_x_authority::{
        XDrmDeviceHint, XServerFrontendDmaBufImportFormat, XWindowAllocationPreference,
    };
    // A spanning or ambiguously placed window has no single-output preference.
    surfaces
        .iter()
        .filter(|surface| mapped.contains(&surface.surface))
        .filter_map(|surface| {
            let mut outputs = bounds
                .iter()
                .filter(|(_, bounds)| contains_rect(*bounds, surface.geometry));
            let (output, _) = outputs.next()?;
            if outputs.next().is_some() {
                return None;
            }
            let preference = preferences.iter().find(|row| row.output == *output)?;
            if preference.formats.is_empty() {
                return None;
            }
            Some(XWindowAllocationPreference {
                surface: surface.surface,
                device: XDrmDeviceHint {
                    major: rustix::fs::major(preference.device_number),
                    minor: rustix::fs::minor(preference.device_number),
                },
                identity: preference.identity.map(|identity| {
                    sophia_x_authority::XRenderDeviceIdentity {
                        device: identity.device,
                        inode: identity.inode,
                        device_number: identity.device_number,
                    }
                }),
                formats: preference
                    .formats
                    .iter()
                    .map(|row| XServerFrontendDmaBufImportFormat {
                        format: row.format,
                        modifiers: row.modifiers.clone(),
                    })
                    .collect(),
            })
        })
        .collect()
}

impl LiveWindowAllocationPublisher {
    pub(super) fn poll(
        &mut self,
        now: Instant,
        topology_generation: u64,
        runtime: &LiveProductionVisualRuntime,
        native: &LiveProductionNativeScanout,
        layout: &PersistentLiveLayout,
        outputs: &[sophia_engine::HeadlessOutput],
        service: &SyncSender<XServerFrontendServiceCommand>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some((_, receiver)) = self.pending.as_ref() {
            match receiver.try_recv() {
                Ok(sophia_x_authority::XWindowAllocationUpdate::Applied) => {
                    self.applied = self.pending.take().map(|(snapshot, _)| snapshot);
                }
                Ok(_) => {
                    self.pending = None;
                    self.applied = None;
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => return Ok(()),
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    return Err("window allocation acknowledgement disconnected".into());
                }
            }
        }
        if self.next_check.is_some_and(|deadline| now < deadline) {
            return Ok(());
        }
        self.next_check = Some(now + Duration::from_millis(250));
        let windows = window_allocation_rows(
            runtime.committed_surfaces(),
            &layout.mapped_surfaces,
            &wm_output_bounds(outputs),
            &native.output_allocation_preferences(),
        );
        if self.applied.as_ref().is_some_and(|old| {
            old.topology_generation == topology_generation && old.windows == windows
        }) {
            return Ok(());
        }
        let generation = self
            .generation
            .checked_add(1)
            .ok_or("window allocation generation exhausted")?;
        let snapshot = sophia_x_authority::XWindowAllocationPreferences {
            generation,
            topology_generation,
            windows,
        };
        let (sender, receiver) = sync_channel(1);
        match service.try_send(
            XServerFrontendServiceCommand::UpdateWindowAllocationPreferences {
                snapshot: snapshot.clone(),
                acknowledgement: sender,
            },
        ) {
            Ok(()) => {
                self.generation = generation;
                self.pending = Some((snapshot, receiver));
            }
            Err(std::sync::mpsc::TrySendError::Full(_)) => {}
            Err(std::sync::mpsc::TrySendError::Disconnected(_)) => {
                return Err("window allocation frontend disconnected".into());
            }
        }
        Ok(())
    }
}
