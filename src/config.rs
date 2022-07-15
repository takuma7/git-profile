use anyhow::{anyhow, Context, Result};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use toml;

use std::fs;
use std::io::Read;
use std::path::PathBuf;

pub const DEFAULT_FILE_NAME: &'static str = "gitprofile.toml";

pub type ProfileMap = IndexMap<String, Profile>;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "IndexMap::new")]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    #[serde_as(as = "IndexMap<_, _>")]
    pub profile: ProfileMap
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Profile {
    pub name: String,
    pub email: String,
    pub signingkey: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            profile: IndexMap::new(),
        }
    }

    pub fn has_profile_key(&self, key: &str) -> bool {
        self.profile.contains_key(key)
    }

    /// Upserts a profile to the given key
    pub fn upsert_profile(&mut self, key: &str, value: Profile) -> Option<Profile> {
        self.profile.insert(key.to_owned(), value)
    }

    /// Removes the key and its associated profile
    pub fn remove_profile(&mut self, key: &str) -> Option<Profile> {
        self.profile.shift_remove(key)
    }

    /// Renames the given profile
    pub fn rename_profile(&mut self, old_key: &str, new_key: &str) -> Option<Profile> {
        if let Some(old_profile) = self.remove_profile(old_key) {
            self.upsert_profile(new_key, old_profile)
        } else {
            None
        }
    }
}

pub struct AppConfigClient {
    path: PathBuf,
}

impl AppConfigClient {
    pub fn new(path: PathBuf) -> Self {
        AppConfigClient { path }
    }
}

pub trait Persist {
    fn load(&self) -> Result<Config>;
    fn save(&self, config: &Config) -> Result<()>;
}

impl Persist for AppConfigClient {
    fn load(&self) -> Result<Config> {
        // Create the containing dir if not exists
        let containing_dir = match self.path.parent() {
            Some(parent_path) => parent_path,
            None => return Err(anyhow!("The path {} has no parent.", &self.path.display())),
        };
        fs::create_dir_all(containing_dir)?;

        // Create the file if not exists
        fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(&self.path)
            .with_context(|| format!("Can't open {}", &self.path.as_path().display()))?;

        // Then, actually open the file for reading
        let mut file = fs::OpenOptions::new()
            .read(true)
            .open(&self.path)
            .with_context(|| format!("Can't open {}", &self.path.as_path().display()))?;

        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    fn save(&self, config: &Config) -> Result<()> {
        let content = toml::to_string(config)?;
        fs::write(&self.path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod deserialize {
    use super::*;

    #[test]
    fn empty() -> Result<()> {
        let config: Config = toml::from_str("")?;
        assert!(config.profile.is_empty());
        Ok(())
    }

    #[test]
    fn multiple_entries() {
        let config: Config = toml::from_str(
            r#"
            [profile.default]
            name = 'Foo Bar'
            email = 'foo@bar.com'
            signingkey = 'whatever'

            [profile.no_signingkey]
            name = 'Foo Bar'
            email = 'foo@bar.com'
        "#,
        )
        .unwrap();
        assert_eq!(config.profile["default"].name, "Foo Bar");
        assert_eq!(config.profile["default"].email, "foo@bar.com");
        assert_eq!(
            config.profile["default"].signingkey.as_ref().unwrap(),
            "whatever"
        );

        assert_eq!(config.profile["no_signingkey"].name, "Foo Bar");
        assert_eq!(config.profile["no_signingkey"].email, "foo@bar.com");
        assert_eq!(config.profile["no_signingkey"].signingkey, None);
    }
}

#[cfg(test)]
mod serialize {
    use super::*;
    use indexmap::indexmap;

    #[test]
    fn empty() {
        let config = Config {
            profile: IndexMap::new(),
        };
        let toml_str = toml::to_string(&config).unwrap();
        assert_eq!(toml_str, "");
    }

    #[test]
    fn multiple_entries() {
        let config = Config {
            profile: indexmap! {
                "default".to_string() => Profile {
                    name: "Iam Git".to_string(),
                    email: "iam@example.com".to_string(),
                    signingkey: None
                },
                "work".to_string() => Profile {
                    name: "Work Profile".to_string(),
                    email: "profile@work.com".to_string(),
                    signingkey: Some("whatever".to_string()),
                },
            },
        };
        let toml_str = toml::to_string(&config).unwrap();
        assert_eq!(
            toml_str,
            r#"[profile.default]
name = "Iam Git"
email = "iam@example.com"

[profile.work]
name = "Work Profile"
email = "profile@work.com"
signingkey = "whatever"
"#
        );
    }
}
