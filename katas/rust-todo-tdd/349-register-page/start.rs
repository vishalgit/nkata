use leptos::prelude::*;

use super::UserView;
use crate::error::AppError;

#[server(endpoint = "auth/register")]
pub async fn register(email: String, password: String) -> Result<UserView, AppError> {
    shell::handle(email, password)
        .await
        .map_err(crate::effects::fail)
}

#[cfg(feature = "ssr")]
mod shell {
    use chrono::Utc;
    use domain::identity::{RegisterUser, register_user};
    use domain::shared::UserId;
    use uuid::Uuid;

    use super::*;
    use crate::effects;
    use crate::features::auth::{crypto, store};

    pub async fn handle(email: String, password: String) -> Result<UserView, AppError> {
        // impure: gather
        let state = effects::state()?;
        let (id, now) = (UserId::from_uuid(Uuid::new_v4()), Utc::now());
        // pure: decide
        let user = register_user(
            RegisterUser { email, password },
            id,
            now,
            crypto::hash_password,
        )?;
        // impure: persist and respond
        store::insert_user(&state.pool, &user).await?;
        effects::start_session(&state, user.id, now).await?;
        Ok(UserView {
            email: user.email.as_str().to_owned(),
            is_admin: false,
        })
    }
}
