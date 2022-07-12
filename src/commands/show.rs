use anyhow::{anyhow, Result};

use crate::{config::Profile, context::AppContext};

pub enum ProfileProperty {
    Name,
    Email,
    SigningKey,
    ProfileKey,
    Profile,
}

fn print_profile(profile: &Profile) -> String {
    let mut lines = vec![
        format!("user.name={}", profile.name),
        format!("user.email={}", profile.email),
    ];
    if let Some(signingkey) = profile.signingkey.as_ref() {
        lines.push(format!("user.signingkey={}", signingkey));
    }
    let lines = lines;
    lines.join("\n")
}

pub fn execute(context: &AppContext, profile_key: &str) -> Result<()> {
    let config = context.config_client.load()?;

    if let Some(profile) = config.profile.get(profile_key) {
        println!("{}", print_profile(profile));
        Ok(())
    } else {
        Err(anyhow!("Profile {} doesn't exist.", profile_key))
    }
}
