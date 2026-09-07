---
id: 2mb23diq
date: 2026-09-06
kind: milestone
tags: [milestone, session, validation]
---
# Ghostty launcher startup accepted in ordinary use

The operator launched Ghostty through Super+Space and reported that it worked
well. The active session is
`00000001788748050781-210f9ce5-a3bc-4596-ad5b-b4e9b6367a22`, whose previously
inspected manifest identifies installed release
`8921174cc75a02bc6a18e87b4f5ecee293356510` and executable SHA-256
`2c6b947797be70985f0fb2d557acbaa0bd6da173537b6c07e4ae06b9b3b98fc5`.
Session listing after the report confirms it remains running with active
recording. The user's report is the evidence for the selected application and
visible success; session listing alone cannot establish either.

This accepts the Ghostty-launch portion of
[t003](../plans/queue-02-cp-14-3-development-session-readiness-and-milestone-14-c.md#t003)
and supplies one real-use observation for t002. It does not establish the Brave
typing check, all launcher interactions, or sustained Ghostty rendering
correctness. Task status remains in [todo.md](../../../todo.md). Preserve this
accepted result while the remaining workflows and X11 defects are investigated.
