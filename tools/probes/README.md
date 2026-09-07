# GTK redraw probe

`gtk_redraw.c` exercises GTK3 drawing through Sophia's X frontend: a dialog,
a menu, dialog hide/show, and an explicit redraw. It uses synthetic text and
captures only its own windows. It needs a C compiler, `pkg-config`, and GTK3
development files. Build Sophia with `native-session` and supply a WM that
implements `sophia_wm_v1`:

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

These checks establish frontend drawing, remap behavior, and nonempty headless
composition. They do not prove
physical composition, focus policy, or pointer grabs. The synthetic menu has
no physical trigger event; GTK may report that warning. Installed-session
acceptance still requires opening Thunar menus and submenus, dismissing them,
and switching windows without missing pixels or lingering overlays.
