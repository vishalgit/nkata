use leptos::prelude::*;

use super::TodoPage;
use crate::error::AppError;

#[server(endpoint = "todos/list")]
pub async fn list_todos(
    filter: Option<String>,
    q: Option<String>,
    page: Option<u32>,
) -> Result<TodoPage, AppError> {
    shell::handle(filter, q, page)
        .await
        .map_err(crate::effects::fail)
}

#[cfg(feature = "ssr")]
mod shell {
    use domain::todos::TodoQuery;

    use super::*;
    use crate::effects;
    use crate::features::todos::store;

    pub async fn handle(
        filter: Option<String>,
        q: Option<String>,
        page: Option<u32>,
    ) -> Result<TodoPage, AppError> {
        let user = effects::require_user().await?;
        let state = effects::state()?;
        // pure: three raw strings become one valid query
        let query = TodoQuery::parse(filter.as_deref(), q.as_deref(), page)?;
        let paged = store::search_for_owner(&state.pool, user.actor.id, &query).await?;
        Ok(TodoPage::from(paged))
    }
}
