use std::process::Command;

fn pctrl() -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_pctrl"));
    cmd.current_dir(env!("CARGO_MANIFEST_DIR"));
    cmd
}

#[test]
fn test_convert_plain() {
    let output = pctrl()
        .args(["convert", "单于夜遁逃"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {}", stderr);
    assert!(stdout.trim().contains("chan2 yu2"), "got stdout: {}, stderr: {}", stdout, stderr);
}

#[test]
fn test_convert_json() {
    let output = pctrl()
        .args(["convert", "长孙无忌", "--format", "json"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {}", stderr);
    assert!(stdout.contains("zhang3"), "got stdout: {}, stderr: {}", stdout, stderr);
    assert!(
        stdout.contains("history_core") || stdout.contains("cc_cedict_common"),
        "got stdout: {}, stderr: {}",
        stdout,
        stderr
    );
}

#[test]
fn test_doctor() {
    let output = pctrl()
        .args(["doctor"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Configuration loaded successfully"));
}
