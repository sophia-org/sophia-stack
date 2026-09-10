use super::*;

pub(super) struct PreparedCoreReload {
    pub(super) report: sophia_config::CoreReloadReport,
    pub(super) state: sophia_config::CoreConfigState,
    pub(super) applications: Option<SessionApplicationConfig>,
}

impl PersistentXtermSessionConfig {
    pub(super) fn effective_launch_profile(
        &self,
        startup: &sophia_config::DesktopSessionCandidate,
    ) -> sophia_config::DesktopSessionCandidate {
        let mut effective = startup.clone();
        if let Some(launch) = &self.active_launch_profile {
            effective.applications.clone_from(&launch.applications);
            effective.terminal.clone_from(&launch.terminal);
            effective.browser.clone_from(&launch.browser);
        }
        effective
    }

    /// Publish a core reload only after its application identities resolve with
    /// the active desktop profile and the retained launch overrides.
    pub(super) fn prepare_core_config_reload(
        &self,
        bytes: &[u8],
    ) -> Result<PreparedCoreReload, Box<dyn std::error::Error>> {
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
                let effective = self.effective_launch_profile(profile);
                Ok::<_, Box<dyn std::error::Error>>(
                    self.session_application_overrides.prepare_live_reload(
                        Self::applications_from_core(snapshot)?,
                        &effective,
                        &self.applications.startup,
                    )?,
                )
            })
            .transpose()?;
        Ok(PreparedCoreReload {
            report,
            state: next,
            applications,
        })
    }

    pub(super) fn publish_core_config_reload(
        &mut self,
        prepared: PreparedCoreReload,
    ) -> sophia_config::CoreReloadReport {
        if prepared.report.disposition == sophia_config::ReloadDisposition::Applied {
            self.applications = prepared
                .applications
                .expect("applied core reload has prepared applications");
        }
        self.core_config_state = prepared.state;
        prepared.report
    }

    pub(super) fn reload_core_config(
        &mut self,
        bytes: &[u8],
    ) -> Result<sophia_config::CoreReloadReport, Box<dyn std::error::Error>> {
        let prepared = self.prepare_core_config_reload(bytes)?;
        Ok(self.publish_core_config_reload(prepared))
    }
}

#[path = "../../../tests/support/core_config_reload.rs"]
mod tests;
