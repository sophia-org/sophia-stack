# Private graphics probes

## Private GLX pixmap probe

`glx_pixmap.c` creates offscreen resources and compares synthetic pixels read
through a direct GL texture. It checks the initial CPU contents, a later partial
write and retention after `FreePixmap`, for depth-24 and depth-32 buffers with
2D and rectangle textures. Its non-power-of-two width exercises the texture
shape needed by ordinary video frames. It follows the required
synchronize/bind/read/release sequence. It opens no visible window and sends no
input.

The optional `--texture-1d` diagnostic exercises a separate Mesa client limit:
its direct GLX target decoder does not recognize 1D despite the driver's target
mask including it. The local Mesa 26.1.8 client reports target zero on that path.
The default pixel gate therefore covers 2D and rectangle sampling; it does not
claim 1D client support.

The integrated test starts a private X frontend with the same pixmap provider
as a native session. Select a render node explicitly:

```sh
SOPHIA_PIXMAP_TEST_DEVICE=/dev/dri/renderD128 \
  cargo test --offline -p sophia-session --all-features --lib \
  direct_glx_client_reads_live_and_retained_pixmap_exports \
  -- --ignored --nocapture --test-threads=1
```

The lower-level renderer tests verify repeated imports, partial updates,
revision replay and allocation lifetime through a private EGL/GL consumer:

```sh
SOPHIA_PIXMAP_TEST_DEVICE=/dev/dri/renderD128 \
  cargo test --offline -p sophia-renderer-live --all-features \
  --test shared_pixmap -- --ignored --nocapture --test-threads=1
```

Both commands require DRM render-node access. The GLX test also requires a C
compiler and Xlib/GL development files. They do not install a build, replace a
session or prove browser hardware-video playback. See the
[export contract](../../docs/pixmap-texture-exports.md) for those boundaries.

## GTK redraw

`gtk_redraw.c` exercises GTK3 drawing through Sophia's X frontend: a dialog,
a menu, dialog hide/show, and an explicit redraw. It uses synthetic text and
captures only its own windows. It needs a C compiler, `pkg-config`, and GTK3
and Xlib development files. Build Sophia with `native-session` and supply a WM
that implements `sophia_wm_v1`:

```sh
cargo build --offline -p sophia-cli --bin sophia --features native-session
python3 tools/run_gtk_redraw_probe.py --wm /path/to/wm
```

The runner creates a private directory under `/tmp` and prints its path. It
records binary identities, configuration, the session log, five PNG files, and
`result.json`. Use `--sophia` to select another candidate and `--output-parent`
to retain evidence elsewhere. It starts a bounded headless session with no
physical input. It does not alter the installed desktop or launch a shell.

A pass requires a successful client exit, clean session health and cleanup,
and background and text in both halves of each dialog and every menu row.
After the dialog hides, X11 focus must refer to a viewable window or to the
special None/PointerRoot targets. Retaining focus on its unmapped window fails
the probe even when its stored pixels are correct.
The headless composed scene must also report nonempty frames. This rejects the
admission regression where frontend `GetImage` returned complete windows but
the scene remained empty. The legacy `cpu_max_nonzero_pixel_bytes` field uses
exact counts only for initial proof frames, then bounded composition evidence;
the probe treats it solely as a nonempty-scene witness. It cannot distinguish
correct window content from surviving decoration. Exact compositor pixels are
checked separately by the admission/visibility regression.
The controlled white background and dark text make these checks independent
of the user's theme; they are content checks, not exact font-image comparisons.
The probe reads pixels only at five observation points. It installs no drawing
interposer. The socket regression `gtk_clip_copy_stream` separately compares
canonical pixels and published buffer updates for batched, fragmented, and
paced writes without intervening readbacks.

These checks establish frontend drawing, remap behavior, focus eligibility after
unmap, and nonempty headless composition. They do not prove
physical composition, replacement-focus policy, or pointer grabs. The synthetic menu has
no physical trigger event; GTK may report that warning. Installed-session
acceptance still requires opening Thunar menus and submenus, dismissing them,
and switching windows without missing pixels or lingering overlays.

## Qt popup probe

`qt_popup.cpp` opens an owned QMenu, selects an action, reopens it, opens and
selects a nested menu, then dismisses another opening with Escape. A separately labelled raw-X phase
uses the same XCB connection to create an owned popup, map it and immediately
grab without drawing first. Only GrabSuccess permits its marker drawing. This
checks the client pattern that cannot wait for presentation before receiving
the grab reply. Its events
are delivered locally to its own Qt widgets. It never injects physical input.
The executable forwards Qt's core and XI2 grab calls unchanged and records their
actual replies; it sends no extra X request between mapping and grabbing.
This observation is local to the probe process, with no desktop-wide interposer.

```sh
python3 tools/run_qt_popup_probe.py --sophia /path/to/sophia --wm /path/to/wm
python3 -m unittest discover -s tools/tests -p qt_popup_probe_test.py
```

Building needs a C++17 compiler, `pkg-config`, Qt6 Widgets, Xlib, XCB and XCB
XInput development files. The runner creates a private headless session with no
input devices, records executable/source hashes and configuration, and retains
four own-window PPM captures under the printed evidence directory. It does not
use the current display or restart the installed session.

A pass requires observed successful grabs on the actual first, reopened and
final menu windows, with mapping preceding each grab. Missing observations fail
the check. It also requires mapped menus with the expected transient-owner
chain, light backgrounds and dark text, successful selections and dismissal,
and successful raw grab followed by exact marker pixels. It also requires
a successful client exit, and clean session health and cleanup.

Own-window `GetImage` checks frontend pixels. The compositor's nonempty-scene
witness establishes only that composition occurred; neither establishes exact
composed menu pixels. The Engine scene regressions cover exact composition
separately. These local Qt events also do not prove physical click delivery or
recovery from a real application grab. Installed-session acceptance remains
necessary. Use `--authority-trace` to retain synthetic X request diagnostics; this changes
scheduling and is recorded in the identity file. The ordering race can depend
on scheduling: preserve repeated baseline runs and do not interpret one successful baseline as a disproof.
