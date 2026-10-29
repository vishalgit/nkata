use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EmailAddress(String);

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum EmailError {
    #[error("email address is not valid")]
    Invalid,
}

impl EmailAddress {
    pub fn parse(raw: &str) -> Result<Self, EmailError> {
        let email = raw.trim().to_lowercase();
        let (local, domain) = email.split_once('@').ok_or(EmailError::Invalid)?;
        let valid = !local.is_empty()
            && !domain.contains('@')
            && domain.contains('.')
            && !domain.starts_with('.')
            && !domain.ends_with('.')
            && !email.chars().any(char::is_whitespace);
        if !valid {
            return Err(EmailError::Invalid);
        }
        Ok(Self(email))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<EmailAddress> for String {
    fn from(email: EmailAddress) -> Self {
        email.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalises_case_and_whitespace() {
        let email = EmailAddress::parse("  Alice@Example.COM ").unwrap();
        assert_eq!(email.as_str(), "alice@example.com");
    }

    #[test]
    fn rejects_addresses_missing_a_part() {
        for bad in ["alice", "@example.com", "alice@"] {
            assert_eq!(
                EmailAddress::parse(bad),
                Err(EmailError::Invalid),
                "{bad:?}"
            );
        }
    }

    #[test]
    fn rejects_malformed_domains() {
        for bad in [
            "alice@example",
            "alice@.com",
            "alice@example.",
            "a@b@c.com",
            "al ice@example.com",
        ] {
            assert_eq!(
                EmailAddress::parse(bad),
                Err(EmailError::Invalid),
                "{bad:?}"
            );
        }
    }

    #[test]
    fn rejects_overlong_addresses() {
        let raw = format!("{}@example.com", "a".repeat(250));
        assert_eq!(EmailAddress::parse(&raw), Err(EmailError::TooLong));
    }
}
