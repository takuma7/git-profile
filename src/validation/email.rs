use anyhow::{anyhow, Result};

pub fn is_email(input: &String) -> Result<()> {
    if input.contains('@') {
        Ok(())
    } else {
        Err(anyhow!("Invalid email address"))
    }
}
