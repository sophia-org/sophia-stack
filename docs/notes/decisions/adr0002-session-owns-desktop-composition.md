---
id: adr0002
date: 2026-09-06
kind: adr
status: accepted
tags: [adr, session, shell, policy]
---
# Session owns desktop composition

## Context

Users need one place to choose the WM, native shell, and applications that start
at login. Treating those choices as WM policy conflates component lifecycle with
layout and makes it difficult to replace the WM or shell independently.

## Decision

The operator's desktop profile selects components. Session owns WM and native
shell executable selection, private shell configuration selection, and login
applications. The WM owns its policy vocabulary; the shell owns its interface
choices. Engine retains visual and input authority.

Component choices and startup changes take effect at the next login. WM reload
does not replay the startup list. Selecting an executable does not grant it
additional capabilities; authority remains explicitly admitted at its boundary.
An empty startup list is a valid choice.

## Alternatives

Keeping all component selection in the WM couples desktop composition to one
policy implementation and its reload semantics. Hard-coding the reference shell
or panel into Engine turns user interface choices into compositor behavior.
General service supervision is broader than the present component-selection
contract and remains separate work.

## Consequences

Users configure each component where its policy belongs, while Session handles
composition and lifecycle. Other WMs and shells can use the same boundary.
Changing component selection requires a new login; reload is not a substitute
for authority activation. Existing confinement and launch rules still apply.

## Acceptance and connections

Recorded retrospectively on 2026-09-06 from the documented decision and approved
implementation in the [desktop-composition source note](../sources/2026-09/legacy-active-0001-2026-09-06-desktop-composition-belongs-to-the-session.md).
This record does not promote proposed content-shell capabilities or service
supervision to implemented behavior.

The [operator guide](../../desktop-composition.md) and [architecture](../../architecture.md)
own the current contract. The [readiness decision](adr0001-separate-desktop-readiness-from-application-proofs.md)
explains why selecting no startup application must still produce a usable desktop.
