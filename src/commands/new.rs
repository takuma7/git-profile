use crate::{config::{Profile, Config}, context::AppContext, git::find_profile_key, validation::{self, email::is_email}, commands::rename};
use anyhow::{Result};
use dialoguer::{Confirm, Input, theme::ColorfulTheme};

fn retrieve_profile_key_value(config: &Config) -> Result<(String, Profile)> {
    let profile_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter profile name")
        .validate_with(|input: &String| -> Result<()> {
            validation::profile_key::no_duplicates(input, &config.profile)?;
            Ok(())
        })
        .interact_text()?;

    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter user name (user.name)")
        .interact_text()?;

    let email: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter email (user.email)")
        .validate_with(|input: &String| is_email(input))
        .interact_text()?;

    let should_set_signingkey = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to set signing key (user.signingkey)")
        .default(false)
        .interact()?;

    let maybe_signingkey: Option<String> = if should_set_signingkey {
        let signingkey = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter signing key (user.singingkey)")
            .interact_text()?;
        Some(signingkey)
    } else {
        None
    };

    let new_profile = Profile {
        name,
        email,
        signingkey: maybe_signingkey,
    };

    Ok((profile_name, new_profile))
}

pub fn execute(context: &AppContext) -> Result<()> {
    let config = context.config_client.load()?;

    let (profile_name, new_profile) = retrieve_profile_key_value(&config)?;

    // Since some commands such as `git profile current` depend on value matching,
    // it should not be allow to create a new profile with the same values as one of the existing ones.
    if let Some(existing_profile_key) = find_profile_key(&config.profile, &new_profile) {
        println!(
            "We found an existing profile with the same values: {}",
            existing_profile_key
        );
        let should_rename = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Do you wish to rename this to {}?", &profile_name))
            .default(false)
            .interact()?;
        if should_rename {
            rename::execute(&context, &existing_profile_key, &profile_name)?;
        }
        return Ok(())
    }

    let mut config = config;
    config.upsert_profile(&profile_name, new_profile);
    let config = config;

    context.config_client.save(&config)?;

    println!("✨ Created a new profile {}", profile_name);

    Ok(())
}
