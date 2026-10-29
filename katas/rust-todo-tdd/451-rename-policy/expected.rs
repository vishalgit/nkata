use leptos::prelude::*;
use uuid::Uuid;

use super::TodoView;
use crate::error::AppError;

#[server(endpoint = "todos/rename")]
pub async fn rename_todo(id: Uuid, title: String) -> Result<TodoView, AppError> {
    shell::handle(id, title).await.map_err(crate::effects::fail)
}

#[cfg(feature = "ssr")]
mod shell {
    use domain::identity::Action;
    use domain::todos::rename_todo as decide;

    use super::*;
    use crate::effects;
    use crate::features::todos::{access, store};

    pub async fn handle(id: Uuid, title: String) -> Result<TodoView, AppError> {
        let user = effects::require_user().await?;
        let state = effects::state()?;
        let todo = access::load_for(&user.actor, &state.pool, id, |owner| Action::EditTodo {
            owner,
        })
        .await?;

        let event = decide(todo, &title)?;

        store::save(&state.pool, &event.todo).await?;
        Ok(TodoView::from(&event.todo))
    }
}
