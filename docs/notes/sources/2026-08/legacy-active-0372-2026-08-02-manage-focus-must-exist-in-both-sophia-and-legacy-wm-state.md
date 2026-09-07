---
id: legacy-active-0372
date: 2026-08-02
recorded_date: 2026-08-02
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy"]
---
# 2026-08-02: manage focus must exist in both Sophia and legacy-WM state

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11379–11400. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- A live run from the identity-preserving bridge repeated the same visible
  demotion. Firefox committed focused in Sophia at transaction 6, then the
  recovery relayout at transaction 7 immediately returned Kitty as focused and
  master while placing Firefox in the lower-right slave pane. The retained
  synthetic ID therefore fixed a real lifecycle defect but was not the final
  stack-order cause.
- The xmonad profile always appended `FocusSurface(new)` to a manage response,
  but it did not ensure the private xmonad process had performed that focus
  transition. The previous real-xmonad regression hid this split state with a
  separate `FocusRequested` before releasing the constraint. Sophia and X
  Authority consequently focused Firefox while xmonad retained Kitty; the next
  relayout truthfully exposed xmonad's older focus/stack.
- Xmonad-profile manage now performs the same bounded synthetic pointer-focus
  synchronization used by an explicit focus request before returning the
  manage proposal. The real-xmonad smoke no longer contains the masking focus
  request and still requires the recovery surface to remain master/focused
  after hint release. A process-external fixture additionally requires a
  managed surface to remain the queried legacy-WM focus at the very next
  relayout.

<!-- END IMPORTED BODY -->
