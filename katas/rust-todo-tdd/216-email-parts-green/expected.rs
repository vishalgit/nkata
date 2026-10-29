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
        if local.is_empty() || domain.is_empty() {
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
}
