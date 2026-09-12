"""Private arboard markers. No clipboard contents, added requests or wakeups."""
ARCHIVE = "arboard-3.6.1"
CHECKSUM = "0348a1c054491f4bfe6ab86a7b6ab1e44e45d899005de92f58b3df180b36ddaf"

def instrument(root, replace, mark):
    # Path-patching a crate re-resolves its broad Windows range even on Linux.
    # Preserve the exact original pinentry lock choice rather than accepting drift.
    replace(root / ARCHIVE / "Cargo.toml", 'version = ">=0.52.0, <0.61.0"',
            'version = "=0.60.2"')
    path = root / ARCHIVE / "src/platform/linux/x11.rs"
    replace(path, "Ok(Self { conn, win_id })",
            mark("clipboard_window", "u64::from(win_id)") + "\nOk(Self { conn, win_id })")
    replace(path, "Event::DestroyNotify(_) => {",
            'Event::DestroyNotify(event) => {\n' + mark("clipboard_destroy_notify", "u64::from(event.window)"))
    # Scope the global-lock anchor to this destructor, not Clipboard::new.
    original = path.read_text()
    before, drop = original.split("impl Drop for Clipboard {", 1)
    def patch(old, new):
        nonlocal drop
        if drop.count(old) != 1:
            raise RuntimeError("arboard destructor anchor changed")
        drop = drop.replace(old, new)
    patch("fn drop(&mut self) {",
          "fn drop(&mut self) {\n" + mark("clipboard_drop_enter", "u64::from(self.inner.server.win_id)"))
    patch("let mut global_cb = CLIPBOARD.lock();",
          mark("clipboard_lock_enter") + "\nlet mut global_cb = CLIPBOARD.lock();\n" + mark("clipboard_lock_return"))
    for name, call in (
        ("manager", "self.inner.ask_clipboard_manager_to_request_our_data()"),
        ("destroy", "self.inner.server.conn.destroy_window(self.inner.server.win_id)"),
        ("flush", "self.inner.server.conn.flush()"),
        ("join", "server_handle.join()"),
    ):
        patch("if let Err(e) = " + call + " {",
              mark("clipboard_" + name + "_enter") + "\nlet result = " + call + ";\n"
              + mark("clipboard_" + name + "_return", "u64::from(result.is_ok())")
              + "\nif let Err(e) = result {")
    path.write_text(before + "impl Drop for Clipboard {" + drop)

CLIENT = """fn main() {
    let _trace = t082_trace::start();
    t082_trace::mark("clipboard_new_enter", 0);
    let clipboard = arboard::Clipboard::new().expect("private clipboard initialization failed");
    t082_trace::mark("clipboard_new_return", 0);
    drop(clipboard);
    t082_trace::mark("clipboard_drop_complete", 0);
}
"""
