use chrono::{DateTime, Utc};
use domain::shared::{TodoId, UserId};
use domain::todos::{ActiveTodo, Todo, TodoInfo, TodoTitle};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error(transparent)]
    Db(#[from] sqlx::Error),
}

/// Persistence DTO: one row of the `todos` table.
struct TodoRow {
    id: Uuid,
    owner_id: Uuid,
    title: String,
    created_at: DateTime<Utc>,
}

impl TodoRow {
    fn into_domain(self) -> Result<Todo, StoreError> {
        let title = TodoTitle::parse(&self.title).expect("the database holds valid titles");
        Ok(Todo::Active(ActiveTodo::new(TodoInfo {
            id: TodoId::from_uuid(self.id),
            owner: UserId::from_uuid(self.owner_id),
            title,
            created_at: self.created_at,
        })))
    }
}

pub async fn insert(pool: &PgPool, todo: &Todo) -> Result<(), StoreError> {
    let info = todo.info();
    sqlx::query!(
        "INSERT INTO todos (id, owner_id, title, created_at) VALUES ($1, $2, $3, $4)",
        info.id.into_uuid(),
        info.owner.into_uuid(),
        info.title.as_str(),
        info.created_at,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn find(pool: &PgPool, id: TodoId) -> Result<Option<Todo>, StoreError> {
    sqlx::query_as!(
        TodoRow,
        "SELECT id, owner_id, title, created_at FROM todos WHERE id = $1",
        id.into_uuid(),
    )
    .fetch_optional(pool)
    .await?
    .map(TodoRow::into_domain)
    .transpose()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use domain::todos::{CreateTodo, create_todo};

    fn user(n: u128) -> UserId {
        UserId::from_uuid(Uuid::from_u128(n))
    }

    fn new_todo(owner: UserId, title: &str, hour: u32) -> Todo {
        let id = TodoId::from_uuid(Uuid::new_v4());
        let now = Utc.with_ymd_and_hms(2026, 1, 1, hour, 0, 0).unwrap();
        let cmd = CreateTodo {
            owner,
            title: title.to_owned(),
        };
        Todo::Active(create_todo(cmd, id, now).unwrap().todo)
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn an_inserted_todo_can_be_found_again(pool: PgPool) {
        let todo = new_todo(user(1), "Buy milk", 9);
        insert(&pool, &todo).await.unwrap();

        assert_eq!(find(&pool, todo.info().id).await.unwrap(), Some(todo));
    }
}
