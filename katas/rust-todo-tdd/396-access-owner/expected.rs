use domain::identity::Actor;
use domain::shared::TodoId;
use domain::todos::Todo;
use sqlx::PgPool;
use uuid::Uuid;

use super::store;
use crate::error::AppError;

pub async fn load_for(actor: &Actor, pool: &PgPool, id: Uuid) -> Result<Todo, AppError> {
    let todo = store::find(pool, TodoId::from_uuid(id))
        .await?
        .ok_or(AppError::NotFound)?;
    if todo.info().owner != actor.id {
        return Err(AppError::NotFound); // Chapter 10 replaces this with a policy
    }
    Ok(todo)
}
