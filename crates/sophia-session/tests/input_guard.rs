#![cfg(feature = "native-session")]

#[test]
fn automatic_arming_cannot_publish_readiness_when_input_cannot_be_opened() {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let directory =
        std::env::temp_dir().join(format!("sophia-input-guard-{}-{nonce}", std::process::id()));
    std::fs::create_dir(&directory).unwrap();
    let armed = directory.join("armed");
    let triggered = directory.join("triggered");
    for mode in ["automatic", "manual"] {
        let result = sophia_session::run_input_guard(&[
            format!("--arming={mode}"),
            format!(
                "--input-devices={}",
                directory.join("missing-device").display()
            ),
            format!("--armed-file={}", armed.display()),
            format!("--triggered-file={}", triggered.display()),
            format!("--owner-pid={}", std::process::id()),
        ]);
        assert!(result.is_err(), "{mode} must fail without usable input");
        assert!(!armed.exists(), "{mode} must not claim recovery is ready");
        assert!(!triggered.exists());
    }
    std::fs::remove_dir(directory).unwrap();
}

#[test]
fn an_unknown_arming_mode_is_refused_before_opening_input() {
    let error = sophia_session::run_input_guard(&["--arming=disabled".to_owned()]).unwrap_err();
    assert_eq!(
        error.to_string(),
        "input guard --arming must be manual or automatic"
    );
}
