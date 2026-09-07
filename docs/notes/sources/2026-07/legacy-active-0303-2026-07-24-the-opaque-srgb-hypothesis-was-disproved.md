---
id: legacy-active-0303
date: 2026-07-24
recorded_date: 2026-07-24
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-07-24: The Opaque sRGB Hypothesis Was Disproved

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9375–9478. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The next physical run proved mixed scanout and input were functional despite
the display still appearing black. Present transaction 202 reached mixed KMS
scanout, retired, became stable in 708 milliseconds, and accepted the user's
`exit` input. The failure was therefore the content of the imported client
layer rather than scheduling, focus, or VT ownership.

Adding an opaque sRGB framebuffer configuration did not change the client
selection: the next retained request stream still selected FBConfig 3, created
depth-32 windows, and exported ARGB8888 pixmaps. Both physical outputs remained
blank even though the mixed frame retired and keyboard input reached the
client. The speculative configuration and its compatibility claim are removed.

The remaining boundary is pixel content. Native mixed composition now has an
opt-in, one-shot readback that reports only aggregate counts and checksums after
the CPU background and after the DMA-BUF layer. The verifier distinguishes an
unchanged framebuffer, no visible RGB delta, and a visibly changed client
layer. ARGB composition also uses premultiplied source-over blending (`ONE`,
`ONE_MINUS_SRC_ALPHA`) instead of multiplying source RGB by alpha twice. These
changes remain application-agnostic and preserve Engine's protocol-neutral
authority boundary.

The first attempted diagnostic capture contained no pixel-evidence records, so
it could not classify the blank frame. Until the Kitty-only physical gate
passes, that profile now enables the bounded one-shot trace directly rather
than depending on an operator choosing a separate wrapper. An explicit
`status=enabled` record distinguishes missing activation from a failed GL
readback.

The resulting physical capture localized the blank screen further. The CPU
background and the framebuffer after the first Kitty DMA-BUF layer had the
same checksum and zero nonzero RGB pixels, while scanout retired the mixed
transaction normally. Kitty submitted that initial Present before mapping its
top-level window, then allocated a second DRI3 pixmap but submitted no second
Present. This is consistent with the client waiting for Present
Complete/Idle feedback before exposing its rendered window, not with a KMS,
cursor, keyboard-routing, or terminal-shell failure. The Kitty gate now traces
whether each backend completion was actually routed into the X frontend so the
next physical capture can distinguish feedback routing from client-side
consumption.

The next physical capture made that distinction: Complete and Idle were both
generated for the retired frame, but both reported `routed=false`. Standard
Present decode retained the X server frontend's globally allocated transaction
for its pending-feedback registry, while dispatch replaced the transaction
sent to Engine with the client's 16-bit request sequence. Retirement therefore
could not match the pending entry. Standard Present now carries the global
transaction explicitly through decode and dispatch, matching the existing
feedback registry key and avoiding cross-client sequence collisions. A wire
regression uses deliberately different request-sequence and global-transaction
values and requires the resulting surface transaction to retain the global
value.

The first capture after that change still reported `routed=false`, proving the
single-client run did not depend on the transaction distinction. The request
order exposed the actual feedback loss: Kitty selected Present events for a
bootstrap window, selected them again for its main window, then cleared the
bootstrap selection. The frontend stored only one selection per client, so
clearing the old event ID removed the newer main-window selection. Complete
marked the pending presentation finished but found no subscriber; Idle then
removed it without notifying Kitty. Kitty consequently never mapped its main
window or submitted the rendered follow-up pixmap.

Present selections are now retained per client and event ID, and a zero mask
removes only that selection. Complete and Idle route to every matching
window/mask selection. A regression preserves Kitty's observed bootstrap/main/
clear ordering and requires both events on the main event ID.

The same capture confirmed exact KD and termios restoration but returned to
greetd's VT because restarting the service activated its console. The guarded
TTY3 launcher now records its originating VT, restores the display manager,
reactivates that VT with `chvt`, and records the resulting active VT for both
normal and emergency cleanup.

The next physical run reached a visible Kitty command line, routed typed keys,
and continued retiring rendered Present frames with Complete/Idle feedback.
The session terminated only after pointer motion attempted a nonblocking atomic
cursor-plane update while another KMS commit was still in flight. Linux
returned `EBUSY`; the cursor path recognized only `EAGAIN`/`WouldBlock` as
transient and escalated `EBUSY` into a fatal session error. Nonblocking cursor
attach, move, and detach now classify both results as deferred and retry from
the existing dirty-cursor loop after scanout progresses. Other atomic errors
remain fatal.

The following run stayed healthy but did not echo `ll`. Its ordering identified
a separate startup race: Sophia declared focus ready and forwarded the physical
keys roughly one second before GLFW changed the mapped window's core event mask
to include KeyPress and KeyRelease. The X frontend previously fell back to the
focused window even when no window in its ancestor chain had selected keyboard
events, so the client ignored those early records. The input writer now keeps a
physical key boundedly pending for up to five seconds while the focused route
has no keyboard selection, then targets the selected window as soon as the
client installs its mask. This is based solely on standard X11 event selection;
it contains no Kitty-specific policy.

The next operator observation showed `ll` on tty3 only after emergency teardown.
That proves the VT input queue was still receiving the physical keyboard:
`stty raw -echo` disabled canonical processing and echo but did not disconnect
the Linux console keyboard from the VT. The guarded launcher now saves the
console keyboard mode with `KDGKBMODE`, selects `K_OFF` while Sophia owns the
graphical VT, and restores the exact saved mode during every cleanup path.
Evdev remains available to libinput and the independent emergency guard.

<!-- END IMPORTED BODY -->
