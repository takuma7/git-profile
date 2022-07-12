use crate::{
    context::AppContext,
    git::{Level},
    validation::{self},
};
use anyhow::{bail, Result};
use dialoguer::{theme::ColorfulTheme, Input};

pub fn execute(context: &AppContext, maybe_level: &Option<Level>) -> Result<()> {
    let config = context.config_client.load()?;
    if let Some(profile_key) = context
        .git_config_client
        .get_profile_key(&config, &maybe_level)?
    {
        // Already imported
        bail!("Already imported as {}", profile_key);
    }

    let maybe_profile = context.git_config_client.get(&maybe_level)?;

    if maybe_profile.is_none() {
        bail!("Can't import anything as git config values were not found.");
    }

    let profile = maybe_profile.unwrap();

    println!("Found a user config as follows:");
    println!("user.name={}", &profile.name);
    println!("user.email={}", &profile.email);
    if let Some(signingkey) = &profile.signingkey {
        println!("user.signingkey={}", signingkey);
    }

    let profile_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter profile name")
        .validate_with(|input: &String| -> Result<()> {
            validation::profile_key::no_duplicates(input, &config.profile)?;
            Ok(())
        })
        .interact_text()?;

    let mut config = config;
    config.upsert_profile(&profile_name, profile);
    let config = config;

    context.config_client.save(&config)?;

    println!(
        "✨ Successfully imported a git user as {}",
        &profile_name
    );
    Ok(())
}
