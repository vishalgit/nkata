use leptos::prelude::*;
use uuid::Uuid;

use crate::error::AppError;

#[server(endpoint = "todos/delete")]
pub async fn delete_todo(id: Uuid) -> Result<(), AppError> {
    shell::handle(id).await.map_err(crate::effects::fail)
}

#[cfg(feature = "ssr")]
mod shell {
    use domain::identity::Action;

    use super::*;
    use crate::effects;
    use crate::features::todos::{access, store};

    pub async fn handle(id: Uuid) -> Result<(), AppError> {
        let user = effects::require_user().await?;
        let state = effects::state()?;
        let todo = access::load_for(&user.actor, &state.pool, id, |owner| Action::DeleteTodo {
            owner,
        })
        .await?;
        store::delete(&state.pool, todo.info().id).await?;
        Ok(())
    }
}
