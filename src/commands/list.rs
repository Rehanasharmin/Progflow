use crate::config::list_flows;
use crate::error::AppError;

pub fn run() -> Result<(), AppError> {
    let flows = list_flows()?;

    if flows.is_empty() {
        println!("no flows configured");
    } else {
        for flow in flows {
            println!("{}", flow);
        }
    }

    Ok(())
}
