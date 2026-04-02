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
    let cmd = if is_termux() {
        "termux-open-url"
    } else {
        "xdg-open"
    };

    if let Err(e) = Command::new(cmd).arg(url).spawn() {
        eprintln!("Warning: Failed to open {}: {}", url, e);
    }
}

pub fn get_editor() -> Option<String> {
    std::env::var("EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .ok()
}
