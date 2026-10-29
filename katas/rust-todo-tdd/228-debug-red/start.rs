use thiserror::Error;

/// A plaintext password that passed the policy. Deliberately not Serialize or Display.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);

pub const MIN_PASSWORD_CHARS: usize = 12;
pub const MAX_PASSWORD_CHARS: usize = 128;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PasswordError {
    #[error("password must be at least {min} characters")]
    TooShort { min: usize },
    #[error("password must be at most {max} characters")]
    TooLong { max: usize },
}

impl Password {
    pub fn parse(raw: &str) -> Result<Self, PasswordError> {
        match raw.chars().count() {
            n if n < MIN_PASSWORD_CHARS => Err(PasswordError::TooShort {
                min: MIN_PASSWORD_CHARS,
            }),
            n if n > MAX_PASSWORD_CHARS => Err(PasswordError::TooLong {
                max: MAX_PASSWORD_CHARS,
            }),
            _ => Ok(Self(raw.to_owned())),
        }
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

    #[test]
    fn rejects_absurdly_long_passwords() {
        assert_eq!(
            Password::parse(&"x".repeat(129)),
            Err(PasswordError::TooLong { max: 128 })
        );
    }
}
