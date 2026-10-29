use leptos::prelude::*;
use uuid::Uuid;

use super::TodoView;
use crate::error::AppError;

#[server(endpoint = "todos/reopen")]
pub async fn reopen_todo(id: Uuid) -> Result<TodoView, AppError> {
    shell::handle(id).await.map_err(crate::effects::fail)
}

#[cfg(feature = "ssr")]
mod shell {
    use domain::identity::Action;
    use domain::todos::{Todo, reopen_todo as decide};

    use super::*;
    use crate::effects;
    use crate::features::todos::{access, store};

    pub async fn handle(id: Uuid) -> Result<TodoView, AppError> {
        let user = effects::require_user().await?;
        let state = effects::state()?;
        let todo = access::load_for(&user.actor, &state.pool, id, |owner| Action::EditTodo {
            owner,
        })
        .await?;

        let event = decide(todo)?;

        let todo = Todo::Active(event.todo);
        store::save(&state.pool, &todo).await?;
        Ok(TodoView::from(&todo))
    }
}
