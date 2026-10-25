#![cfg(target_os = "linux")]

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

const ENTER_ALTERNATE_SCREEN: &str = "\u{1b}[?1049h";
const LEAVE_ALTERNATE_SCREEN: &str = "\u{1b}[?1049l";
const SHOW_CURSOR: &str = "\u{1b}[?25h";

fn scratch_config(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("weathrz-lifecycle-{name}-{}", std::process::id()));
    let config_dir = dir.join("weathrz");
    fs::create_dir_all(&config_dir).unwrap();
    fs::write(
        config_dir.join("config.toml"),
        "[location]\nauto = false\nlatitude = -6.2088\nlongitude = 106.8456\n",
    )
    .unwrap();
    dir
}

fn run_in_pty(name: &str, body: &str) -> Option<(Option<i32>, String)> {
    let has_script = Command::new("script")
        .arg("--version")
        .stdout(Stdio::null())
        .status()
        .is_ok_and(|status| status.success());
    if !has_script {
        eprintln!("skipping: util-linux `script` not available");
        return None;
    }

    let dir = scratch_config(name);
    let output = Command::new("script")
        .args(["-qfec", body, "/dev/null"])
        .env("SHELL", "/bin/sh")
        .env("XDG_CONFIG_HOME", &dir)
        .env("XDG_CACHE_HOME", &dir)
        .stdin(Stdio::null())
        .output()
        .unwrap();
    let _ = fs::remove_dir_all(&dir);
    Some((
        output.status.code(),
        String::from_utf8_lossy(&output.stdout).into_owned(),
    ))
}

fn run_in_pty_and_signal(signal: &str) -> Option<(Option<i32>, String)> {
    run_in_pty(
        signal,
        &format!(
            "stty cols 120 rows 30; \
             (sleep 1.5; kill -{signal} $$; sleep 3; kill -KILL $$) >/dev/null 2>&1 </dev/null & \
             exec {bin} --simulate clear",
            bin = env!("CARGO_BIN_EXE_weathrz"),
        ),
    )
}

fn tail(output: &str) -> &str {
    let mut start = output.len().saturating_sub(300);
    while !output.is_char_boundary(start) {
        start += 1;
    }
    &output[start..]
}

fn assert_terminal_restored(signal: &str) {
    let Some((status, output)) = run_in_pty_and_signal(signal) else {
        return;
    };
    let tail = tail(&output);
    assert_eq!(status, Some(0), "SIG{signal}: no clean exit: {tail:?}");
    let entered = output
        .rfind(ENTER_ALTERNATE_SCREEN)
        .unwrap_or_else(|| panic!("SIG{signal}: never entered the alternate screen: {tail:?}"));
    let left = output
        .rfind(LEAVE_ALTERNATE_SCREEN)
        .unwrap_or_else(|| panic!("SIG{signal}: never left the alternate screen: {tail:?}"));
    assert!(left > entered, "SIG{signal}: still in the alternate screen");
    assert!(
        output[left..].contains(SHOW_CURSOR),
        "SIG{signal}: cursor left hidden"
    );
}

#[test]
fn sigterm_restores_the_terminal() {
    assert_terminal_restored("TERM");
}

#[test]
fn sighup_restores_the_terminal() {
    assert_terminal_restored("HUP");
}

#[test]
fn sigint_restores_the_terminal() {
    assert_terminal_restored("INT");
}

#[test]
fn waits_for_a_pane_that_starts_at_zero_size() {
    let Some((status, output)) = run_in_pty(
        "zero-size",
        &format!(
            "stty cols 0 rows 0; \
             (sleep 0.5; stty cols 120 rows 30 </dev/tty; sleep 1.5; kill -TERM $$; sleep 3; kill -KILL $$) >/dev/null 2>&1 & \
             exec {bin} --simulate clear",
            bin = env!("CARGO_BIN_EXE_weathrz"),
        ),
    ) else {
        return;
    };
    let tail = tail(&output);
    assert!(
        output.contains(ENTER_ALTERNATE_SCREEN),
        "never started: {tail:?}"
    );
    assert_eq!(status, Some(0), "no clean exit: {tail:?}");
}
