---
id: adr0001
date: 2026-09-06
kind: adr
status: accepted
tags: [adr, session, validation]
---
# Separate desktop readiness from application proofs

## Context

The ordinary launcher inherited an eight-second application proof deadline from
early development. In a panel-only session, the launch queue waited for a focused
application frame, while producing that frame required admitting the launch.
Super+Enter committed its intent but could not start the process; the deadline
ended the desktop.

## Decision

Ordinary desktops have no overall application-startup proof requirement or
replacement desktop timer. Empty and panel-only startup are valid. Authorized
launches depend on admission-pipeline readiness. Native activation depends on
completed output presentation and cleared quarantine; it does not require a
focused application in ordinary operation.

Explicit proof sessions retain the requested deadline and exact-surface evidence.
Authority activation, application admission, WM replies, input delivery, page
flips, and shutdown retain their separate checks and deadlines. Normal completion
records the proof as not requested rather than inventing a successful timestamp.

## Alternatives

Keeping or extending the global deadline would preserve the circular dependency
and reject valid desktops without applications. Removing only the launcher flag
would leave completion and native activation coupled to the proof. Treating an
empty desktop as a successful application proof would falsify the evidence.

## Consequences

Normal lifecycle and development proofs have separate completion semantics and
verifiers. Tests must cover empty startup, background applications, contained
spawn failures, and later user launches, while explicit proofs still fail when
their obligations are unmet. Physical acceptance must use a matching installed
candidate; deterministic checks alone do not establish it.

## Acceptance and connections

Recorded retrospectively on 2026-09-06 from the user's confirmation that this
deadline was development scaffolding and approval to implement its retirement.
The [original investigation and implementation record](../sources/2026-09/legacy-active-0638-2026-09-06--retire-application-startup-proofs-from-normal-desktop-lifetime.md)
preserves that basis. This ADR records the existing decision rather than granting
new approval or declaring physical acceptance complete.

The [architecture](../../architecture.md) owns the current lifecycle contract;
the [operator guide](../../desktop-composition.md) explains empty startup.
[Readiness must name an obligation](../concepts/readiness-readiness-must-name-an-obligation.md)
extracts the reusable reasoning. The [maintained investigation](../investigations/startup-panel-only-startup-physical-acceptance.md)
links the remaining physical check to the roadmap.
