#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalises_case_and_whitespace() {
        let email = EmailAddress::parse("  Alice@Example.COM ").unwrap();
        assert_eq!(email.as_str(), "alice@example.com");
    }
}
