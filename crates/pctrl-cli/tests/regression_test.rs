use std::process::Command;

fn pctrl() -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_pctrl"));
    cmd.current_dir(env!("CARGO_MANIFEST_DIR"));
    cmd
}

fn assert_convert(input: &str, expected: &str) {
    let output = pctrl()
        .args(["convert", input])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {}", stderr);
    assert_eq!(stdout.trim(), expected, "input: {}", input);
}

#[test]
fn test_history_terms() {
    assert_convert("\u{5355}\u{4e8e}\u{591c}\u{9041}\u{9003}", "chan2 yu2 ye4 dun4 tao2");
    assert_convert("\u{957f}\u{5b59}\u{65e0}\u{5fcc}", "zhang3 sun1 wu2 ji4");
    assert_convert("\u{5410}\u{8543}", "tu3 bo1");
    assert_convert("\u{9f9f}\u{5179}", "qiu1 ci2");
    assert_convert("\u{4ec6}\u{5c04}", "pu2 ye4");
    assert_convert("\u{8c25}\u{53f7}", "shi4 hao4");
    assert_convert("\u{5e99}\u{53f7}", "miao4 hao4");
    assert_convert("\u{89ca}\u{89ce}", "ji4 yu2");
    assert_convert("\u{9f83}\u{9f89}", "ju3 yu3");
    assert_convert("\u{7765}\u{7768}", "pi4 ni4");
    assert_convert("\u{684e}\u{688f}", "zhi4 gu4");
    assert_convert("\u{7ea8}\u{7ed4}", "wan2 ku4");
    assert_convert("\u{89d2}\u{8272}", "jue2 se4");
    assert_convert("\u{6c1b}\u{56f4}", "fen1 wei2");
}

#[test]
fn test_dict_validate() {
    let output = pctrl()
        .args(["dict", "validate", "../../dictionaries/history/history_core.json"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {}", stderr);
    assert!(stdout.contains("Dictionary is valid"), "got: {}", stdout);
}
