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
        .args(&["start", "-a", "android.intent.action.VIEW", "-d", url])
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
        "xdg-open", "gio open", "firefox", "chromium", "brave", "xdg-open",
    ];

    for opener in openers {
        if Command::new(opener).arg(url).spawn().is_ok() {
            return;
        }
    }

    eprintln!("Warning: Failed to open URL: {}", url);
    eprintln!("Hint: Install xdg-utils: sudo apt install xdg-utils");
}

pub fn get_editor() -> Option<String> {
    std::env::var("EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .ok()
}
