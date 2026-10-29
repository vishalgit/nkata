use leptos::prelude::*;

use crate::error::AppError;

#[server(endpoint = "auth/logout")]
pub async fn logout() -> Result<(), AppError> {
    shell::handle().await.map_err(crate::effects::fail)
}

#[cfg(feature = "ssr")]
mod shell {
    use super::*;
    use crate::effects;

    pub async fn handle() -> Result<(), AppError> {
        let state = effects::state()?;
        effects::end_session(&state).await
    }
}
