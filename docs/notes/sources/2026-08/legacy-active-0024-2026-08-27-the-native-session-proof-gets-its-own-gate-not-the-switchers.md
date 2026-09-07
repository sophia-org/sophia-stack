---
id: legacy-active-0024
date: 2026-08-27
recorded_date: 2026-08-27
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "shell", "validation"]
---
# 2026-08-27: the native session proof gets its own gate, not the switcher's

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 824–858. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The bounded native workflow on the critical path -- three terminal launches, a
  visible focus-next, one close, and a normal logout -- now has a complete
  offline-proven harness: `tools/fixtures/hagia_native_session_guide.sh`,
  `tools/verify_hagia_native_session.sh`, `tools/hagia_native_session_gate.sh`,
  its `run_current_hagia_native_gate_tty4.sh` identity wrapper, an archiver and
  independent archive verifier over `hagia-native-runs/`, and
  `tools/check_hagia_native_matchers.sh`. The operator entry is
  `tools/hagia-native-proof`. No physical run has happened yet.
- The gate runs its session through the ordinary `hagia` runner profile rather
  than launching `sophia-live-session` itself. Exact TTY recovery is one of the
  row's exit criteria and only `run_sophia_xmonad_session.sh` produces
  `sophia_tty_recovery`; it also owns TTY mode save/restore, keyd, and the
  Ctrl-Alt-Backspace input guard. Routing keeps one session-lifecycle owner
  instead of standing a second one beside it and copying the record out.
- The proof phrase is typed first, while the startup terminal is the session's
  only window. Sophia matches the routed key events and the guide writes the
  received phrase to `SOPHIA_INPUT_PROOF_RESULT`; both halves need the
  keystrokes to land in that terminal, which is only guaranteed before any other
  window exists. The close may later land on the guide's own window, so the
  guide prints the close and logout steps together and the verifier reads its
  expected action totals from the guide source rather than from what the guide
  lived long enough to execute.
- The verifier adds one check nothing else in the tree performs:
  `frame_slots_leased == 0` at completion. A slot still leased after the session
  drained is a page flip that retired without releasing its buffer, which is the
  failure the three-slot ledger exists to make impossible. It keeps the mirror
  gate's balance assertion beside it -- every renderer-worker request settles as
  a completion or a bounded deferral -- and refuses a high-watermark above three.
- Each negative case in the matchers states the reason it expects to be rejected
  for. A mutation rejected for an unrelated reason proves nothing about the check
  it was written for, and a `sed` that silently stopped matching would otherwise
  still read as a passing negative case.

<!-- END IMPORTED BODY -->
