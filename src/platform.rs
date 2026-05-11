use once_cell::sync::Lazy;
use std::process::Command;

static IS_TERMUX: Lazy<bool> = Lazy::new(is_termux_impl);

pub fn is_termux() -> bool {
    *IS_TERMUX
}

fn is_termux_impl() -> bool {
    if let Ok(prefix) = std::env::var("PREFIX") {
        if prefix.starts_with("/data/data/com.termux") {
            return true;
        }
    }

    if let Ok(output) = Command::new("which").arg("termux-open-url").output() {
        if output.status.success() {
            return true;
        }
    }

    false
}

pub fn spawn_url(url: &str) {
    if is_termux() {
        spawn_url_termux(url);
    } else {
        spawn_url_linux(url);
    }
}

fn spawn_url_termux(url: &str) {
    if let Ok(output) = Command::new("termux-open-url").arg(url).output() {
        if output.status.success() {
            return;
        }
    }

    if let Ok(output) = Command::new("am")
        .args(["start", "-a", "android.intent.action.VIEW", "-d", url])
        .output()
    {
        if output.status.success() {
            return;
        }
    }

    eprintln!("Warning: Failed to open URL: {}", url);
    eprintln!("Hint: Install a browser or open URLs manually");
}

fn spawn_url_linux(url: &str) {
    let openers = [
        vec!["xdg-open"],
        vec!["open"], // macOS
        vec!["gio", "open"],
        vec!["firefox"],
        vec!["chromium"],
        vec!["brave"],
    ];

    for args in openers {
        let cmd_name = args[0];
        if !command_exists(cmd_name) {
            continue;
        }

        let mut cmd = Command::new(cmd_name);
        if args.len() > 1 {
            cmd.args(&args[1..]);
        }
        cmd.arg(url);

        if cmd.spawn().is_ok() {
            return;
        }
    }

    eprintln!("Warning: Failed to open URL: {}", url);
}

fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn get_editor() -> Option<String> {
    std::env::var("EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .ok()
}
