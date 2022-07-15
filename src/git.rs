use anyhow::{Context, Result};

use crate::config::{Config, Profile};
use indexmap::{IndexMap};
use std::process::Command;

/// Where to store git-config values
pub enum Level {
    /// https://git-scm.com/docs/git-config#Documentation/git-config.txt---global
    Global,
    /// https://git-scm.com/docs/git-config#Documentation/git-config.txt---system
    System,
    /// https://git-scm.com/docs/git-config#Documentation/git-config.txt---local
    Local,
    /// https://git-scm.com/docs/git-config#Documentation/git-config.txt---worktree
    WorkTree,
    /// https://git-scm.com/docs/git-config#Documentation/git-config.txt---fileltconfig-filegt
    File(String),
}

const USER_NAME: &'static str = "user.name";
const USER_EMAIL: &'static str = "user.email";
const USER_SIGNING_KEY: &'static str = "user.signingkey";

const GET_FLAG: &'static str = "--get";
const UNSET_FLAG: &'static str = "--unset";

fn get_level_flag(level: &Level) -> String {
    match level {
        Level::Global => "--global".to_owned(),
        Level::System => "--system".to_owned(),
        Level::Local => "--local".to_owned(),
        Level::WorkTree => "--worktree".to_owned(),
        Level::File(config_file_path) => format!("--file {config_file_path}"),
    }
}

pub trait GitConfigWrite {
    fn set(&self, profile: &Profile, maybe_level: &Option<Level>) -> Result<()>;
}

pub trait GitConfigRead {
    /// Gets the output of `git config --get user.name`
    fn get_name(&self, maybe_level: &Option<Level>) -> Result<Option<String>>;
    /// Gets the output of `git config --get user.email`
    fn get_email(&self, maybe_level: &Option<Level>) -> Result<Option<String>>;
    /// Gets the output of `git config --get user.signingkey`
    fn get_signingkey(&self, maybe_level: &Option<Level>) -> Result<Option<String>>;
    /// Constructs a profile object from the output of git config
    fn get(&self, maybe_level: &Option<Level>) -> Result<Option<Profile>>;
    /// Searches for a profile key in the config file based on the current git config
    fn get_profile_key(&self, config: &Config, maybe_level: &Option<Level>) -> Result<Option<String>>;
}

pub trait GitConfigClientType: GitConfigWrite + GitConfigRead {}
impl<T> GitConfigClientType for T where T: GitConfigRead + GitConfigWrite {}

pub struct GitConfigClient;

impl GitConfigClient {
    pub fn new() -> Self {
        GitConfigClient {}
    }
}

fn git_config(maybe_level: &Option<Level>) -> Command {
    let mut cmd = Command::new("git");
    cmd.arg("config");
    if let Some(level) = maybe_level {
        cmd.arg(get_level_flag(&level));
    }
    cmd
}

impl GitConfigWrite for GitConfigClient {
    fn set(&self, profile: &Profile, maybe_level: &Option<Level>) -> Result<()> {
        git_config(&maybe_level).args([USER_NAME, &profile.name]).output()?;
        git_config(&maybe_level).args([USER_EMAIL, &profile.email]).output()?;
        if let Some(signingkey) = profile.signingkey.as_ref() {
            git_config(&maybe_level).args([USER_SIGNING_KEY, signingkey]).output()?;
        } else {
            // Unset user.signingkey in case it was set in the old config
            git_config(&maybe_level).args([UNSET_FLAG, USER_SIGNING_KEY]).output()?;
        }

        Ok(())
    }
}

impl GitConfigRead for GitConfigClient {
    fn get_name(&self, maybe_level: &Option<Level>) -> Result<Option<String>> {
        let output = git_config(&maybe_level).args([GET_FLAG, USER_NAME]).output()?;
        Ok(Some(String::from_utf8(output.stdout)?.trim().to_string()))
    }

    fn get_email(&self, maybe_level: &Option<Level>) -> Result<Option<String>> {
        let output = git_config(&maybe_level).args([GET_FLAG, USER_EMAIL]).output()?;
        Ok(Some(String::from_utf8(output.stdout)?.trim().to_string()))
    }

    fn get_signingkey(&self, maybe_level: &Option<Level>) -> Result<Option<String>> {
        let output = git_config(&maybe_level).args([GET_FLAG, USER_SIGNING_KEY]).output()?;
        if output.status.success() {
            Ok(Some(String::from_utf8(output.stdout)?.trim().to_string()))
        } else {
            Ok(None)
        }
    }

    fn get(&self, maybe_level: &Option<Level>) -> Result<Option<Profile>> {
        let maybe_name = self.get_name(&maybe_level)?;
        let maybe_email = self.get_email(&maybe_level)?;
        let signingkey = self.get_signingkey(&maybe_level)?;

        if let (Some(name), Some(email)) = (maybe_name, maybe_email) {
            Ok(Some(Profile {
                name,
                email,
                signingkey,
            }))
        } else {
            Ok(None)
        }
    }

    fn get_profile_key(&self, config: &Config, maybe_level: &Option<Level>) -> Result<Option<String>> {
        if let Some(target) = self.get(&maybe_level).with_context(|| "Current profile not found")? {
            let profile_key = find_profile_key(&config.profile, &target);
            Ok(profile_key)
        } else {
            Ok(None)
        }
    }
}

pub fn find_profile_key(
    profile_catalog: &IndexMap<String, Profile>,
    target: &Profile,
) -> Option<String> {
    profile_catalog.iter().find_map(|(profile_key, profile)| {
        if *profile == *target {
            Some(String::from(profile_key))
        } else {
            None
        }
    })
}

#[cfg(test)]
mod find_profile_key_tests {
    use super::*;
    use indexmap::{indexmap};

    #[test]
    fn has_match() {
        let catalog = indexmap! {
            "a".to_string() => Profile {
                name: "a".to_string(),
                email: "a@example.com".to_string(),
                signingkey: None
            },
            "b".to_string() => Profile {
                name: "b".to_string(),
                email: "b@example.com".to_string(),
                signingkey: None
            },
            "c".to_string() => Profile {
                name: "c".to_string(),
                email: "c@example.com".to_string(),
                signingkey: Some("signingkey".to_string())
            }
        };
        let result_b = find_profile_key(
            &catalog,
            &Profile {
                name: "b".to_string(),
                email: "b@example.com".to_string(),
                signingkey: None
            }
        );
        assert!(result_b.is_some());
        assert_eq!(result_b.unwrap(), "b")
    }

    #[test]
    fn no_match() {
        let catalog = indexmap! {
            "a".to_string() => Profile {
                name: "a".to_string(),
                email: "a@example.com".to_string(),
                signingkey: None
            },
            "b".to_string() => Profile {
                name: "b".to_string(),
                email: "b@example.com".to_string(),
                signingkey: None
            },
            "c".to_string() => Profile {
                name: "c".to_string(),
                email: "c@example.com".to_string(),
                signingkey: Some("signingkey".to_string())
            }
        };
        let result_a = find_profile_key(
            &catalog,
            &Profile {
                name: "a".to_string(),
                email: "b@example.com".to_string(),
                signingkey: None
            }
        );
        assert!(result_a.is_none());

        let result_b = find_profile_key(
            &catalog,
            &Profile {
                name: "b".to_string(),
                email: "b@example.com".to_string(),
                signingkey: Some("test".to_string())
            }
        );
        assert!(result_b.is_none());

        let result_d = find_profile_key(
            &catalog,
            &Profile {
                name: "d".to_string(),
                email: "d@example.com".to_string(),
                signingkey: None
            }
        );
        assert!(result_d.is_none());
    }
}
