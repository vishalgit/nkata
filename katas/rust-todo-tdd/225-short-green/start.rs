use thiserror::Error;

/// A plaintext password that passed the policy. Deliberately not Serialize or Display.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PasswordError {}

impl Password {
    pub fn parse(raw: &str) -> Result<Self, PasswordError> {
        Ok(Self(raw.to_owned()))
    }

    /// The only way to read the secret. Grep for this name in code review.
    pub fn expose_secret(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_twelve_or_more_characters() {
        assert_eq!(
            Password::parse("correct horse").unwrap().expose_secret(),
            "correct horse"
        );
    }

    #[test]
    fn rejects_short_passwords() {
        assert_eq!(
            Password::parse("short"),
            Err(PasswordError::TooShort { min: 12 })
        );
    }
}
