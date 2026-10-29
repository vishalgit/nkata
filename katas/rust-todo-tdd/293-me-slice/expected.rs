use leptos::prelude::*;

use super::UserView;
use crate::error::AppError;

#[server(endpoint = "auth/me")]
pub async fn me() -> Result<Option<UserView>, AppError> {
    shell::handle().await.map_err(crate::effects::fail)
}

#[cfg(feature = "ssr")]
mod shell {
    use super::*;
    use crate::effects;

    pub async fn handle() -> Result<Option<UserView>, AppError> {
        Ok(effects::current_user().await?.map(UserView::from))
    }
}
