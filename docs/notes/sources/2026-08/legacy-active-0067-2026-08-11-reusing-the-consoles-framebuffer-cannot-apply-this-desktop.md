---
id: legacy-active-0067
date: 2026-08-11
recorded_date: 2026-08-11
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-11: Reusing the console's framebuffer cannot apply this desktop

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 2025–2053. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The apply path was bounded to reuse the framebuffer each CRTC already scans out,
  on the reasoning that re-applying the topology already on screen would be the
  smallest real mutation available and therefore the right first one. The first
  authorized run refused, and the reasoning was wrong for a reason worth recording.
- `DP-2` is a 1920x1080 panel scanning out a **2560x1440** framebuffer. The console
  gives both CRTCs one buffer sized for the larger monitor, while Sophia's candidate
  asks each output for its own preferred mode. So "the topology already on screen"
  and "the configured topology" are not the same topology on this host, and the
  buffer that exists fits neither output's requested mode.
- The same constraint appears from the other direction. Two heads sharing one
  framebuffer are a mirror group, and a mirror group must be same-mode because no
  plane scaling exists anywhere; these two outputs disagree on size, so reusing the
  shared buffer for both heads would fail `MismatchedMirrorSize`. Two independent
  rules, one answer, which is a good sign the model is coherent.
- Nothing was submitted. The resolver failed closed before the first commit, which
  is what the bound was for: the failure mode it was chosen to prevent is a
  half-applied desktop, and a refusal at resolution time cannot produce one.
- The first refusal said only "has no framebuffer at the requested mode's size",
  which could not separate an output displaying nothing from one displaying the
  wrong size — the same defect as a rejection without an errno, made twice in one
  day. The cause now carries what the CRTC actually scans out. A diagnosis that
  requires a second tool is not a diagnosis.
- Consequence for the tranche: apply is complete as code and blocked as evidence,
  on output-scoped framebuffer allocation rather than on anything in the apply path.
  That is renderer work, and it now has a concrete reason to exist rather than a
  speculative one.

<!-- END IMPORTED BODY -->
