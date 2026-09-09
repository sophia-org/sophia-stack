use super::*;

impl LiveProductionNativeScanout {
    pub(super) fn invalidate_layout_probes(&mut self) {
        for exporter in &mut self.exporters {
            exporter.set_layout_probe_available(false);
        }
    }

    pub(crate) fn service_layout_probe_cleanup(&mut self) {
        let now = Instant::now();
        for (head, exporter) in self.heads.iter().zip(&mut self.exporters) {
            exporter.expire_layout_probe(now);
            crate::retry_scanout_layout_probe_cleanup(
                self.groups[head.group].session.card(),
                exporter,
            );
        }
    }

    pub(super) fn prepare_layout_probe_turn(&mut self, index: usize) {
        let group = self.heads[index].group;
        // Tests are consecutive in this owner turn. A pending sibling commit
        // could still change omitted KMS state between the two ioctls.
        let available = self.output_topology_preparation.is_none()
            && self.heads[index].enabled
            && self
                .heads
                .iter()
                .filter(|head| head.group == group)
                .all(|head| {
                    head.submitted_sequence.is_none()
                        && head.scanout_submission.is_none()
                        && head.prepared_scanout.is_none()
                })
            && self.render_devices.group_identity_known(group);
        self.exporters[index].set_layout_probe_available(available);
    }

    pub(super) fn layout_probe_cleanup_pending(&self) -> bool {
        self.exporters
            .iter()
            .any(|exporter| exporter.layout_probe_cleanup_pending())
    }
}
