# Investigation and concept map

[Notebook guide](../README.md) · [ADRs](decisions.md) · [Historical topics](topic.md)

## Desktop startup

The [panel-only startup investigation](../investigations/startup-panel-only-startup-physical-acceptance.md)
tracks the remaining physical check after the launch/readiness cycle was repaired.
It motivates the concept that [readiness must name an obligation](../concepts/readiness-readiness-must-name-an-obligation.md)
and the [decision to separate desktop readiness from application proofs](../decisions/adr0001-separate-desktop-readiness-from-application-proofs.md).

## Desktop composition

The [Session ownership decision](../decisions/adr0002-session-owns-desktop-composition.md)
explains where users choose the WM, shell, and startup applications. It links to
the operator guide and its original implementation evidence.

## Shell direction

The historical [descriptor/content-shell discussion](../sources/2026-09/legacy-active-0634-2026-09-06--descriptor-and-content-shells-have-distinct-trust-contracts.md)
explains the distinction between today's descriptor capabilities and proposed
shell-owned content. The [current content-shell proposal](../../content-shell.md)
owns the scope and remaining design gates.

Add connections here when they help someone find an investigation. Search the
whole collection with `zk list docs/notes --match "terms"`; this page is a curated
map, not a list of every note or another roadmap.
