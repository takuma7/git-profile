use crate::{context::AppContext, git::Level};
use anyhow::{anyhow, Result};

pub fn execute(context: &AppContext, maybe_level: &Option<Level>) -> Result<()> {
    let config = context.config_client.load()?;
    let maybe_current_key = context.git_config_client.get_profile_key(&config, &maybe_level)?;

    if config.profile.is_empty() {
        return Err(anyhow!("No profile has been set up!"))
    }

    config.profile.keys().for_each(|key| {
        let is_current = maybe_current_key.as_ref() == Some(key);
        let indent = if is_current { "* " } else { "  " };
        println!("{indent}{key}")
    });

    Ok(())
}
