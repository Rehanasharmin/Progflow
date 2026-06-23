use std::io::{self, Write};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::error::AppError;

pub fn run() -> Result<(), AppError> {
    let mut stdout = io::stdout();

    // Attempt to get terminal size via stty
    let (height, width) = get_terminal_size().unwrap_or((24, 80));

    // ANSI sequences: hide cursor, clear screen, home
    let _ = write!(stdout, "\x1b[?25l\x1b[2J\x1b[H");

    let chars =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789$+-*/=%\"'#&_(),.;:?!\\|{}<>[]^~あいうえおかきくけこ";
    let char_vec: Vec<char> = chars.chars().collect();
    let mut columns = vec![0; width];
    let mut speeds = vec![0; width];

    for i in 0..width {
        columns[i] = (rand_simple(i as u32) % height as u32) as usize;
        speeds[i] = ((rand_simple(i as u32 + 100) % 3) + 1) as usize;
    }

    // Animation loop
    for frame in 0..150 {
        // Move cursor to top-left for each frame instead of clearing to avoid flicker
        let _ = write!(stdout, "\x1b[H");

        for y in 0..height {
            let mut line = String::with_capacity(width * 15);
            #[allow(clippy::needless_range_loop)]
            for x in 0..width {
                let col_y = columns[x];
                if y == col_y {
                    // Brightest white for the "head"
                    line.push_str("\x1b[1;37m");
                    let idx =
                        (rand_simple(frame + x as u32 + y as u32) % char_vec.len() as u32) as usize;
                    line.push(char_vec[idx]);
                } else if y < col_y && (col_y - y) < 10 {
                    // Fading green trail
                    let intensity = 10 - (col_y - y);
                    if intensity > 7 {
                        line.push_str("\x1b[1;32m"); // Bold green
                    } else if intensity > 4 {
                        line.push_str("\x1b[0;32m"); // Normal green
                    } else {
                        line.push_str("\x1b[2;32m"); // Dim green
                    }
                    let idx =
                        (rand_simple(frame + x as u32 + y as u32) % char_vec.len() as u32) as usize;
                    line.push(char_vec[idx]);
                } else {
                    line.push(' ');
                }
            }
            let _ = writeln!(stdout, "{}", line);
        }

        // Update column positions
        for i in 0..width {
            if rand_simple(frame + i as u32) % 100 < 10 {
                columns[i] = (columns[i] + speeds[i]) % height;
            }
            #[allow(clippy::manual_is_multiple_of)]
            if rand_simple(frame + i as u32 + 500) % 500 == 0 {
                columns[i] = 0;
            }
        }

        let _ = stdout.flush();
        thread::sleep(Duration::from_millis(40));
    }

    // Show cursor, reset colors, clear screen
    let _ = write!(stdout, "\x1b[?25h\x1b[0m\x1b[2J\x1b[H");

    println!("\nSystem integrity verified.");
    thread::sleep(Duration::from_millis(500));
    println!("Progflow environment: OK");
    thread::sleep(Duration::from_millis(500));
    println!("Wake up, developer...");
    thread::sleep(Duration::from_secs(1));

    Ok(())
}

fn get_terminal_size() -> Option<(usize, usize)> {
    #[cfg(unix)]
    {
        use std::mem::MaybeUninit;
        let mut sz = MaybeUninit::<libc::winsize>::uninit();
        unsafe {
            if libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, sz.as_mut_ptr()) == 0 {
                let sz = sz.assume_init();
                return Some((sz.ws_row as usize, sz.ws_col as usize));
            }
        }
    }

    use std::process::Command;
    let output = Command::new("stty").arg("size").output().ok()?;
    if !output.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&output.stdout);
    let mut parts = s.split_whitespace();
    let h = parts.next()?.parse().ok()?;
    let w = parts.next()?.parse().ok()?;
    Some((h, w))
}

fn rand_simple(seed: u32) -> u32 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros() as u32;
    // Simple LCG
    (now.wrapping_mul(1103515245).wrapping_add(12345 + seed)) % 2147483647
}
