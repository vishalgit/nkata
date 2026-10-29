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
        Ok(Self(raw.trim().to_lowercase()))
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
}
