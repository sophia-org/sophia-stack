---
id: h0vxis10
date: 2026-09-06
kind: investigation
status: investigating
tags: [investigation, x11, rendering]
---
# Brave GPU watchdog repeats during live use

## Question

Why is Brave Origin slow in the current live session?

## Evidence

The user reported sluggish Brave Origin and later an intermittent fade confined
to the window border under CPU activity. Application content did not fade.
The user clarified that the border flashes orange to black and back.
The inspected session is
`00000001788748566248-d7be692d-f3cf-4f0d-9e70-9e6dfb3e7359`, running installed
`ef1ba0e7`. Its Sophia binary SHA-256 is
`a53655c27d7b6e7819dccabcbffc7a6ec5f31a4da314eed431c15e8267cfa64f`.
Brave is the user's `brave-origin-1.94.121`, launched with `--ozone-platform=x11`.

Read-only inspection found three GPU-process dumps in the existing
`~/.config/BraveSoftware/Brave-Origin/Crash Reports/completed` directory:

| Local time, 2026-09-06 | Dump | Process |
| --- | --- | --- |
| 22:58:25 EDT | `a6f7f1be-83e0-4c5b-a6d5-aae519d1b25b.dmp` | 12877 |
| 22:58:55 EDT | `de58f976-55af-4a02-9b92-ea836dcac72a.dmp` | 14642 |
| 22:59:25 EDT | `91494f3f-9c46-4f58-b8c0-eaabaf2f7909.dmp` | 16249 |

All three identify `--type=gpu-process`, crashing file
`gpu/ipc/service/gpu_watchdog_thread.cc`, GPU thread `.main`, and a hung main
thread. Their annotations say neither GPU initialization nor power-resume
handling was the trigger. The replacement GPU process, PID 18429, initially
used about one CPU core and had `--use-gl=disabled` when inspected later.
Only selected diagnostic fields were inspected; no browsing history, URLs,
credentials, or memory payloads were exported from the dumps.

Both Brave's main process and GPU process sent stdout/stderr to `/dev/null`.
No separate debug log was found. The session launcher inherits those streams,
so the missing browser error lines cannot be recovered from Sophia's structured
recorder. The current recorder remained running with zero discarded records
and storage errors. No retained protocol-error or timeout event matched this
launch; `timeout_msec` on resize configuration is not itself a timeout.

## Finding and limits

A repeating GPU watchdog hang is established. The disabled-GL replacement is
consistent with graphics fallback after the failures. Neither observation
identifies the GPU operation that blocked or proves that QueryPointer caused
it. No browser or session process was restarted, signalled, or reconfigured.

The border report is separate evidence. Engine's frame/focus-ring construction
uses opaque borders and focused/unfocused colours; the inspected path has no
CPU-load fade indicator. Recent retained chrome-set generations alternate, but
the recorder omits the frame and focus counts from those records. A generation
change alone cannot distinguish focus changes, changed geometry, or missing
chrome during composition. The border cause remains unconfirmed.

## Replacement-session recurrence

Installed `86ab21e4`, session
`00000001788751946481-31db6852-d07f-4f08-8ed9-87f63a561f59`, produced dump
`70caf92a-1dba-4215-9b18-57045b6dab1b.dmp` at 23:41:18 EDT on September 6.
Selective inspection identifies a GPU process and the same watchdog source
file. The user reports that all Brave interaction stops after a few clicks,
then recovers after switching windows.

At 23:46:55 EDT, browser PID 29334 and replacement GPU PID 28919 were both
sleeping in `poll_schedule_timeout.constprop.0`; the GPU watchdog thread was
waiting on a futex. No newer dump existed. These are waiting-state samples,
not stack traces or proof of a particular blocked X request.

Source inspection found that deferred `PresentNotifyMSC` requests advance only
through Present completions. A future timing request can therefore lack progress
when there are no further completions; reading the clock and queuing a request
also use separate locks. This is a liveness hypothesis, not an attribution of
the browser freeze. An invisible temporary probe window subscribed only to its
own Present events and asked for current MSC followed by MSC + 1. Two probes
returned in approximately 1 ms and 14 ms, including the one immediately after
the latest freeze report. The second advanced from 19566741 to 19566742.
The probe did not reproduce a stopped global presentation clock. It never
mapped a window, took focus, injected input, or changed another application's
subscription. Its observed script path is `/tmp/sophia-present-clock-probe.py`.

The [Present specification](https://sources.debian.org/src/xorgproto/2025.1-1/presentproto.txt)
defines timing notifications independently of submitting new pixmaps. Any
repair must use Engine/backend timing and retain frontend event ownership,
without fabricated frame completions or a WM timing policy.

[Interaction diagnostics](ce2b55uy-blank-thunar-menus-and-frozen-brave-need-separate-pixel-and-delivery-evidence.md)
now preserve delivery and grab counters that the old recorder discarded. This
is an evidence repair; neither the graphics hang nor the freeze is fixed.

The next session, installed from `e8573cf1`, retained routed clicks followed by
repeated explicit-grab rejections. The [click-lease investigation](744uylx4-explicit-pointer-grabs-must-replace-their-own-click-lease.md)
reproduces that ownership defect and tracks its repair separately. No new GPU
dump accompanied the sampled recurrence. Neither this correlation nor the
grab regression attributes the earlier watchdog dumps to pointer ownership.

## Next diagnostic step

On a deliberate browser relaunch, retain browser stderr in a private bounded
log and correlate the next watchdog failure with frontend request/presentation
progress. Do not disable the watchdog or advertise unsupported GPU capabilities
to hide the failure. For the border, correlate focus/chrome counts and exact
presentations before changing frame colours, animation policy, or damage.

## Connections

- [t003](../plans/queue-02-cp-14-3-development-session-readiness-and-milestone-14-c.md#t003)
  still requires usable Brave typing; Ghostty's accepted startup remains valid.
- [QueryPointer repair](knjco01f-pointer-queries-must-share-admitted-namespace-state.md)
  addresses a separately reproduced X input-query defect.
- [Runtime crash](fltuldiq-runtime-session-crash-retains-no-specific-cause.md)
  remains a separate incident; these dumps do not establish the same cause.
