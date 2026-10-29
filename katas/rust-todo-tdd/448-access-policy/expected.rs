use domain::identity::{Action, Actor, authorize};
use domain::shared::{TodoId, UserId};
use domain::todos::Todo;
use sqlx::PgPool;
use uuid::Uuid;

use super::store;
use crate::error::AppError;

/// Loads a todo if `actor` may perform `action` on it.
/// A denial is reported as NotFound, so ids of other people's todos never leak.
pub async fn load_for(
    actor: &Actor,
    pool: &PgPool,
    id: Uuid,
    action: impl FnOnce(UserId) -> Action,
) -> Result<Todo, AppError> {
    let todo = store::find(pool, TodoId::from_uuid(id))
        .await?
        .ok_or(AppError::NotFound)?;
    authorize(actor, action(todo.info().owner)).map_err(|_| AppError::NotFound)?;
    Ok(todo)
}
