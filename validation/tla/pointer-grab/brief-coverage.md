# t066 model brief, coverage, and correspondence

These are hand-maintained contract models for the approved t066 design. The
system is concurrent runtime/owner-loop coordination with message boundaries.
Publication, dequeue, application, reservation, response receipt, activation,
and presentation are distinct steps because another actor may run between
them. This uses the scenario and counterexample discipline from the
spec-generation and TLA-checking skills within Sophia's existing model layout.
It does not claim a generated trace-validation pipeline or Rust refinement.

## Scenarios and enabled checks

| Scenario | Model actions | Enabled check/configuration |
| --- | --- | --- |
| Map published but not yet accounted | PublishObservation, DequeueObservation, AccountObservation, Reserve | AppliedBeforeGrant in PointerGrabAdmission.cfg |
| A later no-Engine-work observation, with transaction gaps | Observations = <<2, 5>>, RequestPrepare | AccountedPrefix and AppliedBeforeGrant in the same configuration |
| Reply enables drawing before presentation | Reserve, ReceivePrepared, Activate, Present | DeliveryRequiresCurrentEvidence; Present requires activation, Reserve does not require presentation |
| Input arrives before activation/presentation | Hold, Activate, Bind, Deliver | DeliveryRequiresCurrentEvidence and UnboundLifetimeIsAbsolute |
| Cancellation races a late prepared response | ClearOwnership, ReceivePrepared | CancelledHasNoOwnership |
| Expiry, scope loss, owner/control invalidation, overflow | Tick, LeaveScope, Invalidate, Overflow | CancelledHasNoOwnership, UnboundLifetimeIsAbsolute, TypeOK |
| Same-scope scene change with original target retained | CommitChoice(retainPrior = TRUE), Present, RouteExistingLease | EligibleLeaseRemainsRoutable and ApplicationLeasesAreProfileScoped in InputAuthorityArbitration.cfg |
| Original target lost, forbidden scope, shell occlusion | CommitChoice, RequestLeaseRelease, RouteExistingLease | ApplicationLeasesAreProfileScoped, ShellAndApplicationCaptureAreExclusive |
| Explicit provisional versus automatic provisional | BeginFrontendGrab(origin), ConfirmFrontendGrab, RouteExistingLease | ApplicationLeasesAreProfileScoped |

Every invariant named above is enabled in the checked-in configuration. Both
negative configurations retain the positive configuration's full invariant
list and change one switch:

- `PointerGrabAdmissionWithoutPrerequisite.cfg` permits Reserve without its
  applied prefix. The trace publishes map 2 and observation 5, applies only map
  2, requests after 5, then incorrectly grants. `AppliedBeforeGrant` fails.
- `PointerGrabAdmissionReviveCancelled.cfg` restores ownership when a prepared
  response arrives after cancellation. `CancelledHasNoOwnership` fails.

A temporary arbitration mutation restores `held.presented = presentedScene`
only in RouteExistingLease. The unchanged availability invariant then fails
when a later same-scope scene preserves the original target. This checks that
the intended contract amendment is observable rather than an unused field. A second temporary mutation removes only the
original-target eligibility guard; `ApplicationLeasesAreProfileScoped` then
fails even when the currently selected application remains inside scope.

## Source correspondence

The listed owner files are the implementation seams being changed by t066.
Action boundaries describe the approved design; source line numbers are not
claimed as stable while the implementation is being integrated.

| Model boundary | Rust owner |
| --- | --- |
| PublishObservation / per-connection receipt | sophia-x-authority x11_socket/connection/server.rs observer, transport.rs ordered egress |
| DequeueObservation / AccountObservation | sophia-session live_session/owner_loop/authority.rs intake, observation and lifecycle cleanup |
| RequestPrepare / Reserve / ReceivePrepared | sophia-x-authority explicit_pointer_grab.rs bridge and sophia-session live_session/input/explicit_pointer_grab.rs owner service |
| Activate / readiness / release | sophia-engine input/route_lease.rs state reducer |
| Hold / Bind / Deliver / scope resolution | sophia-session live_session/input.rs and presented input projections |
| ClearOwnership / late-response refusal | bridge cancellation and session-owned reservation/input teardown |
| Presented target eligibility | sophia-backend-live production_visual_runtime/projection.rs and session layout owner visibility |

## Bounds and limits

The admission model has one attempt, two outputs, two published observations,
a two-tick Prepare interval, four-tick binding interval, and at most two held
events. It preserves ordering and deadline relationships, not milliseconds or
the production 32-request/256-event capacities. The second observation has no
Engine effects; the first applies mapping before the frontier advances.

All interleavings within those bounds are explored. An absolute clock tick
performs due cleanup; fairness of real owner-loop scheduling and wall-clock
latency are not proved. Present deliberately requires Activate to represent
the draw-after-success client. This tests a permitted difficult client, not
all possible client rendering orders.

The arbitration model's selected scene choice represents the result of a
correct scope resolver. Its eligible target set represents current target
identity independently of that choice. It does not calculate pixel geometry,
SHAPE regions, descriptor/chrome/tab precedence, transforms or coordinates;
Rust resolver tests and physical acceptance must establish that correspondence.
The model does not model immediate pre-flip per-surface unmap cleanup or prove
multi-connection barrier accounting, lock release,
request-capacity enforcement, promotion rollback or raw button/axis replay.
Those are explicit Rust/integration obligations in the approved plan.

## Observed checks

Pinned TLA+ Tools v1.7.4, SHA-256
`936a262061c914694dfd669a543be24573c45d5aa0ff20a8b96b23d01e050e88`,
one worker, fingerprint polynomial 0, 2026-09-07:

| Configuration | Result |
| --- | --- |
| InputAuthorityArbitration | Pass; 1,635,555 generated / 432,648 distinct states; depth 20 |
| PointerGrabAdmission | Pass; 184,589 generated / 44,213 distinct states; depth 25 |
| WithoutPrerequisite | Expected AppliedBeforeGrant failure; depth 7 |
| ReviveCancelled | Expected CancelledHasNoOwnership failure; depth 8 |
| Temporary exact-scene mutation | Expected EligibleLeaseRemainsRoutable failure |
| Temporary target-eligibility mutation | Expected ApplicationLeasesAreProfileScoped failure |

Temporary logs were captured under `/tmp/t066-model-check/`; their path is not
a retained-evidence promise. `tools/check_tla.sh` runs the positive models and
requires the two retained controls to fail for their named invariant. The
root investigation owns final candidate identity and durable evidence.
