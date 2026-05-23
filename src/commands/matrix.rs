use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub fn run() -> Result<(), AppError> {
    let mut stdout = io::stdout();
    let _ = write!(stdout, "\x1b[2J\x1b[H"); // Clear screen
    
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789$+-*/=%\"'#&_(),.;:?!\\|{}<>[]^~";
    let width = 80;
    let height = 24;
    let mut columns = vec![0; width];
    
    // Hidden surprise loop
    for _ in 0..100 {
        let mut line = String::with_capacity(width);
        for i in 0..width {
            if columns[i] > 0 {
                let idx = (rand_simple() % chars.len() as u32) as usize;
                line.push(chars.chars().nth(idx).unwrap());
                columns[i] -= 1;
            } else {
                if rand_simple() % 100 < 2 {
                    columns[i] = (rand_simple() % height as u32) as usize;
                    line.push(chars.chars().nth(0).unwrap());
                } else {
                    line.push(' ');
                }
            }
        }
        
        let _ = writeln!(stdout, "\x1b[32m{}\x1b[0m", line);
        let _ = stdout.flush();
        thread::sleep(Duration::from_millis(50));
    }
    
    println!("\nWake up, developer...");
    thread::sleep(Duration::from_secs(1));
    println!("Progflow has you now.");
    thread::sleep(Duration::from_secs(2));
    
    Ok(())
}

fn rand_simple() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros() as u32
}

use crate::error::AppError;
