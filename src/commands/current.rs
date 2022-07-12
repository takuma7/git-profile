use anyhow::{anyhow, Result};

use crate::{context::AppContext, git::Level};

pub enum ProfileProperty {
    Name,
    Email,
    SigningKey,
    ProfileKey,
    Profile,
}

fn generate_output(context: &AppContext, selected_property: &ProfileProperty, maybe_level: &Option<Level>) -> Result<Option<String>> {
    let git_config_client = &context.git_config_client;
    let config = context.config_client.load()?;

    if let Some(current_profile_key) = git_config_client.get_profile_key(&config, maybe_level)? {
        let current_profile = config.profile.get(&current_profile_key).unwrap();
        match selected_property {
            ProfileProperty::Name => Ok(Some(current_profile.name.to_owned())),
            ProfileProperty::Email => Ok(Some(current_profile.email.to_owned())),
            ProfileProperty::SigningKey => Ok(current_profile.signingkey.to_owned()),
            ProfileProperty::Profile => {
                let mut lines = vec![
                    format!("profile.key={}", current_profile_key),
                    format!("user.name={}", current_profile.name),
                    format!("user.email={}", current_profile.email),
                ];
                if let Some(signingkey) = current_profile.signingkey.as_ref() {
                    lines.push(format!("user.signingkey={}", signingkey));
                }
                let lines = lines;
                Ok(Some(lines.join("\n")))
            }
            ProfileProperty::ProfileKey => Ok(Some(current_profile_key)),
        }
    } else {
        Ok(None)
    }
}

pub fn execute(context: &AppContext, selected_property: &ProfileProperty, maybe_level: &Option<Level>) -> Result<()> {
    if let Some(output) = generate_output(&context, &selected_property, &maybe_level)? {
        println!("{}", output);
        Ok(())
    } else {
        Err(anyhow!("Empty"))
    }
}
