use std::fs;
use std::process::Command;
use std::thread;
use std::time::Duration;

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
        .args(["new", name, "--dir", ".", "--editor", "echo 'hello'"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(&format!("flow '{}' created", name)));

    let config_path = dirs::config_dir()
        .unwrap()
        .join("flow")
        .join(format!("{}.json", name));
    assert!(config_path.exists());

    // Cleanup
    let _ = fs::remove_file(config_path);
}

#[test]
fn test_edit_set_note_quiet() {
    let name = "test_note_quiet";
    // Create flow
    let _ = Command::new("target/release/progflow")
        .args(["new", name, "--dir", ".", "--editor", "echo 'hello'"])
        .output()
        .expect("Failed to execute command");

    // Set note
    let output = Command::new("target/release/progflow")
        .args(["edit", name, "--set-note", "test note", "--quiet"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(output.stdout.is_empty());

    // Check note
    let output = Command::new("target/release/progflow")
        .args(["note", name])
        .output()
        .expect("Failed to execute command");

    let note = String::from_utf8_lossy(&output.stdout);
    assert!(note.contains("test note"));

    // Cleanup
    let config_path = dirs::config_dir()
        .unwrap()
        .join("flow")
        .join(format!("{}.json", name));
    let _ = fs::remove_file(config_path);
}

#[test]
fn test_start_commands_and_termination() {
    let name = "test_start_cmds";
    let _ = fs::remove_file(
        dirs::config_dir()
            .unwrap()
            .join("flow")
            .join(format!("{}.json", name)),
    );

    // Create flow with a background start command that sleeps
    let output = Command::new("target/release/progflow")
        .args([
            "new",
            name,
            "--cmd",
            "sleep 60",
            "--cmd-dir",
            ".",
            "--cmd-bg",
            "true",
        ])
        .output()
        .expect("Failed to execute command");
    assert!(output.status.success());

    // Start flow
    let output = Command::new("target/release/progflow")
        .args(["on", name])
        .output()
        .expect("Failed to execute command");
    assert!(output.status.success());

    // Check if sleep is running
    let output = Command::new("pgrep")
        .args(["-f", "sleep 60"])
        .output()
        .expect("Failed to execute pgrep");
    assert!(output.status.success());
    let pid = String::from_utf8_lossy(&output.stdout).trim().to_string();
    assert!(!pid.is_empty());

    // Stop flow
    let output = Command::new("target/release/progflow")
        .args(["off", name])
        .output()
        .expect("Failed to execute command");
    assert!(output.status.success());

    // Wait a bit for termination
    thread::sleep(Duration::from_secs(4));

    // Check if sleep is killed
    let output = Command::new("pgrep")
        .args(["-f", "sleep 60"])
        .output()
        .expect("Failed to execute pgrep");
    assert!(!output.status.success());

    // Cleanup
    let config_path = dirs::config_dir()
        .unwrap()
        .join("flow")
        .join(format!("{}.json", name));
    let _ = fs::remove_file(config_path);
}

#[test]
fn test_persistent_notes() {
    let name = "test_persist_note";
    let _ = fs::remove_file(
        dirs::config_dir()
            .unwrap()
            .join("flow")
            .join(format!("{}.json", name)),
    );

    // Create flow
    Command::new("target/release/progflow")
        .args(["new", name])
        .output()
        .expect("Failed to execute command");

    // Start flow
    Command::new("target/release/progflow")
        .args(["on", name])
        .output()
        .expect("Failed to execute command");

    // Stop flow with a note
    let output = Command::new("target/release/progflow")
        .args(["off", name, "--note", "persistent note content"])
        .output()
        .expect("Failed to execute command");
    assert!(output.status.success());

    // Start flow again and check if note is displayed
    let output = Command::new("target/release/progflow")
        .args(["on", name])
        .output()
        .expect("Failed to execute command");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Previous note:"));
    assert!(stdout.contains("persistent note content"));

    // Cleanup
    let config_path = dirs::config_dir()
        .unwrap()
        .join("flow")
        .join(format!("{}.json", name));
    let _ = fs::remove_file(config_path);
}

#[test]
fn test_url_readiness_check() {
    let name = "test_url_ready";
    let _ = fs::remove_file(
        dirs::config_dir()
            .unwrap()
            .join("flow")
            .join(format!("{}.json", name)),
    );

    // Create flow with unreachable localhost URL
    Command::new("target/release/progflow")
        .args(["new", name, "--urls", "http://localhost:12345"])
        .output()
        .expect("Failed to execute command");

    // Start flow and check for warning
    let output = Command::new("target/release/progflow")
        .args(["on", name])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Warning: localhost:12345 does not respond"));

    // Cleanup
    let config_path = dirs::config_dir()
        .unwrap()
        .join("flow")
        .join(format!("{}.json", name));
    let _ = fs::remove_file(config_path);
}
