use thiserror::Error;

use super::{Actor, PasswordHash, Role};
use crate::shared::UserId;

/// What the shell loaded for the email the user typed, if anything.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredCredentials {
    pub user_id: UserId,
    pub role: Role,
    pub password_hash: PasswordHash,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum LoginError {
    #[error("invalid email or password")]
    InvalidCredentials,
}

pub fn authenticate<V>(
    stored: Option<StoredCredentials>,
    candidate: &str,
    verify: V,
) -> Result<Actor, LoginError>
where
    V: FnOnce(&str, &PasswordHash) -> bool,
{
    match stored {
        Some(creds) if verify(candidate, &creds.password_hash) => Ok(Actor {
            id: creds.user_id,
            role: creds.role,
        }),
        _ => Err(LoginError::InvalidCredentials),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn stored() -> StoredCredentials {
        StoredCredentials {
            user_id: UserId::from_uuid(Uuid::from_u128(7)),
            role: Role::Admin,
            password_hash: PasswordHash::new("hash-of-right".into()),
        }
    }
    fn verify(candidate: &str, hash: &PasswordHash) -> bool {
        hash.as_str() == format!("hash-of-{candidate}")
    }

    #[test]
    fn the_right_password_yields_the_actor_with_its_role() {
        let actor = authenticate(Some(stored()), "right", verify).unwrap();
        assert_eq!(
            actor,
            Actor {
                id: stored().user_id,
                role: Role::Admin
            }
        );
    }

    #[test]
    fn a_wrong_password_is_rejected() {
        assert_eq!(
            authenticate(Some(stored()), "wrong", verify),
            Err(LoginError::InvalidCredentials)
        );
    }

    #[test]
    fn an_unknown_user_gets_the_very_same_error() {
        assert_eq!(
            authenticate(None, "right", verify),
            Err(LoginError::InvalidCredentials)
        );
    }
}
