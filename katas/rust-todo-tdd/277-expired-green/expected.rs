use chrono::{DateTime, Utc};
use domain::identity::{
    Actor, EmailAddress, PasswordHash, Role, StoredCredentials, UserRegistered,
};
use domain::shared::UserId;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum AuthStoreError {
    #[error("user row {0} is corrupt")]
    Corrupt(Uuid),
    #[error("email address is already registered")]
    EmailTaken,
    #[error(transparent)]
    Db(#[from] sqlx::Error),
}

/// The authenticated user behind a session cookie.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionUser {
    pub actor: Actor,
    pub email: EmailAddress,
}

pub async fn insert_user(pool: &PgPool, user: &UserRegistered) -> Result<(), AuthStoreError> {
    sqlx::query!(
        "INSERT INTO users (id, email, password_hash, role, created_at) VALUES ($1, $2, $3, $4, $5)",
        user.id.into_uuid(),
        user.email.as_str(),
        user.password_hash.as_str(),
        user.role.as_str(),
        user.registered_at,
    )
    .execute(pool)
    .await
    .map_err(|err| match &err {
        sqlx::Error::Database(db) if db.is_unique_violation() => AuthStoreError::EmailTaken,
        _ => AuthStoreError::Db(err),
    })?;
    Ok(())
}

pub async fn find_credentials(
    pool: &PgPool,
    email: &EmailAddress,
) -> Result<Option<StoredCredentials>, AuthStoreError> {
    let row = sqlx::query!(
        "SELECT id, role, password_hash FROM users WHERE email = $1",
        email.as_str()
    )
    .fetch_optional(pool)
    .await?;

    row.map(|r| {
        Ok(StoredCredentials {
            user_id: UserId::from_uuid(r.id),
            role: Role::parse(&r.role).ok_or(AuthStoreError::Corrupt(r.id))?,
            password_hash: PasswordHash::new(r.password_hash),
        })
    })
    .transpose()
}

pub async fn create_session(
    pool: &PgPool,
    token_hash: &str,
    user: UserId,
    now: DateTime<Utc>,
    expires_at: DateTime<Utc>,
) -> Result<(), AuthStoreError> {
    sqlx::query!(
        "INSERT INTO sessions (token_hash, user_id, created_at, expires_at) VALUES ($1, $2, $3, $4)",
        token_hash,
        user.into_uuid(),
        now,
        expires_at,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_session(pool: &PgPool, token_hash: &str) -> Result<(), AuthStoreError> {
    sqlx::query!("DELETE FROM sessions WHERE token_hash = $1", token_hash)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn find_session_user(
    pool: &PgPool,
    token_hash: &str,
    now: DateTime<Utc>,
) -> Result<Option<SessionUser>, AuthStoreError> {
    let row = sqlx::query!(
        r#"SELECT u.id, u.email, u.role
           FROM sessions s JOIN users u ON u.id = s.user_id
           WHERE s.token_hash = $1 AND s.expires_at > $2"#,
        token_hash,
        now,
    )
    .fetch_optional(pool)
    .await?;

    row.map(|r| {
        let corrupt = || AuthStoreError::Corrupt(r.id);
        Ok(SessionUser {
            actor: Actor {
                id: UserId::from_uuid(r.id),
                role: Role::parse(&r.role).ok_or_else(corrupt)?,
            },
            email: EmailAddress::parse(&r.email).map_err(|_| corrupt())?,
        })
    })
    .transpose()
}

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

    #[sqlx::test(migrations = "../../migrations")]
    async fn an_unknown_email_has_no_credentials(pool: PgPool) {
        let email = EmailAddress::parse("nobody@example.com").unwrap();
        assert!(find_credentials(&pool, &email).await.unwrap().is_none());
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn the_same_email_cannot_register_twice(pool: PgPool) {
        insert_user(&pool, &registration("alice@example.com"))
            .await
            .unwrap();
        let err = insert_user(&pool, &registration("ALICE@example.com"))
            .await
            .unwrap_err();
        assert!(matches!(err, AuthStoreError::EmailTaken), "got {err:?}");
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn a_live_session_resolves_to_its_user(pool: PgPool) {
        let user = registration("alice@example.com");
        insert_user(&pool, &user).await.unwrap();
        create_session(
            &pool,
            "tokenhash",
            user.id,
            t(9),
            t(9) + chrono::Duration::days(7),
        )
        .await
        .unwrap();

        let current = find_session_user(&pool, "tokenhash", t(10))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(current.actor.id, user.id);
        assert_eq!(current.email.as_str(), "alice@example.com");
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn an_expired_or_deleted_session_resolves_to_nobody(pool: PgPool) {
        let user = registration("alice@example.com");
        insert_user(&pool, &user).await.unwrap();
        create_session(&pool, "old", user.id, t(1), t(2))
            .await
            .unwrap();
        create_session(
            &pool,
            "gone",
            user.id,
            t(9),
            t(9) + chrono::Duration::days(7),
        )
        .await
        .unwrap();
        delete_session(&pool, "gone").await.unwrap();

        assert!(
            find_session_user(&pool, "old", t(10))
                .await
                .unwrap()
                .is_none()
        );
        assert!(
            find_session_user(&pool, "gone", t(10))
                .await
                .unwrap()
                .is_none()
        );
    }
}
