use std::fs;
use std::process::Command;

#[test]
fn test_new_flow_non_interactive() {
    let name = "test_non_interactive";
    let _ = fs::remove_file(
        dirs::config_dir()
            .unwrap()
            .join("flow")
            .join(format!("{}.json", name)),
    );

    let output = Command::new("target/release/progflow")
        .args(&["new", name, "--dir", "/tmp", "--quiet"])
        .stdin(std::process::Stdio::null())
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(output.stdout.is_empty());

    let config_dir = dirs::config_dir().unwrap().join("flow");
    let config_path = config_dir.join(format!("{}.json", name));
    assert!(config_path.exists());

    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("\"directory\": \"/tmp\""));

    // Cleanup
    let _ = fs::remove_file(config_path);
}

#[test]
fn test_edit_set_note_quiet() {
    let name = "test_note";
    let _ = fs::remove_file(
        dirs::config_dir()
            .unwrap()
            .join("flow")
            .join(format!("{}.json", name)),
    );

    // Create flow
    Command::new("target/release/progflow")
        .args(&["new", name, "--quiet"])
        .stdin(std::process::Stdio::null())
        .status()
        .unwrap();

    let output = Command::new("target/release/progflow")
        .args(&["edit", name, "--set-note", "test note", "--quiet"])
        .stdin(std::process::Stdio::null())
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(output.stdout.is_empty());

    let output = Command::new("target/release/progflow")
        .args(&["note", name])
        .output()
        .unwrap();

    let note = String::from_utf8_lossy(&output.stdout);
    assert!(note.contains("test note"));

    // Cleanup
    let config_path = dirs::config_dir()
        .unwrap()
        .join("flow")
        .join(format!("{}.json", name));
    let _ = fs::remove_file(config_path);
}
