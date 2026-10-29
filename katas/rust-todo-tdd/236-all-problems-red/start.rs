use chrono::{DateTime, Utc};
use thiserror::Error;

use super::{EmailAddress, EmailError, Password, PasswordError, Role};
use crate::shared::UserId;

/// An Argon2 PHC string such as `$argon2id$v=19$...`.
#[derive(Clone, PartialEq, Eq)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(phc: String) -> Self {
        Self(phc)
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for PasswordHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("PasswordHash(***)")
    }
}

/// Returned by a hashing function that failed. Carries no details on purpose.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HashingFailed;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisterUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserRegistered {
    pub id: UserId,
    pub email: EmailAddress,
    pub password_hash: PasswordHash,
    pub role: Role,
    pub registered_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum RegistrationProblem {
    #[error(transparent)]
    Email(#[from] EmailError),
    #[error(transparent)]
    Password(#[from] PasswordError),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum RegisterError {
    #[error("registration is invalid")]
    Invalid(Vec<RegistrationProblem>),
    #[error("could not secure the password")]
    Hashing,
}

pub fn register_user<H>(
    cmd: RegisterUser,
    id: UserId,
    now: DateTime<Utc>,
    hash: H,
) -> Result<UserRegistered, RegisterError>
where
    H: FnOnce(&Password) -> Result<PasswordHash, HashingFailed>,
{
    let invalid = |p: RegistrationProblem| RegisterError::Invalid(vec![p]);
    let email = EmailAddress::parse(&cmd.email).map_err(|e| invalid(e.into()))?;
    let password = Password::parse(&cmd.password).map_err(|e| invalid(e.into()))?;
    let password_hash = hash(&password).map_err(|_| RegisterError::Hashing)?;
    Ok(UserRegistered {
        id,
        email,
        password_hash,
        role: Role::User,
        registered_at: now,
    })
}

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
