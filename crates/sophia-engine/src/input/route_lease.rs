use std::collections::BTreeMap;

use sophia_protocol::{
    ApplicationRouteLeaseId, ApplicationRouteLeaseIdentity, ClientAdmissionId, DeviceId,
    NamespaceId, NamespaceProfile, OutputId, SeatId, SurfaceId,
};

pub const APPLICATION_ROUTE_RELEASE_TIMEOUT_MSEC: u64 = 500;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ApplicationRouteScope {
    pub profile: NamespaceProfile,
    pub authority: NamespaceId,
}

impl ApplicationRouteScope {
    pub fn covers(self, candidate: Self) -> bool {
        match self.profile {
            NamespaceProfile::ClassicShared => {
                matches!(candidate.profile, NamespaceProfile::ClassicShared)
            }
            NamespaceProfile::Confined => {
                matches!(candidate.profile, NamespaceProfile::Confined)
                    && self.authority == candidate.authority
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApplicationRouteLeasePhase {
    Provisional,
    Active,
    Releasing { deadline_msec: u64 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApplicationRouteLeaseOrigin {
    PointerBoundary,
    ExplicitPointer,
}

/// What a lease is waiting on before it can route input.
///
/// Informational, never a permit: readiness says a lease is eligible to be
/// checked, not that an event may be delivered. It is never carried across a
/// time boundary -- callers resolve the lease and ask again.
///
/// One decision, shared by the session and by `authorize`, so the two cannot
/// disagree.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApplicationRouteLeaseReadiness {
    Releasing,
    WaitForActivation,
    WaitForPresentation,
    ReadyForEvidenceValidation,
}

/// Where a lease is presented. Orthogonal to phase.
///
/// A grab may be taken before its surface has reached scanout, so the output is
/// not always knowable at creation. `authorize` requires the output to match,
/// so an unbound lease waits rather than guessing one.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApplicationRouteLeaseBinding {
    /// No output yet. `pinned_output` is inherited by a replacement, which may
    /// bind only there. The deadline is absolute from creation and never
    /// extended.
    AwaitingPresentation {
        pinned_output: Option<OutputId>,
        deadline_msec: u64,
    },
    /// Pinned by the first eligible retired evidence on that output.
    ///
    /// `revision` is evidence of the scene last validated against, not a gate:
    /// authorization never refuses on a revision change.
    Bound { output: OutputId, revision: u64 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApplicationRouteLeaseBindingTimeout {
    NotAwaiting,
    Pending,
    Expired(ApplicationRouteLease),
}

/// What the caller must re-resolve before an event is delivered or buffered.
///
/// Gathered fresh per event. Readiness says a lease is eligible to be checked;
/// these are the checks.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ApplicationRouteTargetEvidence {
    /// Scope of the surface now resolved under the pointer. A held lease may
    /// cross application surfaces inside its own scope; leaving that scope is
    /// what ends it.
    pub resolved_scope: ApplicationRouteScope,
    /// The surface this evidence was gathered for. Checked against the lease
    /// target, so evidence about a surface that has since been replaced on the
    /// same seat cannot validate its replacement.
    pub target_surface: SurfaceId,
    /// Freshly resolved admission of the lease target, not the remembered one.
    pub target_admission: ClientAdmissionId,
    /// The scene this evidence was read from. Recorded onto the lease after a
    /// successful check; never a reason to refuse.
    pub presentation_revision: u64,
    /// Whether the target is still presented and reachable, decided by the
    /// caller against the current scene.
    pub target_eligible: bool,
    pub output: OutputId,
    pub device: DeviceId,
    pub authority_session_epoch: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ApplicationRouteLease {
    pub identity: ApplicationRouteLeaseIdentity,
    pub origin: ApplicationRouteLeaseOrigin,
    pub phase: ApplicationRouteLeasePhase,
    pub target_surface: SurfaceId,
    pub admission: ClientAdmissionId,
    pub scope: ApplicationRouteScope,
    pub authority_session_epoch: u64,
    pub binding: ApplicationRouteLeaseBinding,
    pub initiating_device: Option<DeviceId>,
    pub initiating_button: Option<u32>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ApplicationRouteLeaseCandidate {
    pub seat: SeatId,
    pub origin: ApplicationRouteLeaseOrigin,
    pub target_surface: SurfaceId,
    pub admission: ClientAdmissionId,
    pub scope: ApplicationRouteScope,
    pub authority_session_epoch: u64,
    pub binding: ApplicationRouteLeaseBinding,
    pub initiating_device: Option<DeviceId>,
    pub initiating_button: Option<u32>,
}

impl ApplicationRouteLease {
    pub const fn binding(&self) -> ApplicationRouteLeaseBinding {
        self.binding
    }

    /// What this lease is waiting on, if anything.
    ///
    /// Precedence, which the arm order encodes and a test pins: a releasing
    /// lease is finished and never waits on anything else; an unactivated
    /// explicit grab waits for its client, never for pixels; anything unbound
    /// waits for presentation; everything else is eligible to be checked.
    ///
    /// Every combination is spelled out and there is no wildcard, so a new
    /// origin, phase or binding fails to compile here rather than falling
    /// through to routing input.
    ///
    /// An automatic click is routable while provisional: the lease is created
    /// and used within one event.
    pub const fn routing_readiness(&self) -> ApplicationRouteLeaseReadiness {
        use ApplicationRouteLeaseBinding as Binding;
        use ApplicationRouteLeaseOrigin as Origin;
        use ApplicationRouteLeasePhase as Phase;
        use ApplicationRouteLeaseReadiness as Ready;
        match (self.origin, self.phase, self.binding) {
            (
                Origin::PointerBoundary | Origin::ExplicitPointer,
                Phase::Releasing { .. },
                Binding::AwaitingPresentation { .. } | Binding::Bound { .. },
            ) => Ready::Releasing,
            (
                Origin::ExplicitPointer,
                Phase::Provisional,
                Binding::AwaitingPresentation { .. } | Binding::Bound { .. },
            ) => Ready::WaitForActivation,
            (
                Origin::PointerBoundary,
                Phase::Provisional | Phase::Active,
                Binding::AwaitingPresentation { .. },
            )
            | (Origin::ExplicitPointer, Phase::Active, Binding::AwaitingPresentation { .. }) => {
                Ready::WaitForPresentation
            }
            (
                Origin::PointerBoundary,
                Phase::Provisional | Phase::Active,
                Binding::Bound { .. },
            )
            | (Origin::ExplicitPointer, Phase::Active, Binding::Bound { .. }) => {
                Ready::ReadyForEvidenceValidation
            }
        }
    }
}

impl ApplicationRouteLeaseBinding {
    /// Where this binding may still land, if it has not landed already.
    const fn pinned(self) -> Option<OutputId> {
        match self {
            Self::Bound { output, .. } => Some(output),
            Self::AwaitingPresentation { pinned_output, .. } => pinned_output,
        }
    }

    /// The binding a replacement inherits from the lease it replaces.
    ///
    /// A promotion may not reach an output the original could not, and may not
    /// buy itself more time. So the original pin wins over the candidate one,
    /// and the deadline is the earlier of the two: a client that promotes
    /// repeatedly must not be able to hold a seat indefinitely by restarting
    /// the clock.
    fn inherit(self, replacement: Self) -> Result<Self, ApplicationRouteLeaseError> {
        let inherited = self.pinned();
        match replacement {
            Self::Bound { output, revision } => {
                if inherited.is_some_and(|pinned| pinned != output) {
                    return Err(ApplicationRouteLeaseError::StalePresentation);
                }
                Ok(Self::Bound { output, revision })
            }
            Self::AwaitingPresentation {
                pinned_output,
                deadline_msec,
            } => {
                if let (Some(pinned), Some(wanted)) = (inherited, pinned_output)
                    && pinned != wanted
                {
                    return Err(ApplicationRouteLeaseError::StalePresentation);
                }
                let deadline_msec = match self {
                    Self::AwaitingPresentation {
                        deadline_msec: existing,
                        ..
                    } => existing.min(deadline_msec),
                    Self::Bound { .. } => deadline_msec,
                };
                Ok(Self::AwaitingPresentation {
                    pinned_output: inherited.or(pinned_output),
                    deadline_msec,
                })
            }
        }
    }
}

impl ApplicationRouteLeaseCandidate {
    fn is_valid(self) -> bool {
        self.seat.is_valid()
            && self.target_surface.is_valid()
            && self.admission.is_valid()
            && self.scope.authority.is_valid()
            && self.authority_session_epoch != 0
            && match self.binding {
                // A bound candidate must name a real output and scene.
                ApplicationRouteLeaseBinding::Bound { output, revision } => {
                    output.is_valid() && revision != 0
                }
                // An unbound one must carry a deadline, and any inherited pin
                // must be a real output.
                ApplicationRouteLeaseBinding::AwaitingPresentation {
                    pinned_output,
                    deadline_msec,
                } => deadline_msec != 0 && pinned_output.is_none_or(OutputId::is_valid),
            }
            && self.initiating_device.is_none_or(DeviceId::is_valid)
            && self.initiating_button.is_none_or(|button| button != 0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApplicationRouteLeaseError {
    InvalidCandidate,
    SeatAlreadyOwned,
    NoLease,
    IdentityMismatch,
    InvalidPhase,
    /// The lease exists but is not ready to be checked, and says what it waits
    /// on. Carrying the readiness keeps a refusal diagnosable without making
    /// readiness itself an error type.
    NotRoutable(ApplicationRouteLeaseReadiness),
    InvalidOrigin,
    StaleAuthoritySession,
    StaleControlEpoch,
    StalePresentation,
    OutsideScope,
    WrongDevice,
    IdExhausted,
    SequenceExhausted,
    ControlEpochExhausted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApplicationRouteLeaseTimeout {
    NotReleasing,
    Pending,
    Quarantine(ApplicationRouteLease),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApplicationRouteLeaseState {
    control_epoch: u64,
    next_id: u64,
    frontend_sequence: BTreeMap<SeatId, u64>,
    leases: BTreeMap<SeatId, ApplicationRouteLease>,
}

impl Default for ApplicationRouteLeaseState {
    fn default() -> Self {
        Self {
            control_epoch: 1,
            next_id: 1,
            frontend_sequence: BTreeMap::new(),
            leases: BTreeMap::new(),
        }
    }
}

impl ApplicationRouteLeaseState {
    pub fn control_epoch(&self) -> u64 {
        self.control_epoch
    }

    pub fn lease(&self, seat: SeatId) -> Option<ApplicationRouteLease> {
        self.leases.get(&seat).copied()
    }

    pub fn leases(&self) -> impl Iterator<Item = ApplicationRouteLease> + '_ {
        self.leases.values().copied()
    }

    pub fn begin_provisional(
        &mut self,
        candidate: ApplicationRouteLeaseCandidate,
    ) -> Result<ApplicationRouteLease, ApplicationRouteLeaseError> {
        if !candidate.is_valid() {
            return Err(ApplicationRouteLeaseError::InvalidCandidate);
        }
        if self.leases.contains_key(&candidate.seat) {
            return Err(ApplicationRouteLeaseError::SeatAlreadyOwned);
        }
        let id = ApplicationRouteLeaseId::from_raw(self.next_id);
        self.next_id = self
            .next_id
            .checked_add(1)
            .ok_or(ApplicationRouteLeaseError::IdExhausted)?;
        let sequence = self
            .frontend_sequence
            .get(&candidate.seat)
            .copied()
            .unwrap_or(0)
            .checked_add(1)
            .ok_or(ApplicationRouteLeaseError::SequenceExhausted)?;
        self.frontend_sequence.insert(candidate.seat, sequence);
        let lease = ApplicationRouteLease {
            identity: ApplicationRouteLeaseIdentity {
                id,
                seat: candidate.seat,
                frontend_sequence: sequence,
                control_epoch: self.control_epoch,
            },
            origin: candidate.origin,
            phase: ApplicationRouteLeasePhase::Provisional,
            target_surface: candidate.target_surface,
            admission: candidate.admission,
            scope: candidate.scope,
            authority_session_epoch: candidate.authority_session_epoch,
            binding: candidate.binding,
            initiating_device: candidate.initiating_device,
            initiating_button: candidate.initiating_button,
        };
        self.leases.insert(candidate.seat, lease);
        Ok(lease)
    }

    pub fn replace_explicit_provisional(
        &mut self,
        identity: ApplicationRouteLeaseIdentity,
        candidate: ApplicationRouteLeaseCandidate,
    ) -> Result<ApplicationRouteLease, ApplicationRouteLeaseError> {
        if !candidate.is_valid() || candidate.origin != ApplicationRouteLeaseOrigin::ExplicitPointer
        {
            return Err(ApplicationRouteLeaseError::InvalidCandidate);
        }
        let existing = *self.exact_mut(identity)?;
        // The frontend may ask to promote the click before its delivery
        // acknowledgement reaches the owner loop. Its exact lease identity
        // and admission still bind that promotion to the initiating client.
        if existing.phase != ApplicationRouteLeasePhase::Active
            && !(existing.origin == ApplicationRouteLeaseOrigin::PointerBoundary
                && existing.phase == ApplicationRouteLeasePhase::Provisional)
        {
            return Err(ApplicationRouteLeaseError::InvalidPhase);
        }
        if existing.admission != candidate.admission
            || existing.scope != candidate.scope
            || existing.authority_session_epoch != candidate.authority_session_epoch
            || identity.seat != candidate.seat
        {
            return Err(ApplicationRouteLeaseError::IdentityMismatch);
        }
        // The replacement inherits where the original could bind and how long
        // it had left. Passing the candidate through unchanged would let a
        // promotion bind to an output the original never pointed at, and reset
        // its deadline.
        let mut candidate = candidate;
        candidate.binding = existing.binding.inherit(candidate.binding)?;
        self.leases.remove(&identity.seat);
        match self.begin_provisional(candidate) {
            Ok(replacement) => Ok(replacement),
            Err(error) => {
                self.leases.insert(identity.seat, existing);
                Err(error)
            }
        }
    }

    pub fn confirm(
        &mut self,
        identity: ApplicationRouteLeaseIdentity,
        target_surface: SurfaceId,
        admission: ClientAdmissionId,
        authority_session_epoch: u64,
    ) -> Result<ApplicationRouteLease, ApplicationRouteLeaseError> {
        let lease = self.exact_mut(identity)?;
        if lease.target_surface != target_surface || lease.admission != admission {
            return Err(ApplicationRouteLeaseError::IdentityMismatch);
        }
        if lease.authority_session_epoch != authority_session_epoch {
            return Err(ApplicationRouteLeaseError::StaleAuthoritySession);
        }
        if lease.phase != ApplicationRouteLeasePhase::Provisional {
            return Err(ApplicationRouteLeaseError::InvalidPhase);
        }
        lease.phase = ApplicationRouteLeasePhase::Active;
        Ok(*lease)
    }

    pub fn reject(
        &mut self,
        identity: ApplicationRouteLeaseIdentity,
    ) -> Result<ApplicationRouteLease, ApplicationRouteLeaseError> {
        self.remove_exact(identity, ApplicationRouteLeasePhase::Provisional)
    }

    /// What the seat's current lease is waiting on, if it holds one.
    ///
    /// Resolves the lease here rather than accepting one, so a lease that has
    /// since changed cannot be asked about as though it had not. Informational:
    /// it never says an event may be delivered.
    pub fn routing_readiness(&self, seat: SeatId) -> Option<ApplicationRouteLeaseReadiness> {
        self.leases
            .get(&seat)
            .map(ApplicationRouteLease::routing_readiness)
    }

    /// Whether this lease may receive this event.
    ///
    /// Resolves the lease and asks `routing_readiness` itself, and takes no
    /// readiness argument: one produced earlier describes a lease that may
    /// since have been released, revoked, or had its control epoch advanced.
    ///
    /// A changed scene revision is not a refusal. What refuses is the target
    /// having changed or gone -- identity, eligibility, scope, output, device,
    /// and the generational barriers. The revision is recorded only after every
    /// check passes, so a failed validation cannot advance what the lease
    /// claims to have been seen against.
    pub fn authorize(
        &mut self,
        identity: ApplicationRouteLeaseIdentity,
        evidence: ApplicationRouteTargetEvidence,
    ) -> Result<ApplicationRouteLease, ApplicationRouteLeaseError> {
        let control_epoch = self.control_epoch;
        let lease = *self.exact_mut(identity)?;
        match lease.routing_readiness() {
            ApplicationRouteLeaseReadiness::ReadyForEvidenceValidation => {}
            readiness => return Err(ApplicationRouteLeaseError::NotRoutable(readiness)),
        }
        if lease.identity.control_epoch != control_epoch {
            return Err(ApplicationRouteLeaseError::StaleControlEpoch);
        }
        if lease.authority_session_epoch != evidence.authority_session_epoch {
            return Err(ApplicationRouteLeaseError::StaleAuthoritySession);
        }
        if lease
            .initiating_device
            .is_some_and(|expected| expected != evidence.device)
        {
            return Err(ApplicationRouteLeaseError::WrongDevice);
        }
        let ApplicationRouteLeaseBinding::Bound { output, .. } = lease.binding else {
            return Err(ApplicationRouteLeaseError::StalePresentation);
        };
        if output != evidence.output {
            return Err(ApplicationRouteLeaseError::StalePresentation);
        }
        // The evidence must be about THIS lease target. Without it, evidence
        // gathered for a surface that has since been replaced on the same seat
        // would validate its replacement.
        if lease.target_surface != evidence.target_surface {
            return Err(ApplicationRouteLeaseError::IdentityMismatch);
        }
        if lease.admission != evidence.target_admission {
            return Err(ApplicationRouteLeaseError::IdentityMismatch);
        }
        if !evidence.target_eligible {
            return Err(ApplicationRouteLeaseError::StalePresentation);
        }
        if !lease.scope.covers(evidence.resolved_scope) {
            return Err(ApplicationRouteLeaseError::OutsideScope);
        }
        // Recorded only now. Refreshing before the checks would let a failed
        // validation still advance what the lease claims to have been seen
        // against.
        let refreshed = self.exact_mut(identity)?;
        refreshed.binding = ApplicationRouteLeaseBinding::Bound {
            output,
            revision: evidence.presentation_revision,
        };
        Ok(*refreshed)
    }

    /// Pins the output an unbound lease will bind to, without binding it.
    ///
    /// The first held event names a candidate output before anything has been
    /// presented. Pinning early stops a later binding landing somewhere the
    /// grab never pointed at, and the pin survives promotion.
    pub fn pin_output(
        &mut self,
        identity: ApplicationRouteLeaseIdentity,
        output: OutputId,
    ) -> Result<ApplicationRouteLease, ApplicationRouteLeaseError> {
        if !output.is_valid() {
            return Err(ApplicationRouteLeaseError::InvalidCandidate);
        }
        let lease = self.exact_mut(identity)?;
        let ApplicationRouteLeaseBinding::AwaitingPresentation {
            pinned_output,
            deadline_msec,
        } = lease.binding
        else {
            return Err(ApplicationRouteLeaseError::InvalidPhase);
        };
        if pinned_output.is_some_and(|pinned| pinned != output) {
            return Err(ApplicationRouteLeaseError::StalePresentation);
        }
        lease.binding = ApplicationRouteLeaseBinding::AwaitingPresentation {
            pinned_output: Some(output),
            deadline_msec,
        };
        Ok(*lease)
    }

    /// Binds an unbound lease to the output its target was presented on.
    pub fn bind_presentation(
        &mut self,
        identity: ApplicationRouteLeaseIdentity,
        output: OutputId,
        revision: u64,
    ) -> Result<ApplicationRouteLease, ApplicationRouteLeaseError> {
        if !output.is_valid() || revision == 0 {
            return Err(ApplicationRouteLeaseError::InvalidCandidate);
        }
        let lease = self.exact_mut(identity)?;
        let ApplicationRouteLeaseBinding::AwaitingPresentation { pinned_output, .. } =
            lease.binding
        else {
            return Err(ApplicationRouteLeaseError::InvalidPhase);
        };
        // A pin inherited from the original grab decides where a promotion may
        // bind; it is not advice.
        if pinned_output.is_some_and(|pinned| pinned != output) {
            return Err(ApplicationRouteLeaseError::StalePresentation);
        }
        lease.binding = ApplicationRouteLeaseBinding::Bound { output, revision };
        Ok(*lease)
    }

    /// Reports an unbound lease that has passed its absolute deadline.
    ///
    /// Reports without removing. The frontend still holds an X grab, and
    /// dropping the lease here would let a shell capture take the seat before
    /// that grab is released; the caller runs the ordered release handshake and
    /// ownership stays with this lease until acknowledgement or quarantine. A
    /// lease already releasing is not reported again.
    pub fn observe_binding_deadline(
        &self,
        seat: SeatId,
        now_msec: u64,
    ) -> ApplicationRouteLeaseBindingTimeout {
        let Some(lease) = self.leases.get(&seat).copied() else {
            return ApplicationRouteLeaseBindingTimeout::NotAwaiting;
        };
        if matches!(lease.phase, ApplicationRouteLeasePhase::Releasing { .. }) {
            return ApplicationRouteLeaseBindingTimeout::NotAwaiting;
        }
        let ApplicationRouteLeaseBinding::AwaitingPresentation { deadline_msec, .. } =
            lease.binding
        else {
            return ApplicationRouteLeaseBindingTimeout::NotAwaiting;
        };
        if now_msec < deadline_msec {
            return ApplicationRouteLeaseBindingTimeout::Pending;
        }
        ApplicationRouteLeaseBindingTimeout::Expired(lease)
    }

    pub fn request_release(
        &mut self,
        seat: SeatId,
        now_msec: u64,
    ) -> Result<ApplicationRouteLease, ApplicationRouteLeaseError> {
        let lease = self
            .leases
            .get_mut(&seat)
            .ok_or(ApplicationRouteLeaseError::NoLease)?;
        if matches!(lease.phase, ApplicationRouteLeasePhase::Releasing { .. }) {
            return Err(ApplicationRouteLeaseError::InvalidPhase);
        }
        lease.phase = ApplicationRouteLeasePhase::Releasing {
            deadline_msec: now_msec.saturating_add(APPLICATION_ROUTE_RELEASE_TIMEOUT_MSEC),
        };
        Ok(*lease)
    }

    pub fn request_exact_release(
        &mut self,
        identity: ApplicationRouteLeaseIdentity,
        admission: ClientAdmissionId,
        now_msec: u64,
    ) -> Result<ApplicationRouteLease, ApplicationRouteLeaseError> {
        let lease = self.exact_mut(identity)?;
        if lease.admission != admission {
            return Err(ApplicationRouteLeaseError::IdentityMismatch);
        }
        // Engine withdrawal and the client's ungrab can race. Joining the
        // same release is idempotent and must not extend its deadline.
        if matches!(lease.phase, ApplicationRouteLeasePhase::Releasing { .. }) {
            return Ok(*lease);
        }
        lease.phase = ApplicationRouteLeasePhase::Releasing {
            deadline_msec: now_msec.saturating_add(APPLICATION_ROUTE_RELEASE_TIMEOUT_MSEC),
        };
        Ok(*lease)
    }

    pub fn acknowledge_release(
        &mut self,
        identity: ApplicationRouteLeaseIdentity,
        admission: ClientAdmissionId,
    ) -> Result<ApplicationRouteLease, ApplicationRouteLeaseError> {
        let lease = self
            .leases
            .get(&identity.seat)
            .copied()
            .ok_or(ApplicationRouteLeaseError::NoLease)?;
        if lease.identity != identity || lease.admission != admission {
            return Err(ApplicationRouteLeaseError::IdentityMismatch);
        }
        if !matches!(lease.phase, ApplicationRouteLeasePhase::Releasing { .. }) {
            return Err(ApplicationRouteLeaseError::InvalidPhase);
        }
        self.leases.remove(&identity.seat);
        Ok(lease)
    }

    pub fn frontend_release(
        &mut self,
        identity: ApplicationRouteLeaseIdentity,
        admission: ClientAdmissionId,
    ) -> Result<ApplicationRouteLease, ApplicationRouteLeaseError> {
        let lease = self
            .leases
            .get(&identity.seat)
            .copied()
            .ok_or(ApplicationRouteLeaseError::NoLease)?;
        if lease.identity != identity || lease.admission != admission {
            return Err(ApplicationRouteLeaseError::IdentityMismatch);
        }
        if identity.control_epoch != self.control_epoch {
            return Err(ApplicationRouteLeaseError::StaleControlEpoch);
        }
        // A physical button release ends automatic retention. Explicit grabs
        // require an ordered release first, including Engine-initiated scope exit.
        if lease.origin == ApplicationRouteLeaseOrigin::ExplicitPointer
            && !matches!(lease.phase, ApplicationRouteLeasePhase::Releasing { .. })
        {
            return Err(ApplicationRouteLeaseError::InvalidOrigin);
        }
        self.leases.remove(&identity.seat);
        Ok(lease)
    }

    pub fn observe_timeout(&mut self, seat: SeatId, now_msec: u64) -> ApplicationRouteLeaseTimeout {
        let Some(lease) = self.leases.get(&seat).copied() else {
            return ApplicationRouteLeaseTimeout::NotReleasing;
        };
        let ApplicationRouteLeasePhase::Releasing { deadline_msec } = lease.phase else {
            return ApplicationRouteLeaseTimeout::NotReleasing;
        };
        if now_msec < deadline_msec {
            return ApplicationRouteLeaseTimeout::Pending;
        }
        self.leases.remove(&seat);
        ApplicationRouteLeaseTimeout::Quarantine(lease)
    }

    pub fn revoke_admission(&mut self, admission: ClientAdmissionId) -> Vec<ApplicationRouteLease> {
        self.retain_collect(|lease| lease.admission != admission)
    }

    /// Drops every lease that depended on an output that is gone.
    ///
    /// Loss, not revision: an output that no longer exists cancels a lease
    /// bound to it whatever scene it was bound against. Ordinary revision
    /// changes must not call this -- a revision advances whenever anything on
    /// an output is added or removed, and cancelling on that is what let one
    /// popup end every grab on the screen. A revision change is answered by
    /// revalidating evidence in `authorize`.
    ///
    /// Unbound leases go too, including unpinned ones: a lease pinned here has
    /// lost the output it was promised, and an unpinned one was taken against a
    /// topology that no longer exists. Only a lease pinned to a surviving
    /// output is kept.
    pub fn lose_output(&mut self, output: OutputId) -> Vec<ApplicationRouteLease> {
        self.retain_collect(|lease| {
            lease
                .binding
                .pinned()
                .is_some_and(|pinned| pinned != output)
        })
    }

    pub fn security_transition(
        &mut self,
    ) -> Result<Vec<ApplicationRouteLease>, ApplicationRouteLeaseError> {
        self.control_epoch = self
            .control_epoch
            .checked_add(1)
            .ok_or(ApplicationRouteLeaseError::ControlEpochExhausted)?;
        Ok(std::mem::take(&mut self.leases).into_values().collect())
    }

    fn exact_mut(
        &mut self,
        identity: ApplicationRouteLeaseIdentity,
    ) -> Result<&mut ApplicationRouteLease, ApplicationRouteLeaseError> {
        if identity.control_epoch != self.control_epoch {
            return Err(ApplicationRouteLeaseError::StaleControlEpoch);
        }
        let lease = self
            .leases
            .get_mut(&identity.seat)
            .ok_or(ApplicationRouteLeaseError::NoLease)?;
        if lease.identity != identity {
            return Err(ApplicationRouteLeaseError::IdentityMismatch);
        }
        Ok(lease)
    }

    fn remove_exact(
        &mut self,
        identity: ApplicationRouteLeaseIdentity,
        phase: ApplicationRouteLeasePhase,
    ) -> Result<ApplicationRouteLease, ApplicationRouteLeaseError> {
        let lease = *self.exact_mut(identity)?;
        if lease.phase != phase {
            return Err(ApplicationRouteLeaseError::InvalidPhase);
        }
        self.leases.remove(&identity.seat);
        Ok(lease)
    }

    fn retain_collect(
        &mut self,
        mut keep: impl FnMut(ApplicationRouteLease) -> bool,
    ) -> Vec<ApplicationRouteLease> {
        let mut removed = Vec::new();
        self.leases.retain(|_, lease| {
            let retain = keep(*lease);
            if !retain {
                removed.push(*lease);
            }
            retain
        });
        removed
    }
}
