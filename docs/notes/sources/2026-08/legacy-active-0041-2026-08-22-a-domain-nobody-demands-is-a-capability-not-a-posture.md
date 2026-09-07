---
id: legacy-active-0041
date: 2026-08-22
recorded_date: 2026-08-22
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "security"]
---
# 2026-08-22: a domain nobody demands is a capability, not a posture

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1317–1352. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Protection domains were complete and unused. `ProtectionDomainSpec` refused
  the forbidden compositions, the Bubblewrap launcher applied them, the PID
  handshake recovered the in-sandbox peer, and the executable probe confirmed
  the negative claims. Every one of those checks fired only for a caller that
  built a domain. A caller that built none reached role admission with a bare
  PID and was admitted, because `authorize_supervised_pid` took a number and a
  number carries no evidence of anything but identity.
- The question was where the invariant belongs. Construction was the wrong
  place: it can only constrain domains that exist. Admission is where the
  authority is actually granted, so admission is where the requirement now sits.
  A metadata-bearing role -- shell or metadata broker -- refuses a supervised PID
  and takes `ProtectionDomainEvidence` from the launch instead, which must carry
  that role's protection-domain role. Naming an expected PID at bind time is
  refused for the same reason; leaving that open would have made the constructor
  a second door into the rule the admission call rejects. The metadata broker
  transport publishes no PID-only call at all, so spawning it unprotected is a
  compile error rather than a quiet admission.
- Evidence stays a passive record with public fields, per the data-oriented
  rule that records do not grow hidden authority. That bounds the claim
  honestly: this is a declaration the supervisor makes, not a proof the socket
  verifies. It closes silent omission, where building no domain admitted anyway.
  It does not stop a caller from hand-writing evidence that contradicts the
  launch, which is a visible lie in the source rather than an absence.
- The blind spatial-policy and output roles still admit on a supervised PID.
  Requiring a domain for every role has to answer for hosts with no `bwrap`, and
  that decision is deliberately not taken here. The regression test asserting
  those two roles still admit without a domain is what keeps it from arriving as
  a side effect.
- One consequence accepted rather than worked around: the metadata broker
  transport smoke now always builds its domain, because an unprotected broker
  can no longer be admitted at all. `--protected` selects the in-sandbox
  negative probe, which is the part that costs host markers. The broker health
  smokes touch no role socket and still run without `bwrap`.

<!-- END IMPORTED BODY -->
