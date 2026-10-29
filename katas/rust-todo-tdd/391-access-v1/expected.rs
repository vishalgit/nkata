use domain::identity::Actor;
use domain::shared::TodoId;
use domain::todos::Todo;
use sqlx::PgPool;
use uuid::Uuid;

use super::store;
use crate::error::AppError;

pub async fn load_for(actor: &Actor, pool: &PgPool, id: Uuid) -> Result<Todo, AppError> {
    let _ = actor; // test 2 uses it
    store::find(pool, TodoId::from_uuid(id))
        .await?
        .ok_or(AppError::NotFound)
}
