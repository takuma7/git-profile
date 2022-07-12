use anyhow::{Result, bail};

use crate::context::AppContext;

pub fn execute(context: &AppContext, profile_key: &str) -> Result<()> {
    let config = context.config_client.load()?;

    if !config.has_profile_key(&profile_key) {
        bail!("Profile {} doesn't exist", &profile_key);
    }

    let mut config = config;
    config.remove_profile(&profile_key);
    let config = config;

    context.config_client.save(&config)?;

    println!("✨ Successfully removed {}", profile_key);
    Ok(())
}
