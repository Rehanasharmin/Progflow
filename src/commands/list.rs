use crate::config::{is_flow_active, list_flows};
use crate::error::AppError;

pub fn run(json_output: bool, _verbose: bool, quiet: bool) -> Result<(), AppError> {
    let flows = list_flows()?;

    if quiet {
        return Ok(());
    }

    if json_output {
        let mut flows_info = Vec::new();
        for flow in &flows {
            let active = is_flow_active(flow).unwrap_or(false);
            flows_info.push(serde_json::json!({
                "name": flow,
                "active": active
            }));
        }
        let json = serde_json::json!({
            "flows": flows_info,
            "count": flows.len()
        });
        println!("{}", serde_json::to_string_pretty(&json).unwrap());
    } else if flows.is_empty() {
        println!("no flows configured");
    } else {
        for flow in flows {
            let active = if is_flow_active(&flow).unwrap_or(false) {
                " (active)"
            } else {
                ""
            };
            println!("{}{}", flow, active);
        }
    }

    Ok(())
}
