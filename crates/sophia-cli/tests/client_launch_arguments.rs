#[path = "../src/commands/client_launch/arguments.rs"]
mod arguments;

use arguments::{ArgvStyle, DeviceSwitch, Options};

fn strings(args: &[&str]) -> Vec<String> {
    args.iter().map(|arg| (*arg).into()).collect()
}
fn parse(style: &str, tail: &[&str]) -> Options {
    let mut args = strings(&["--adapter=chromium", style, "--", "browser"]);
    args.extend(strings(tail));
    Options::parse(&args).unwrap()
}
const DEVICE: &str = "/dev/dri/renderD132";

#[test]
fn launch_options_stop_at_the_program_boundary() {
    let options = Options::parse(&strings(&[
        "--check-only",
        "--argv-style=direct",
        "--adapter=chromium",
        "--",
        "a browser",
        "--check-only",
        "--adapter=other",
        "",
        "https://example.test/a b",
    ]))
    .unwrap();
    assert_eq!(options.program, "a browser");
    assert_eq!(options.style, ArgvStyle::Direct);
    assert!(options.check_only);
    assert_eq!(
        options.arguments,
        strings(&[
            "--check-only",
            "--adapter=other",
            "",
            "https://example.test/a b"
        ])
    );
}

#[test]
fn incomplete_unknown_and_duplicate_launch_options_are_refused() {
    for args in [
        vec![],
        vec!["--adapter=chromium", "--argv-style=direct", "browser"],
        vec!["--argv-style=direct", "--", "browser"],
        vec!["--adapter=chromium", "--", "browser"],
        vec!["--adapter=chromium", "--argv-style=direct", "--"],
        vec!["--adapter=chromium", "--argv-style=direct", "--", ""],
        vec![
            "--adapter=chromium",
            "--argv-style=direct",
            "--unknown",
            "--",
            "browser",
        ],
        vec![
            "--adapter=chromium",
            "--adapter=chromium",
            "--argv-style=direct",
            "--",
            "browser",
        ],
        vec![
            "--adapter=chromium",
            "--argv-style=direct",
            "--argv-style=wrapper",
            "--",
            "browser",
            "--",
        ],
        vec![
            "--adapter=chromium",
            "--argv-style=direct",
            "--check-only",
            "--check-only",
            "--",
            "browser",
        ],
        vec!["--adapter=other", "--argv-style=direct", "--", "browser"],
    ] {
        assert!(
            Options::parse(&strings(&args)).is_err(),
            "accepted {args:?}"
        );
    }
}

#[test]
fn direct_insertion_preserves_arguments_and_browser_terminator() {
    for tail in [
        vec![],
        vec!["https://example.test"],
        vec!["--", "--render-node-override=/literal", "a b"],
    ] {
        let options = parse("--argv-style=direct", &tail);
        let adapted = options.adapt(DEVICE).unwrap();
        let mut expected = strings(&tail);
        let position = tail
            .iter()
            .position(|arg| *arg == "--")
            .unwrap_or(tail.len());
        expected.insert(position, format!("--render-node-override={DEVICE}"));
        assert_eq!(adapted.arguments, expected);
        assert!(adapted.explicit_devices.is_empty());
        assert_eq!(options.arguments, strings(&tail));
    }
}

#[test]
fn wrapper_flags_are_never_read_as_browser_device_selection() {
    let tail = [
        "--render-node-override=/wrapper-only",
        "--profile",
        "a b",
        "--",
        "--flag",
        "--",
        "--hardware-video-device-path=/literal",
    ];
    let options = parse("--argv-style=wrapper", &tail);
    assert_eq!(options.style, ArgvStyle::Wrapper);
    let adapted = options.adapt(DEVICE).unwrap();
    let mut expected = strings(&tail);
    expected.insert(5, format!("--render-node-override={DEVICE}"));
    assert_eq!(adapted.arguments, expected);
    assert!(adapted.explicit_devices.is_empty());
    assert!(
        Options::parse(&strings(&[
            "--adapter=chromium",
            "--argv-style=wrapper",
            "--",
            "go",
            "--flag"
        ]))
        .is_err()
    );
    assert_eq!(
        parse("--argv-style=wrapper", &["--"])
            .adapt(DEVICE)
            .unwrap()
            .arguments,
        strings(&["--", &format!("--render-node-override={DEVICE}")])
    );
}

