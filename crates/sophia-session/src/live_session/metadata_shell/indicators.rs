use super::LiveMetadataShell;
use sophia_protocol::{
    OutputId, ShellIndicator, ShellIndicatorSnapshot, ShellOutputStatus,
    encode_shell_indicator_snapshot,
};

impl LiveMetadataShell {
    /// Republish the indicator set the Engine already holds, plus which output
    /// the user is actually on.
    ///
    /// The active output is carried explicitly because it cannot be inferred
    /// from the indicators: an output that is focused while holding no window
    /// contributes a status and no entries, and that is exactly the case a bar
    /// has to show. It is sourced from committed public policy rather than the
    /// startup activation plan, which records a topology fact rather than live
    /// focus.
    pub(in crate::live_session) fn service_indicators(
        &mut self,
        publication: Option<&sophia_engine::PolicyIndicatorPublication>,
        active_output: Option<OutputId>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.connected || !self.transport.supports_indicators() {
            return Ok(());
        }
        let Some(publication) = publication else {
            return Ok(());
        };
        self.transport.poll_io()?;

        let snapshot = ShellIndicatorSnapshot {
            connection_epoch: self.transport.connection_epoch(),
            generation: publication.generation,
            active_output,
            statuses: publication
                .output_statuses
                .iter()
                .map(|status| ShellOutputStatus {
                    output: status.output,
                    focus_bits: status.focus_bits,
                    layout: status.layout.clone(),
                })
                .collect(),
            indicators: publication
                .indicators
                .iter()
                .map(|indicator| ShellIndicator {
                    output: indicator.output,
                    indicator: indicator.indicator,
                    // Identities are allocated from one, so zero is free to mean
                    // "not activatable" and can never collide with a real action.
                    action: indicator.action.map_or(0, sophia_protocol::WmActionId::raw),
                    slot: indicator.slot,
                    state_bits: indicator.state_bits,
                    label: indicator.label.clone(),
                })
                .collect(),
        };

        // Republishing an unchanged set would wake a shell for nothing on every
        // committed frame.
        if self.indicators.last_published.as_ref() == Some(&snapshot) {
            return Ok(());
        }

        let tx = self.take_transaction()?;
        let frames = encode_shell_indicator_snapshot(tx, &snapshot)
            .map_err(sophia_runtime::ShellTransportError::Codec)?;
        for frame in frames {
            self.transport.send_async(frame)?;
        }
        self.indicators.last_published = Some(snapshot);
        Ok(())
    }
}

impl LiveMetadataShell {
    /// Take one activation the shell sent, if it is still meaningful.
    ///
    /// The shell must name the snapshot it was presenting. A pill clicked
    /// against a set that has since been replaced refers to a view that may
    /// have moved, so it is answered stale rather than applied late. The
    /// action itself is validated again by the WM against what was published;
    /// this check only establishes that the shell was looking at the same
    /// screen the session was.
    pub(in crate::live_session) fn take_indicator_activation(
        &mut self,
    ) -> Result<Option<(sophia_protocol::OutputId, sophia_protocol::WmActionId)>, Box<dyn std::error::Error>>
    {
        if !self.connected || !self.transport.supports_indicator_activation() {
            return Ok(None);
        }
        let Some(frame) = self
            .transport
            .poll_kind(sophia_protocol::IpcMessageKind::ShellIndicatorActivate)?
        else {
            return Ok(None);
        };
        let (tx, activation) = sophia_protocol::decode_shell_indicator_activation(&frame)
            .map_err(sophia_runtime::ShellTransportError::Codec)?;

        let published = self.indicators.last_published.as_ref();
        let current = published.is_some_and(|snapshot| {
            snapshot.connection_epoch == activation.connection_epoch
                && snapshot.generation == activation.snapshot_generation
        });
        let known = current
            && published.is_some_and(|snapshot| {
                snapshot.indicators.iter().any(|indicator| {
                    indicator.output == activation.output
                        && indicator.indicator == activation.indicator
                        && indicator.action == activation.action
                })
            });

        let status = if !current {
            sophia_protocol::ShellIndicatorActivationStatus::Stale
        } else if !known {
            sophia_protocol::ShellIndicatorActivationStatus::Unknown
        } else if activation.action == 0 {
            // A pill published without an action is not activatable, and zero
            // is how that is spelled.
            sophia_protocol::ShellIndicatorActivationStatus::Unauthorized
        } else {
            sophia_protocol::ShellIndicatorActivationStatus::Accepted
        };

        let outcome = sophia_protocol::ShellIndicatorActivationOutcome {
            connection_epoch: self.transport.connection_epoch(),
            snapshot_generation: activation.snapshot_generation,
            event_id: activation.event_id,
            status,
            reason: 0,
        };
        self.transport.send_async(
            sophia_protocol::encode_shell_indicator_activation_outcome(tx, &outcome)
                .map_err(sophia_runtime::ShellTransportError::Codec)?,
        )?;

        if status == sophia_protocol::ShellIndicatorActivationStatus::Accepted {
            return Ok(Some((
                activation.output,
                sophia_protocol::WmActionId::from_raw(activation.action),
            )));
        }
        Ok(None)
    }
}

#[derive(Default)]
pub(in crate::live_session) struct LiveIndicatorState {
    pub(in crate::live_session) last_published: Option<ShellIndicatorSnapshot>,
}
