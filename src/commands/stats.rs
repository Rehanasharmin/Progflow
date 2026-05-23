use crate::config::load_config;
use crate::error::AppError;

pub fn run(name: &str) -> Result<(), AppError> {
    let config = load_config(name)?;

    println!("Analytics for flow: {}", name);
    println!("-------------------");

    let total_time = format_duration(config.total_seconds);
    println!("Total time spent:      {}", total_time);
    println!("Total sessions:        {}", config.session_count);

    if config.session_count > 0 {
        let avg_seconds = config.total_seconds / config.session_count;
        let avg_time = format_duration(avg_seconds);
        println!("Average session:       {}", avg_time);
    }

    if let Some(last) = config.last_activated {
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&last) {
            println!("Last activated:        {}", dt.format("%Y-%m-%d %H:%M"));
        }
    }

    Ok(())
}

fn format_duration(seconds: u64) -> String {
    if seconds == 0 {
        return "0s".to_string();
    }

    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    let mut parts = Vec::new();
    if hours > 0 {
        parts.push(format!("{}h", hours));
    }
    if minutes > 0 {
        parts.push(format!("{}m", minutes));
    }
    if secs > 0 || parts.is_empty() {
        parts.push(format!("{}s", secs));
    }

    parts.join(" ")
}
