use anyhow::{Result, bail};

use crate::{context::AppContext};

pub fn execute(context: &AppContext, old_name: &str, new_name: &str) -> Result<()> {
    let config = context.config_client.load()?;

    if !config.has_profile_key(&old_name) {
        bail!("Profile key {} doesn't exist", &old_name);
    }

    if config.has_profile_key(&new_name) {
        bail!("Profile {} already exists", &new_name);
    }

    let mut config = config;
    config.rename_profile(&old_name, &new_name);
    let config = config;

    context.config_client.save(&config)?;

    println!("✨ Successfully renamed {} to {}", old_name, new_name);
    Ok(())
}
