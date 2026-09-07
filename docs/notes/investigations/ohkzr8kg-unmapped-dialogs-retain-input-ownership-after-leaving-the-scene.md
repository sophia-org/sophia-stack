---
id: ohkzr8kg
date: 2026-09-07
kind: investigation
status: closed
tags: [investigation, input, session, x11]
---
# Unmapped dialogs retain input ownership after leaving the scene

## Driver and scope

After confirming visible Kitty launch and improved Thunar menu pixels on
`71b9b0d1`, the user reported general Thunar sluggishness followed by loss of
all keyboard and mouse input. VT switching remained available. This is t065,
a user-selected investigation alongside the remaining
[t061 pixel acceptance](ce2b55uy-blank-thunar-menus-and-frozen-brave-need-separate-pixel-and-delivery-evidence.md).
Do not turn the earlier visual confirmation into acceptance of responsiveness.

Exit: hiding a focused managed dialog or grabbed popup promptly revokes its
input standing without destroying its retained content. Pending acknowledgements
and retired frames cannot restore hidden targets. A remapped window can be
admitted again. The WM chooses replacement focus; the Engine enforces input
eligibility; the frontend owns X focus and grab semantics. Deterministic tests
and a real GTK baseline/candidate comparison precede installed confirmation of
Thunar interaction, ordinary application typing, and WM shortcuts.

## Live evidence

Session `00000001788792259184-92b32790-8c54-4552-a0c7-811d5af1c39c` uses
release `71b9b0d1960403ccbb8922ae1f1b91f3d34d60b9`, binary SHA-256
`11c76ed1889e891efac957fbe05227bc5287bafd4d61712db16cee012b4d92eb`.
The recorder reports no loss or storage errors. Read-only evidence was captured
at `/tmp/sophia-thunar-performance-71b9b0d1` and retained with the candidate
evidence described below.

A 15-second thread sample found Sophia's main thread at about 12% of one CPU,
mostly sleeping, with three samples in DRM flip completion. Thunar's main
thread waited in poll. The host reported no current CPU or I/O pressure.
These samples do not establish a GPU stall or a performance root cause.
One hundred read-only X11 focus queries on tty2 took a median 0.01077 ms and
maximum 0.06335 ms. The endpoint was responsive during that observation.

GetInputFocus named Thunar dialog `0x802910`, whose attributes reported
IsUnMapped (492 by 391). Main window `0x800006` was IsViewable. The log shows
Sophia processing the VT chord, releasing the seat, and later resuming it;
post-switch inactivity is expected suspension, not evidence of deadlock.

## Confirmed gaps and limits

Before this repair, Engine focus and pressed-key cleanup ran for destroyed surfaces, while
unmapping leaves the retained surface alive. Application route lease cleanup
also handles destruction without equivalent unmap cleanup. Scene and presented
input pruning correctly remove hidden surfaces; their other input state can
outlive them.

Managed unmap already emits Withdraw and reaches the WM as a surface-removal
request. An earlier claim that the WM was never notified was incorrect.
Likewise, missing focused geometry alone does not suppress pointer buttons.
Shortcuts run before focused application delivery, so stale focus is a real
defect but does not by itself explain the user's report of all input failing.
The installed pointer-batch counters omit chrome consumption and lease dispositions.
The candidate adds payload-free per-batch counts for keyboard routing, stale
focus, WM actions, chrome consumption, and lease waiting/rejection. These are
routing observations, not proof of frontend delivery or rendered response.

## Policy snapshot defect

The private GTK baseline also shows two surfaces in each WM snapshot after the
dialog hides; policy repeatedly chooses the unmapped dialog. The snapshot
builder combined current planning and authority facts with retained raster
layers. Withdrawal removed the policy facts, but the retained layer put the
same dialog back into the next snapshot. The renderer cache was granting
policy membership after that membership had ended.

The candidate builds snapshots from current facts and requires a mapped surface,
a pending plan, or active admission. Retained pixels and fresh passive updates
while hidden cannot request management. This preserves initial admission,
policy-hidden admitted windows, and remapping. A regression keeps a dialog's
layer resident through withdrawal, hidden updates and remap; removing the new
membership guard makes it fail on the hidden update.

## Reproduction

The real GTK redraw probe now checks that focus is viewable after hiding its
dialog. Installed `71b9b0d1` fails that check while all five pixel captures pass
and the scene remains nonempty. Evidence: `/tmp/t065-baseline-gtk.log`, probe
run `/tmp/sophia-gtk-redraw-mrk6k2lj`. This isolates the stale focus defect from
the earlier pixel failures. No live session was restarted or modified by the
probes.

## Repair and lifecycle validation

