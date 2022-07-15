use anyhow::{anyhow, Result};

use crate::config::ProfileMap;

pub fn no_duplicates(input: &str, profile_catalog: &ProfileMap) -> Result<()> {
    if profile_catalog.contains_key(input) {
        Err(anyhow!("Already in use"))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Profile;

    use super::*;
    use indexmap::indexmap;

    #[test]
    fn has_duplicates() {
        let input = "existing";
        let profile_catalog = indexmap! {
            "existing".to_string() => Profile { name: "".to_string(), email: "".to_string(), signingkey: None },
            "other".to_string() => Profile { name: "".to_string(), email: "".to_string(), signingkey: None }
        };
        assert!(no_duplicates(input, &profile_catalog).is_err());
    }


    #[test]
    fn has_no_duplicates() {
        let input = "new";
        let profile_catalog = indexmap! {
            "existing".to_string() => Profile { name: "".to_string(), email: "".to_string(), signingkey: None },
            "other".to_string() => Profile { name: "".to_string(), email: "".to_string(), signingkey: None }
        };
        assert!(no_duplicates(input, &profile_catalog).is_ok());
    }
}
