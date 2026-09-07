---
id: ce2b55uy
date: 2026-09-06
kind: investigation
status: investigating
tags: [investigation, x11, rendering, validation]
---
# Blank Thunar menus and frozen Brave need separate pixel and delivery evidence

## Question

Which boundary fails when Thunar menus have blank, black, or missing portions,
and when Brave Origin stops responding until the user switches windows?

## Evidence

The installed session is `00000001788751946481-31db6852-d07f-4f08-8ed9-87f63a561f59`,
built from `86ab21e4879cc5b3154ca1192775de73e6e6a030`. The binary SHA-256 is
`0d972718c734751aba7fe58eb5075eb9f2f776bb4da7fdea3d69e80f98ac4d06`.
The applications are Brave Origin and Thunar; the user corrected an earlier
reference to Firefox. Thunar's symptom is missing pixels, not a reported menu
position error. Brave accepts a few clicks and then stops responding entirely;
the user reports recovery after switching away and back.

The retained input-lease and explicit-grab records contained only their schema.
Chrome records retained their generation but lost frame and focus counts.
`diagnostics::reduced_record` omitted these fields and most input status values.
Moreover, successful pointer delivery emitted only a first-use marker. Zero
recorder loss therefore did not establish that later clicks reached Brave.

The CPU presentation code in `software.rs::present_window_damage` chooses AR24
only for a bounding shape and otherwise tags the buffer XR24. It does not use
the window's depth-32 visual. The compatibility matrix already names that alpha
loss. It can explain black transparent areas, but no captured Thunar popup
establishes that this is the whole menu failure. Startup smoke tests require no
mapped window or pixel proof, so their success does not accept menu rendering.

## Diagnostic correction

The recorder now keeps record-scoped delivery, grab, frame, and timing counts,
and the fixed status vocabulary needed to interpret them. It still excludes
application identities, coordinates, button/key codes, and payloads. Pointer
button batches retain observed, routed, and suppressed counts after first use.
They use the existing bounded, asynchronous recorder; no disk work enters input
routing. This changes evidence collection, not input policy or presentation.

## Validation and remaining work

Two regression tests pass for diagnostic retention, vocabulary scoping, numeric
bounds, and payload exclusion. `cargo xtask check` passed, including workspace
tests, Clippy, layout checks, fixture verifiers, and the host buffer-age proof.
The first sandbox run stopped at a denied Unix-socket bind; the complete run
passed with local sockets available.

The observed evidence directory is `/tmp/sophia-interaction-evidence-197b2af77a09`.
It holds the source patch, full check log, timing-probe script, and identity.
The source-patch SHA-256 is `197b2af77a093f8cf1c14a23fff4f007e00ff4a9af9fce5ec8b3e5ada2cab217`.
This temporary path is not a durable physical-acceptance archive.
The code change is based on `86ab21e4`; it is not in the running session.

For t061, reproduce a mapped GTK menu with retained pixels and trace its actual
rendering requests. Compare frontend pixels and alpha format with Engine's
composed result before choosing the repair. Regress the failed boundary, pass
required checks, then accept visible menu text, background, edges, and submenus
in one installed Thunar use. Keep menu placement and drag acceptance under t060.
The new diagnostic counters support the existing Brave t003 investigation;
they do not establish its root cause or accept its usability.

## Connections

- [Brave watchdog investigation](h0vxis10-brave-gpu-watchdog-repeats-during-live-use.md)
  owns browser dumps, waiting-state samples, and frame-timing probes.
- [Pointer queries](knjco01f-pointer-queries-must-share-admitted-namespace-state.md)
  now return nonzero live state; pixel correctness remains separate.
- [Compatibility matrix](../../x11-compatibility-matrix.md) distinguishes startup,
  RENDER resource support, alpha limitations, and actual visual acceptance.
