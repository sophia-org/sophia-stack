---
id: readiness
date: 2026-09-06
kind: concept
status: established
tags: [concept, session, validation]
---
# Readiness must name an obligation

A component is ready for a particular operation, not ready in the abstract.
A desktop may accept an application launch before any application has produced
a frame. An application proof needs evidence about the surface it claims to
prove. Reusing one readiness flag for both obligations can create a cycle:
launching waits for a frame from an application that cannot yet launch.

Name the obligation at each boundary. Launch admission depends on authority and
available admission capacity. Native rendering activation depends on output
presentation and quarantine state. A requested application proof depends on its
specific surface evidence. The limits and deadlines for those operations remain
useful even when the desktop has no overall application-startup deadline.

This principle does not justify removing arbitrary timeouts or weakening
presentation evidence. It asks which obligation a check protects and which
component can satisfy it.

The [panel-only startup investigation](../investigations/startup-panel-only-startup-physical-acceptance.md)
provides a concrete failure. It supports the [desktop readiness ADR](../decisions/adr0001-separate-desktop-readiness-from-application-proofs.md);
the [architecture](../../architecture.md) defines the implemented boundaries.
