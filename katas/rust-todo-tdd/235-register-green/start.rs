#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use uuid::Uuid;

    fn now() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 1, 1, 9, 0, 0).unwrap()
    }
    fn id() -> UserId {
        UserId::from_uuid(Uuid::from_u128(7))
    }
    fn fake_hash(pw: &Password) -> Result<PasswordHash, HashingFailed> {
        Ok(PasswordHash::new(format!("hashed:{}", pw.expose_secret())))
    }
    fn cmd(email: &str, password: &str) -> RegisterUser {
        RegisterUser {
            email: email.into(),
            password: password.into(),
        }
    }

    #[test]
    fn a_valid_registration_emits_a_user_with_the_user_role() {
        let event = register_user(
            cmd("Alice@Example.com", "correct horse battery"),
            id(),
            now(),
            fake_hash,
        )
        .unwrap();
        assert_eq!(event.email.as_str(), "alice@example.com");
        assert_eq!(event.role, Role::User);
        assert_eq!(event.password_hash.as_str(), "hashed:correct horse battery");
    }
}
