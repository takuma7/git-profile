use crate::{
    context::AppContext,
    git::{Level}
};
use anyhow::{anyhow, Result};

pub fn execute(context: &AppContext, profile_key: &str, maybe_level: &Option<Level>) -> Result<()> {
    let config = context.config_client.load()?;
    let git_config_client = context.git_config_client.as_ref();

    if let Some(profile) = config.profile.get(profile_key) {
        git_config_client.set(&profile, &maybe_level)?;
        println!("✨ Successfully applied {}", profile_key);
        Ok(())
    } else {
        Err(anyhow!("Profile {} doesn't exist", profile_key))
    }
}
