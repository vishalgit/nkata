#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, TimeZone, Utc};
    use domain::identity::{HashingFailed, RegisterUser, register_user};

    fn t(hour: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 1, 1, hour, 0, 0).unwrap()
    }

    fn registration(email: &str) -> UserRegistered {
        let cmd = RegisterUser {
            email: email.into(),
            password: "correct horse battery".into(),
        };
        let fake = |_: &_| Ok::<_, HashingFailed>(PasswordHash::new("phc".into()));
        register_user(cmd, UserId::from_uuid(Uuid::new_v4()), t(9), fake).unwrap()
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn a_registered_user_can_be_found_by_email(pool: PgPool) {
        let user = registration("alice@example.com");
        insert_user(&pool, &user).await.unwrap();

        let creds = find_credentials(&pool, &user.email).await.unwrap().unwrap();
        assert_eq!(creds.user_id, user.id);
        assert_eq!(creds.role, Role::User);
        assert_eq!(creds.password_hash.as_str(), "phc");
    }
}
