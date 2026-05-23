use crate::config::list_flows;
use crate::error::AppError;

pub fn run() -> Result<(), AppError> {
    let flows = list_flows()?;
    
    if flows.is_empty() {
        return Ok(());
    }

    println!("# Progflow Aliases");
    println!("# Add this to your .bashrc or .zshrc: eval \"\\$(progflow aliases)\"");
    println!();
    
    for flow in flows {
        // Replace non-alphanumeric characters with hyphens for the alias name
        let safe_name = flow.chars().map(|c| {
            if c.is_alphanumeric() { c } else { '-' }
        }).collect::<String>();
        
        println!("alias flow-{}='progflow on {}'", safe_name, flow);
    }
    
    Ok(())
}