#[test]
fn explicit_device_paths_are_preserved_for_physical_identity_validation() {
    for style in ["--argv-style=direct", "--argv-style=wrapper"] {
        for selected in [
            vec!["--render-node-override=/dev/dri/by-path/render"],
            vec!["--hardware-video-device-path=/dev/dri/renderD128"],
            vec![
                "--render-node-override=/dev/dri/renderD128",
                "--hardware-video-device-path=/dev/dri/renderD129",
            ],
            vec![
                "--hardware-video-device-path=/dev/dri/renderD129",
                "--render-node-override=/dev/dri/renderD128",
            ],
        ] {
            let mut tail = if style.ends_with("wrapper") {
                vec!["--wrapper-flag", "--"]
            } else {
                vec![]
            };
            tail.extend(selected.clone());
            tail.extend(["--", "--render-node-override=/literal"]);
            let adapted = parse(style, &tail).adapt(DEVICE).unwrap();
            assert_eq!(adapted.arguments, strings(&tail));
            assert_eq!(adapted.explicit_devices.len(), selected.len());
            for (observed, raw) in adapted.explicit_devices.iter().zip(&selected) {
                let (name, path) = raw.split_once('=').unwrap();
                assert_eq!(observed.path, path);
                assert_eq!(
                    observed.kind,
                    if name == "--hardware-video-device-path" {
                        DeviceSwitch::HardwareVideoDevicePath
                    } else {
                        DeviceSwitch::RenderNodeOverride
                    }
                );
            }
        }
    }
}

#[test]
fn ambiguous_device_switches_are_refused_before_launch() {
    for tail in [
        vec!["--render-node-override"],
        vec!["--hardware-video-device-path", "/dev/dri/renderD128"],
        vec!["--render-node-override="],
        vec!["--hardware-video-device-path="],
        vec!["-render-node-override=/dev/dri/renderD128"],
        vec!["-hardware-video-device-path"],
        vec!["--render-node-override=/a", "--render-node-override=/b"],
        vec![
            "--hardware-video-device-path=/a",
            "--hardware-video-device-path=/a",
        ],
    ] {
        assert!(
            parse("--argv-style=direct", &tail).adapt(DEVICE).is_err(),
            "accepted {tail:?}"
        );
    }
}

#[test]
fn switch_name_prefixes_and_metacharacters_remain_literal_arguments() {
    let tail = [
        "--render-node-override-extra=/other",
        "--hardware-video-device-pathology=x",
        "$(touch /tmp/nope)",
        "`echo hi`",
        "x;y",
        "a=b",
    ];
    let adapted = parse("--argv-style=direct", &tail).adapt(DEVICE).unwrap();
    assert_eq!(&adapted.arguments[..tail.len()], strings(&tail));
    assert_eq!(
        adapted.arguments.last().unwrap(),
        &format!("--render-node-override={DEVICE}")
    );
    assert!(adapted.explicit_devices.is_empty());
}

#[test]
fn invalid_exec_strings_and_invalid_selected_device_are_refused() {
    for program in ["\0", "bad\0program"] {
        assert!(
            Options::parse(&strings(&[
                "--adapter=chromium",
                "--argv-style=direct",
                "--",
                program
            ]))
            .is_err()
        );
    }
    assert!(
        Options::parse(&strings(&[
            "--adapter=chromium",
            "--argv-style=direct",
            "--",
            "browser",
            "bad\0arg"
        ]))
        .is_err()
    );
    let options = parse("--argv-style=direct", &[]);
    for device in ["", "renderD128", "/dev/dri/render\0D128"] {
        assert!(options.adapt(device).is_err());
    }
}

#[test]
fn invalid_helper_options_do_not_echo_private_payloads() {
    let secret = "https://example.test/reset?token=private-token";
    let error = Options::parse(&strings(&[
        "--adapter=chromium",
        "--argv-style=direct",
        secret,
        "--",
        "browser",
    ]))
    .unwrap_err();
    assert_eq!(error, "unknown client-launch option");
    assert!(!error.contains(secret));
    assert!(!error.contains("private-token"));
}
