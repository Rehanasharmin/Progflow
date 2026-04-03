use crate::config::list_flows;
use crate::error::AppError;

pub fn run(json_output: bool) -> Result<(), AppError> {
    let flows = list_flows()?;

    if json_output {
        let json = serde_json::json!({
            "flows": flows,
            "count": flows.len()
        });
        println!("{}", serde_json::to_string_pretty(&json).unwrap());
    } else if flows.is_empty() {
        println!("no flows configured");
    } else {
        for flow in flows {
            println!("{}", flow);
        }
    }

    Ok(())
}
