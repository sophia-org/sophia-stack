---
id: legacy-active-0413
date: 2026-08-08
recorded_date: 2026-08-08
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-08: native pointer retention is output-local and epoch-fenced

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12556–12582. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Native application input now retains one interaction projection and semantic
  epoch per independently retiring output. Pointer placement selects the exact
  output projection; a page flip on one head cannot publish, clear, or advance
  another head's routing state. Buffer-only presentations preserve the epoch,
  while target identity, geometry, stacking, visibility, transform, output, or
  lifecycle changes invalidate it.
- An ordinary or passive-grab press creates a per-seat provisional lease bound
  to an exact lease ID, frontend sequence, control epoch, `SurfaceId`, admission,
  namespace profile, authority session, output, presentation epoch, device, and
  button. The X frontend confirms only after installing its protocol grab.
  Motion and release retain the original surface inside the admitted profile;
  scope exit discards the boundary event and waits for exact release
  acknowledgement rather than reinterpreting it as shell or foreign input.
- VT and external seat-release transitions advance the Engine/frontend control
  epoch. Frontend reduction clears active pointer, keyboard, and server grabs,
  drains frozen routes as rejected, and rejects any bounded-ingress event
  stamped before the transition. This barrier does not wait for cleanup; exact
  lease-release messages remain lifecycle acknowledgements.
- Focused regressions cover independently advancing output epochs, visual-only
  preservation, exact confirmation/release, and queued old-epoch rejection.
  TLC checks the corresponding provisional/active/releasing lifecycle and exact
  frontend sequence. Client-initiated explicit `GrabPointer` and XI requests,
  lock-authority integration, and shell capture remain open; this slice must not
  be described as universal grab arbitration yet.

<!-- END IMPORTED BODY -->
