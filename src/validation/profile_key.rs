use anyhow::{anyhow, Result};
use indexmap::IndexMap;

use crate::config::Profile;

pub fn no_duplicates(
    input: &String,
    profile_catalog: &IndexMap<String, Profile>,
) -> Result<()> {
    if profile_catalog.contains_key(input) {
        Err(anyhow!("Already in use"))
    } else {
        Ok(())
    }
}