The session compares authority input eligibility across each lifecycle batch,
before layout service can apply pending focus. Eligibility follows a popup's
owner chain and includes described surfaces that do not yet have pixel layers.
Initial admission is not a loss of eligibility. Ordinary repaints skip the
sweep; unmap, removal and owner or role changes trigger it.

A surface that loses eligibility releases its application route leases, focus,
key repeat, pressed keys and targeted keyboard or pointer handoffs. Pending,
staged and retirement focus are also cleared. Retained content survives, and
the WM remains responsible for choosing replacement focus. Shared cleanup
keeps destruction and hiding consistent without treating a hidden window as
destroyed for proof accounting.

The first combined GTK run passed the focus and pixel checks but caught a
second race: destroying the dialog overtook its queued `ClearFocus`, and the
resulting `UnknownSurface` acknowledgement failed the session. The control
queue now retires that correlated reply for `ClearFocus`. It still rejects
unknown targets for commands that establish surface state. The queue regression
failed before this correction and passes afterward; all 22 control and
shutdown tests pass.

The final GTK run, `/tmp/sophia-gtk-redraw-z2y8wdcd`, passes all five captures,
the focus-after-unmap check, dialog remap and redraw, client exit and session
cleanup. It records 43 nonempty scene frames and a composition witness of 10;
the latter is bounded evidence, not an exact pixel-byte count. The log also
records the formerly fatal `ClearFocus` race as a retired stale target before
clean session health.

Owner visibility regressions cover nested popups, pending admission, owner
changes and staged focus. The policy snapshot regression preserves pixels
through withdrawal, hidden updates and remap. Mutation checks establish that
the new guards distinguish their respective failures. These helper tests do
not replace the GTK run through the actual owner loop.

## Candidate validation

Base: `71b9b0d1960403ccbb8922ae1f1b91f3d34d60b9`.
Source identity: `e4862810ae3a163676d92bd385bf42fa20cd5ec183754a8466862b0e61dbc3e9`.
Probe binary SHA-256: `ed2084cb7b52a5708f1f8bad0d69241bf82c958ef0e708b9832caaba3ec3c0f2`.
Evidence archive:
`~/.local/state/sophia/development-evidence/t065-input-e4862810ae3a`.
The source manifest includes the new, previously untracked snapshot test;
the archive retains source files, the patch, probe identities and logs.

The final `cargo xtask check` passes, including workspace tests, Clippy,
formatting, architecture/source checks and installed-session fixtures. The
canonical isolated configuration also passes the session tests that failed
under the developer's ambient configuration during the separate lane checks.
Task IDs and note links are valid.

The installed acceptance check covers ordinary
Thunar interaction, menu and dialog dismissal, application typing, and WM
shortcuts. The headless probe supplies no physical input and does not prove
pointer-grab recovery or perceived latency. These repairs close demonstrated
lifecycle defects; they do not yet explain every part of the reported all-input
lockup. Use the new routing counters if it recurs.

## Installed startup observation

The user returned to session
`00000001788812367995-fe320326-8322-45b2-b23e-7c1781c12d10` on September 7.
Its manifest identifies commit `2efff4cec91392d85f81d5c8a633bf7711eea3a4`
and installed binary SHA-256
`de8122fce511cd422c3d18bb0975fb3c90353f4fd0123ccec21a05befd7de0c0`.
The WM digest matches the GTK candidate's WM. Startup reached the session
phase, the input guard was armed, and the recorder reported no discarded
records or storage errors. The initial event log contained no runtime-fatal
or failed-status records. Recent key batches showed routed keys with no stale
focus suppression. No hidden-focus cleanup had yet been observed; this is
startup confirmation, not acceptance of the hide-and-resume repair.

## Installed interaction acceptance

The user completed the short Thunar menu and Properties-dialog sequence, then
explicitly confirmed that clicking, typing in Kitty and Super+Enter worked
after dismissal. The same session remained running with no runtime-fatal or
failed-status records, no recorder loss and no storage errors through sequence
2717. Routing totals at that observation included 107 routed keys, six WM
actions and 28 routed pointer events, with no stale-focus suppression or
lease waits. Two pointer-lease rejections were each followed by a lease release;
subsequent input continued. These counts are routing evidence, not individual
application-response assertions.

No `hidden_focus_cleared` record appeared in this short installed trace, so it
does not independently prove that particular cleanup branch ran. The GTK
baseline/candidate comparison supplies that evidence. Together with the
deterministic regressions and user confirmation, this satisfies t065's short
installed interaction gate. It does not establish a long-duration reliability
result or resolve the separate report of general Thunar sluggishness.

Acceptance evidence, including session and binary identity, is retained at
`~/.local/state/sophia/development-evidence/t065-installed-2efff4cec913`.
