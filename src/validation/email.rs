use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;

pub fn is_email(input: &str) -> Result<()> {
    lazy_static! {
        // Using HTML5's regex: https://html.spec.whatwg.org/multipage/input.html#valid-e-mail-address
        static ref RE: Regex = Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap();
    }

    if RE.is_match(input) {
        Ok(())
    } else {
        Err(anyhow!("Invalid email address"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_inputs() {
        vec![
            "",
            "@",
            "@@",
            "a@b@",
            "a b",
            "test",
            "test@@example.com",
        ].into_iter().for_each(|input| {
            assert!(is_email(input).is_err(), "{} should return error", input);
        })
    }

    #[test]
    fn valid_inputs() {
        vec![
            "test@example.com",
            "what.is.this@foo.bar.com",
            "manydots...+something@gmail.com",
        ].into_iter().for_each(|input| {
            assert!(is_email(input).is_ok(), "{} should return ok", input);
        })
    }
}
