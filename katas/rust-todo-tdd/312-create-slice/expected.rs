use leptos::prelude::*;

use super::TodoView;
use crate::error::AppError;

#[server(endpoint = "todos/create")]
pub async fn create_todo(title: String) -> Result<TodoView, AppError> {
    shell::handle(title).await.map_err(crate::effects::fail)
}

#[cfg(feature = "ssr")]
mod shell {
    use chrono::Utc;
    // aliases: the #[server] macro already generated a struct named `CreateTodo`
    use domain::shared::TodoId;
    use domain::todos::{CreateTodo as Command, Todo, create_todo as decide};
    use uuid::Uuid;

    use super::*;
    use crate::effects;
    use crate::features::todos::store;

    pub async fn handle(title: String) -> Result<TodoView, AppError> {
        let user = effects::require_user().await?;
        let state = effects::state()?;

        let event = decide(
            Command {
                owner: user.actor.id,
                title,
            },
            TodoId::from_uuid(Uuid::new_v4()),
            Utc::now(),
        )?;

        let todo = Todo::Active(event.todo);
        store::insert(&state.pool, &todo).await?;
        Ok(TodoView::from(&todo))
    }
}
