use super::*;

impl PersistentXtermSessionConfig {
    /// Publish a core reload only after its application identities resolve with
    /// the active desktop profile and the retained launch overrides.
    pub(super) fn reload_core_config(
        &mut self,
        bytes: &[u8],
    ) -> Result<sophia_config::CoreReloadReport, Box<dyn std::error::Error>> {
        let mut next = self.core_config_state.clone();
        let report = next.reload(bytes)?;
        let snapshot = match report.disposition {
            sophia_config::ReloadDisposition::Applied => Some(next.active()),
            sophia_config::ReloadDisposition::PendingRestart => Some(
                next.pending_restart()
                    .ok_or("pending core reload has no retained candidate")?,
            ),
            sophia_config::ReloadDisposition::Deferred => None,
            sophia_config::ReloadDisposition::Rejected => {
                return Err("core reload rejected its candidate".into());
            }
        };
        let applications = snapshot
            .map(|snapshot| {
                let slot = self.session_profile.slot();
                let profile = slot
                    .active()
                    .or_else(|| {
                        (slot.participant().phase()
                            == sophia_config::DesktopProfileParticipantPhase::Prepared)
                            .then(|| slot.candidate())
                            .flatten()
                    })
                    .ok_or("core reload requires an active or prepared session profile")?;
                Ok::<_, Box<dyn std::error::Error>>(
                    self.session_application_overrides
                        .prepare(Self::applications_from_core(snapshot)?, profile)?,
                )
            })
            .transpose()?;
        if report.disposition == sophia_config::ReloadDisposition::Applied {
            self.applications = applications.ok_or("applied core reload has no applications")?;
        }
        self.core_config_state = next;
        Ok(report)
    }
}

#[path = "../../../tests/support/core_config_reload.rs"]
mod tests;
