#[cfg(test)]
mod tests {
    use super::*;

    fn pw(raw: &str) -> Password {
        Password::parse(raw).unwrap()
    }

    #[test]
    fn a_hash_verifies_the_original_password_only() {
        let hash = hash_password(&pw("correct horse battery")).unwrap();
        assert!(hash.as_str().starts_with("$argon2id$"));
        assert!(verify_password("correct horse battery", &hash));
        assert!(!verify_password("wrong horse battery", &hash));
    }
}
