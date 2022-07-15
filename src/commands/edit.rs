use crate::{
    config::{Config, Profile},
    context::AppContext, validation::email::is_email,
};
use anyhow::{anyhow, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

fn retrieve_profile_key(config: &Config, supplied_profile_key: &Option<String>) -> Result<String> {
    // If the user supplied the key, we use that here.
    if let Some(profile_key) = supplied_profile_key {
        if !config.has_profile_key(&profile_key) {
            return Err(anyhow!("Profile {} doesn't exist", profile_key));
        }
        return Ok(profile_key.to_owned())
    }

    // Now, we need to ask the user which profile to use.

    // If there's only one profile, obviously we don't need any input.
    // Just use that as profile key.
    if config.profile.len() == 1 {
        return Ok(config.profile.first().as_ref().unwrap().0.to_owned())
    }

    let profile_key_options: Vec<String> = Vec::from_iter(config.profile.keys().cloned());

    let selected_index = Select::with_theme(&ColorfulTheme::default())
        .items(&profile_key_options)
        .with_prompt("Select which profile to edit")
        // .default(0)
        .interact()?;

    Ok(profile_key_options[selected_index].to_owned())
}

fn retrieve_profile(config: &Config, profile_key: &str) -> Result<Profile> {
    let target_profile = &config.profile[profile_key];

    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter user name (user.name)")
        .with_initial_text(&target_profile.name)
        .interact_text()?;

    let email: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter email (user.email)")
        .with_initial_text(&target_profile.email)
        .validate_with(|input: &String| is_email(input))
        .interact_text()?;
    
    let maybe_signingkey: Option<String> = if target_profile.signingkey.is_none() {
        // The profile didn't have a signing key.
        // Asking the user if it's needed
        let should_set_signingkey = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to set signing key (user.signingkey)")
            .default(false)
            .interact()?;
        if should_set_signingkey {
            let signingkey = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter signing key (user.singingkey)")
                .interact_text()?;
            Some(signingkey)
        } else {
            None
        }
    } else {
        // The profile has signing key set up.
        // The user can choose to retain it or to discard it.
        let old_signingkey = target_profile.signingkey.as_ref().unwrap();

        let signingkey_input: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter signing key (user.singingkey)")
            .with_initial_text(old_signingkey)
            // Empty string means discarding the key
            .allow_empty(true) 
            .interact_text()?;

        if signingkey_input.is_empty() {
            None
        } else {
            Some(signingkey_input.to_owned())
        }
    };

    let new_profile = Profile {
        name,
        email,
        signingkey: maybe_signingkey,
    };

    Ok(new_profile)
}

pub fn execute(context: &AppContext, maybe_profile_key: &Option<String>) -> Result<()> {
    let config = context.config_client.load()?;

    if config.profile.is_empty() {
        return Err(anyhow!("No profile has been set up yet!"));
    }

    let profile_key = if config.profile.len() == 1 {
        // If there's only one profile, we don't need to ask which to select.
        config.profile.first().as_ref().unwrap().0.to_owned()
    } else {
        retrieve_profile_key(&config, &maybe_profile_key)?
    };

    let new_profile = retrieve_profile(&config, &profile_key)?;

    let mut config = config;
    config.upsert_profile(&profile_key, new_profile);
    let config = config;

    context.config_client.save(&config)?;

    println!("✨ Successfully modified {}", profile_key);

    Ok(())
}
