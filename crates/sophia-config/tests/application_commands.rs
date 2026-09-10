use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use sophia_config::*;

static NEXT: AtomicU64 = AtomicU64::new(0);

struct Directory(PathBuf);

impl Directory {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "sophia-profile-commands-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o700)).unwrap();
        Self(path)
    }

    fn write(&self, name: &str, source: &str) -> PathBuf {
        let path = self.0.join(name);
        fs::write(&path, source).unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();
        path
    }

    fn load(&self, source: &str) -> Result<PreparedDesktopProfile, DesktopProfileError> {
        let path = self.write("desktop.kdl", source);
        load_prepared_desktop_profile(Some(&path), ConfigGeneration::from_raw(7))
    }
}

impl Drop for Directory {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

#[test]
fn named_commands_core_references_and_inline_argv_share_one_prepared_identity() {
    let directory = Directory::new();
    let prepared = directory
        .load(
            r#"
schema 1
session {
  application "browser" { exec "brave-origin" "--new-window" "a literal argument"; }
  application "work" { use-core "advanced"; }
  browser "browser"
  terminal "work"
}
shortcut {
  profile "desktop"
  bind "Super+b" label="Browser" { launch "browser"; }
  bind "Super+e" { exec "/usr/bin/thunar" "$HOME; echo no" ""; }
  bind "Super+q" "session:close-window"
  bind "Super+j" "policy:focus-next"
}
policy { layout "tile"; }
"#,
        )
        .unwrap();
    let session = &prepared.candidates.session;
    assert_eq!(session.applications.len(), 3);
    assert_eq!(session.browser.as_deref(), Some("browser"));
    assert_eq!(session.terminal.as_deref(), Some("work"));
    assert_eq!(
        session.applications[0].command,
        DesktopApplicationCommand::Exec {
            executable: "brave-origin".into(),
            arguments: vec!["--new-window".into(), "a literal argument".into()],
        }
    );
    assert_eq!(
        session.applications[1].command,
        DesktopApplicationCommand::UseCore("advanced".into())
    );
    assert_eq!(
        session.applications[2].command,
        DesktopApplicationCommand::Exec {
            executable: "/usr/bin/thunar".into(),
            arguments: vec!["$HOME; echo no".into(), "".into()],
        }
    );
    assert!(session.applications[2].name.starts_with("__shortcut_"));
    let bindings = &prepared.candidates.shortcut.bindings;
    assert_eq!(
        bindings[0].target,
        DesktopShortcutTarget::LaunchApplication("browser".into())
    );
    assert_eq!(bindings[0].label.as_deref(), Some("Browser"));
    assert_eq!(
        bindings[1].target,
        DesktopShortcutTarget::LaunchApplication(session.applications[2].name.clone())
    );
    assert_eq!(
        bindings[2].target,
        DesktopShortcutTarget::Session(DesktopSessionShortcut::CloseFocused)
    );
    assert_eq!(
        bindings[3].target,
        DesktopShortcutTarget::PolicyAction("focus-next".into())
    );
    assert_eq!(session.generation, prepared.activation_key().generation());
    assert_eq!(session.digest, prepared.activation_key().digest());
    assert_eq!(prepared.candidates.shortcut.digest, session.digest);
    assert_eq!(
        prepare_desktop_profile_candidates(&prepared.profile).unwrap(),
        prepared.candidates
    );
}

#[test]
fn staging_exposes_commands_only_to_session_and_preserves_lowering_provenance() {
    let directory = Directory::new();
    let included = directory.write(
        "shortcuts.kdl",
        r#"
shortcut { profile "desktop"; bind "Super+e" { exec "file-browser" "private-argv-token"; }; }
"#,
    );
    let path = directory.write(
        "desktop.kdl",
        "schema 1\ninclude \"shortcuts.kdl\"\npolicy { layout \"tile\"; }\n",
    );
    let prepared = load_prepared_desktop_profile(Some(&path), ConfigGeneration::INITIAL).unwrap();
    let command = &prepared.candidates.session.applications[0];
    assert_eq!(command.provenance.path, included);
    assert_eq!(command.provenance.ordinal, 1);
    let lowered = &prepared.profile.candidates[&DesktopAuthority::Shortcut].values[1];
    assert_eq!(command.provenance, lowered.provenance);
    let fragments = stage_desktop_profile(&prepared.profile, &directory.0).unwrap();
    validate_desktop_profile_fragments(&fragments, prepared.activation_key()).unwrap();
    for authority in DesktopAuthority::ALL {
        let encoded = fs::read_to_string(fragments.path(authority)).unwrap();
        assert_eq!(
            encoded.contains("private-argv-token"),
            authority == DesktopAuthority::Session
        );
        assert_eq!(
            encoded.contains("file-browser"),
            authority == DesktopAuthority::Session
        );
    }
    let shortcut = load_desktop_authority_fragment(
        fragments.path(DesktopAuthority::Shortcut),
        DesktopAuthority::Shortcut,
        prepared.activation_key(),
    )
    .unwrap();
    assert_eq!(
        prepare_desktop_shortcut_candidate(&shortcut).unwrap(),
        prepared.candidates.shortcut
    );
    let session = load_desktop_authority_fragment(
        fragments.path(DesktopAuthority::Session),
        DesktopAuthority::Session,
        prepared.activation_key(),
    )
    .unwrap();
    let staged = prepare_desktop_session_candidate(&session).unwrap();
    assert_eq!(staged.applications.len(), 1);
    assert_eq!(staged.applications[0].command, command.command);
    assert_eq!(staged.applications[0].name, command.name);
}

#[test]
fn ambiguous_commands_names_and_cross_authority_references_are_refused() {
    let directory = Directory::new();
    for body in [
        r#"session { application "a" {}; }"#,
        r#"session { application "a" { exec "a"; use-core "b"; }; }"#,
        r#"session { application "a" { exec; }; }"#,
        r#"session { application "a" { exec ""; }; }"#,
        r#"session { application "a" { exec "./relative"; }; }"#,
        r#"session { application "a" { exec "dir/program"; }; }"#,
        r#"session { application "a" { exec executable="program"; }; }"#,
        r#"session { application "a" { exec "program" flag="value"; }; }"#,
        r#"session { application "a" { exec "program" { arg "x"; }; }; }"#,
        r#"session { application "a" { exec "program" (opaque)"arg"; }; }"#,
        r#"session { application "a" { use-core "core" "extra"; }; }"#,
        r#"session { application "a" { use-core "core" {}; }; }"#,
        r#"session { application "a" { exec "a"; }; application "a" { exec "b"; }; }"#,
        r#"session { application "__shortcut_1" { exec "a"; }; }"#,
        r#"shortcut { profile "desktop"; bind "Super+a" { exec "a"; }; bind "Super+b" { launch "__shortcut_1"; }; }"#,
        r#"shortcut { profile "desktop"; bind "Super+a" { exec "a"; }; bind "Super+b" "application:__shortcut_1"; }"#,
        r#"shortcut { profile "desktop"; bind "Super+a" { launch "missing"; }; }"#,
        r#"shortcut { profile "desktop"; bind "Super+a" { exec "a"; launch "b"; }; }"#,
        r#"shortcut { profile "desktop"; bind "Super+a" "session:logout" { exec "a"; }; }"#,
        r#"shortcut { profile "desktop"; pointer-bind "Super+left" { exec "a"; }; }"#,
        r#"shortcut { profile "desktop"; bind "Super+a" { exec "a"; }; bind "super+A" { exec "b"; }; }"#,
        r#"shortcut { bind "Super+a" { exec "a"; }; }"#,
    ] {
        assert!(
            directory.load(&format!("schema 1\n{body}\n")).is_err(),
            "{body}"
        );
    }
}

#[test]
fn source_rendering_restores_inline_commands_without_losing_literal_argv() {
    let directory = Directory::new();
    let original = directory
        .load(
            r#"
schema 1
session { application "named" { exec "named-program" "explicit"; }; }
shortcut {
  profile "desktop"
  bind "Super+a" label="A label" { exec "first-program" "quote\" slash\\" "" "$HOME; $(no-shell)"; }
  bind "Super+b" group="tools" { launch "named"; }
  bind "Super+c" { exec "/usr/bin/second-program" "a space"; }
}
policy { layout "tile"; }
"#,
        )
        .unwrap();
    let rendered = render_desktop_profile_source(&original.profile).unwrap();
    assert!(!rendered.contains("__shortcut_"));
    assert!(rendered.contains("first-program"));
    let restored = directory.load(&rendered).unwrap();
    assert_eq!(
        original.candidates.shortcut.bindings,
        restored.candidates.shortcut.bindings
    );
    let commands = |prepared: &PreparedDesktopProfile| {
        prepared
            .candidates
            .session
            .applications
            .iter()
            .map(|application| (application.name.clone(), application.command.clone()))
            .collect::<Vec<_>>()
    };
    assert_eq!(commands(&original), commands(&restored));
    assert_eq!(
        render_desktop_profile_source(&restored.profile).unwrap(),
        rendered
    );
}

#[test]
fn source_rendering_refuses_orphaned_generated_commands() {
    let directory = Directory::new();
    let mut prepared = directory
        .load(
            "schema 1\nshortcut { profile \"desktop\"; bind \"Super+a\" { exec \"program\"; }; }\n",
        )
        .unwrap();
    prepared
        .profile
        .candidates
        .get_mut(&DesktopAuthority::Shortcut)
        .unwrap()
        .values
        .truncate(1);
    assert!(prepare_desktop_profile_candidates(&prepared.profile).is_ok());
    assert!(render_desktop_profile_source(&prepared.profile).is_err());
}

#[test]
fn explicit_and_inline_commands_share_registry_and_argv_bounds() {
    let directory = Directory::new();
    let applications = |count| {
        (0..count)
            .map(|index| format!("application \"app{index}\" {{ exec \"program\"; }}\n"))
            .collect::<String>()
    };
    for (count, accepted) in [(31, true), (32, false)] {
        let source = format!(
            "schema 1\nsession {{ {} }}\nshortcut {{ profile \"desktop\"; bind \"Super+a\" {{ exec \"program\"; }}; }}\n",
            applications(count)
        );
        assert_eq!(
            directory.load(&source).is_ok(),
            accepted,
            "application count={count}"
        );
    }
    for (count, accepted) in [(32, true), (33, false)] {
        let arguments = " \"literal\"".repeat(count);
        let source = format!(
            "schema 1\nsession {{ application \"a\" {{ exec \"program\"{arguments}; }}; }}\n"
        );
        assert_eq!(
            directory.load(&source).is_ok(),
            accepted,
            "argument count={count}"
        );
    }
    for (bytes, accepted) in [(4096, true), (4097, false)] {
        let argument = "x".repeat(bytes);
        let source = format!(
            "schema 1\nsession {{ application \"a\" {{ exec \"program\" \"{argument}\"; }}; }}\n"
        );
        assert_eq!(
            directory.load(&source).is_ok(),
            accepted,
            "argument bytes={bytes}"
        );
    }
}

#[test]
fn literal_command_changes_change_the_shared_activation_digest() {
    let directory = Directory::new();
    let load = |argument| {
        directory.load(&format!("schema 1\nshortcut {{ profile \"desktop\"; bind \"Super+a\" {{ exec \"program\" \"{argument}\"; }}; }}\n")).unwrap()
    };
    let first = load("first");
    let second = load("second");
    assert_ne!(first.activation_key(), second.activation_key());
    assert_eq!(
        first.candidates.session.applications[0].name,
        second.candidates.session.applications[0].name
    );
    assert_eq!(
        second.candidates.shortcut.digest,
        second.candidates.session.digest
    );
    assert_eq!(second.candidates.session.digest, second.profile.digest);
}

#[test]
fn an_unlowered_shortcut_fragment_cannot_carry_literal_argv() {
    let mut profile = load_desktop_profile(None, ConfigGeneration::INITIAL).unwrap();
    let shortcut = profile
        .candidates
        .get_mut(&DesktopAuthority::Shortcut)
        .unwrap();
    shortcut.values.truncate(1);
    let provenance = shortcut.values[0].provenance.clone();
    shortcut.values.push(DesktopProfileValue {
        key: "shortcut.bind.Super+a".into(),
        encoded: "bind \"Super+a\" { exec \"program\" \"private\"; }".into(),
        provenance,
    });
    assert!(prepare_desktop_shortcut_candidate(shortcut).is_err());
    let prepared = prepare_desktop_profile_candidates(&profile).unwrap();
    assert_eq!(prepared.session.applications.len(), 1);
    assert!(matches!(
        prepared.shortcut.bindings[0].target,
        DesktopShortcutTarget::LaunchApplication(_)
    ));
    let directory = Directory::new();
    let fragments = stage_desktop_profile(&profile, &directory.0).unwrap();
    assert!(
        !fs::read_to_string(fragments.path(DesktopAuthority::Shortcut))
            .unwrap()
            .contains("private")
    );
    assert!(
        fs::read_to_string(fragments.path(DesktopAuthority::Session))
            .unwrap()
            .contains("private")
    );
}
