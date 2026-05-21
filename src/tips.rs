use crate::platform::is_termux;
use std::env;

pub enum TipEvent {
    Create,
    On,
    Off,
}

pub fn show_tip(event: TipEvent) {
    let os = env::consts::OS;
    let tips = match event {
        TipEvent::Create => get_create_tips(os),
        TipEvent::On => get_on_tips(os),
        TipEvent::Off => get_off_tips(os),
    };

    if let Some(tip) = select_random_tip(tips) {
        println!("\n💡 Tip: {}", tip);
    }
}

fn select_random_tip(tips: Vec<&str>) -> Option<&str> {
    if tips.is_empty() {
        return None;
    }
    // Simple pseudo-random using current time micros
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros();
    let index = (now % tips.len() as u128) as usize;
    Some(tips[index])
}

fn get_create_tips(os: &str) -> Vec<&str> {
    let mut tips = vec![
        "You can update any part of this flow later with 'progflow edit <name>'.",
        "Add multiple start commands to launch your database, dev server, and more at once!",
        "Use the --urls flag to provide a comma-separated list of URLs to open automatically.",
        "Keep your tool fresh! Run 'progflow update' periodically to get the latest features.",
    ];

    match os {
        "linux" => {
            if is_termux() {
                tips.push("In Termux, you can use 'termux-boot' to run progflow on startup.");
            } else {
                tips.push("Create a .desktop file to launch your favorite flow from your application menu.");
            }
        }
        "macos" => {
            tips.push("Use Automator to create a 'Quick Action' for this flow and assign it a global hotkey.");
        }
        _ => {}
    }

    tips
}

fn get_on_tips(os: &str) -> Vec<&str> {
    let mut tips = vec![
        "Use 'progflow logs <name>' to see the output of your background start commands.",
        "Need to see what's running? 'progflow status' gives you a full breakdown.",
        "You can add a quick note to this session with 'progflow on <name> --note \"working on X\"'.",
        "New features are added regularly! Run 'progflow update' to stay current.",
    ];

    match os {
        "linux" => {
            if !is_termux() {
                tips.push("Linux Tip: Set a keyboard shortcut (like Ctrl+Alt+P) to quickly activate your main flow.");
            }
        }
        "macos" => {
            tips.push("macOS Tip: You can run progflow commands directly from Raycast or Alfred for faster switching.");
        }
        _ => {}
    }

    tips
}

fn get_off_tips(os: &str) -> Vec<&str> {
    let mut tips = vec![
        "Your last note is saved! It will be shown next time you run 'progflow on'.",
        "Use 'progflow off --force' if you're in a hurry and want to skip the note prompt.",
        "Progflow sends SIGTERM then SIGKILL to ensure all your processes are cleaned up properly.",
        "Want the latest bug fixes? Run 'progflow update' before your next session.",
    ];

    match os {
        "linux" => {
            if !is_termux() {
                tips.push("Linux Tip: Add 'progflow off' to your logout script to ensure all dev environments are closed.");
            }
        }
        "macos" => {
            tips.push("macOS Tip: Use a 'Quit App' trigger in Shortcuts to run 'progflow off' automatically.");
        }
        _ => {}
    }

    tips
}
