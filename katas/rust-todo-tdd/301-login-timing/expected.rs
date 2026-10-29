use leptos::prelude::*;

use super::UserView;
use crate::error::AppError;

#[server(endpoint = "auth/login")]
pub async fn login(email: String, password: String) -> Result<UserView, AppError> {
    shell::handle(email, password)
        .await
        .map_err(crate::effects::fail)
}

#[cfg(feature = "ssr")]
mod shell {
    use chrono::Utc;
    use domain::identity::{EmailAddress, Role, authenticate};

    use super::*;
    use crate::effects;
    use crate::features::auth::{crypto, store};

    pub async fn handle(email: String, password: String) -> Result<UserView, AppError> {
        let state = effects::state()?;
        let email = EmailAddress::parse(&email).ok();
        let stored = match &email {
            Some(email) => store::find_credentials(&state.pool, email).await?,
            None => None,
        };

        if stored.is_none() {
            crypto::verify_password(&password, &crypto::DUMMY_HASH);
        }

        let actor = authenticate(stored, &password, crypto::verify_password)?;

        effects::start_session(&state, actor.id, Utc::now()).await?;
        Ok(UserView {
            email: email.map(String::from).unwrap_or_default(),
            is_admin: actor.role == Role::Admin,
        })
    }
}
