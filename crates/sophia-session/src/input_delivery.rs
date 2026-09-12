//! Session accounting for terminal input delivery outcomes.

use sophia_x_authority::{
    XAuthorityClientInputDelivery, XAuthorityInputDeliveryId, XAuthorityInputDeliveryOutcome,
    XAuthorityInputDeliveryTicket, XAuthorityRoutedInputSender, XServerFrontendClientId,
};
use std::collections::{BTreeMap, BTreeSet};
use std::time::{Duration, Instant};

pub struct InputDeliveryState {
    pub fail_on_client_error: bool,
    pub events_failed: usize,
    pub next: u64,
    pub pending: BTreeMap<XAuthorityInputDeliveryId, PendingInputDelivery>,
    pub recovered_clients: BTreeSet<XServerFrontendClientId>,
    pub events_expected: usize,
    pub events_flushed: usize,
    pub wait_started_at: Option<Instant>,
    pub source: Option<&'static str>,
    pub flush_latency: Option<Duration>,
}

impl Default for InputDeliveryState {
    fn default() -> Self {
        Self {
            fail_on_client_error: true,
            events_failed: 0,
            next: 1,
            pending: BTreeMap::new(),
            recovered_clients: BTreeSet::new(),
            events_expected: 0,
            events_flushed: 0,
            wait_started_at: None,
            source: None,
            flush_latency: None,
        }
    }
}

/// Settle each receipt once. Client failures release their pending barrier but
/// never count as successful flushes. Desktop sessions contain these failures;
/// proof sessions retain strict delivery requirements.
pub fn settle_input_delivery(
    state: &mut InputDeliveryState,
    release_barrier: &mut BTreeSet<XAuthorityInputDeliveryId>,
    delivery: XAuthorityClientInputDelivery,
) -> Result<Option<XAuthorityInputDeliveryOutcome>, XAuthorityClientInputDelivery> {
    let Some(pending) = state.pending.get(&delivery.delivery) else {
        return Ok(None);
    };
    if pending
        .ticket
        .client
        .is_some_and(|client| client != delivery.client)
    {
        return Ok(None);
    }
    state.pending.remove(&delivery.delivery);
    release_barrier.remove(&delivery.delivery);
    match delivery.outcome {
        XAuthorityInputDeliveryOutcome::Flushed => {
            state.events_flushed = state.events_flushed.saturating_add(1);
        }
        XAuthorityInputDeliveryOutcome::TargetGone
        | XAuthorityInputDeliveryOutcome::EpochRevoked => {
            state.events_expected = state.events_expected.saturating_sub(1);
        }
        XAuthorityInputDeliveryOutcome::RouteRejected
        | XAuthorityInputDeliveryOutcome::WriteFailed
        | XAuthorityInputDeliveryOutcome::ClientDisconnected
        | XAuthorityInputDeliveryOutcome::TimedOut => {
            if matches!(
                delivery.outcome,
                XAuthorityInputDeliveryOutcome::ClientDisconnected
                    | XAuthorityInputDeliveryOutcome::TimedOut
            ) {
                state.recovered_clients.insert(delivery.client);
            }
            state.events_failed = state.events_failed.saturating_add(1);
            if state.fail_on_client_error {
                return Err(delivery);
            }
            state.events_expected = state.events_expected.saturating_sub(1);
        }
    }
    Ok(Some(delivery.outcome))
}

#[derive(Clone, Copy, Debug)]
pub struct PendingInputDelivery {
    pub ticket: XAuthorityInputDeliveryTicket,
    pub release_barrier: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum InputDeliveryError {
    MissingTicket(XAuthorityInputDeliveryId),
    ClientFailure(XAuthorityClientInputDelivery),
    ProofTimeout,
}

impl std::fmt::Display for InputDeliveryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "input delivery failure: {self:?}")
    }
}
impl std::error::Error for InputDeliveryError {}

impl InputDeliveryState {
    pub fn track(
        &mut self,
        sender: &XAuthorityRoutedInputSender,
        ids: impl IntoIterator<Item = XAuthorityInputDeliveryId>,
        release_barrier: bool,
    ) -> Result<(), InputDeliveryError> {
        for id in ids {
            let ticket = sender
                .delivery_ticket(id)
                .ok_or(InputDeliveryError::MissingTicket(id))?;
            self.pending.insert(
                id,
                PendingInputDelivery {
                    ticket,
                    release_barrier,
                },
            );
        }
        Ok(())
    }
}
