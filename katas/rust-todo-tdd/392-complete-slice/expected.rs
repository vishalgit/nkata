use leptos::prelude::*;
use uuid::Uuid;

use super::TodoView;
use crate::error::AppError;

#[server(endpoint = "todos/complete")]
pub async fn complete_todo(id: Uuid) -> Result<TodoView, AppError> {
    shell::handle(id).await.map_err(crate::effects::fail)
}

#[cfg(feature = "ssr")]
mod shell {
    use chrono::Utc;
    use domain::todos::{Todo, complete_todo as decide};

    use super::*;
    use crate::effects;
    use crate::features::todos::{access, store};

    pub async fn handle(id: Uuid) -> Result<TodoView, AppError> {
        let user = effects::require_user().await?;
        let state = effects::state()?;
        let todo = access::load_for(&user.actor, &state.pool, id).await?;

        let event = decide(todo, Utc::now())?;

        let todo = Todo::Completed(event.todo);
        store::save(&state.pool, &todo).await?;
        Ok(TodoView::from(&todo))
    }
}
