use chrono::{DateTime, Utc};
use domain::shared::{TodoId, UserId};
use domain::todos::{ActiveTodo, Paged, Todo, TodoInfo, TodoQuery, TodoTitle};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error(transparent)]
    Db(#[from] sqlx::Error),
    #[error("row {id} is corrupt: {reason}")]
    Corrupt { id: Uuid, reason: String },
}

/// Persistence DTO: one row of the `todos` table.
struct TodoRow {
    id: Uuid,
    owner_id: Uuid,
    title: String,
    created_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
}

impl TodoRow {
    fn into_domain(self) -> Result<Todo, StoreError> {
        let title = TodoTitle::parse(&self.title).map_err(|e| StoreError::Corrupt {
            id: self.id,
            reason: e.to_string(),
        })?;
        let active = ActiveTodo::new(TodoInfo {
            id: TodoId::from_uuid(self.id),
            owner: UserId::from_uuid(self.owner_id),
            title,
            created_at: self.created_at,
        });
        Ok(match self.completed_at {
            None => Todo::Active(active),
            Some(at) => Todo::Completed(active.complete(at)),
        })
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

fn completed_at(todo: &Todo) -> Option<DateTime<Utc>> {
    match todo {
        Todo::Active(_) => None,
        Todo::Completed(done) => Some(done.completed_at()),
    }
}

pub async fn save(pool: &PgPool, todo: &Todo) -> Result<(), StoreError> {
    let info = todo.info();
    sqlx::query!(
        "UPDATE todos SET title = $2, completed_at = $3 WHERE id = $1",
        info.id.into_uuid(),
        info.title.as_str(),
        completed_at(todo),
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Escapes LIKE wildcards so user input matches literally (backslash is the default escape).
pub fn escape_like(raw: &str) -> String {
    raw.replace('\\', r"\\")
        .replace('%', r"\%")
        .replace('_', r"\_")
}

pub async fn search_for_owner(
    pool: &PgPool,
    owner: UserId,
    query: &TodoQuery,
) -> Result<Paged<Todo>, StoreError> {
    let pattern = query
        .search
        .as_ref()
        .map(|s| format!("%{}%", escape_like(s.as_str())));

    let total = sqlx::query_scalar!(
        r#"SELECT COUNT(*) AS "count!" FROM todos
           WHERE owner_id = $1 AND ($2::text IS NULL OR title ILIKE $2)"#,
        owner.into_uuid(),
        pattern.as_deref(),
    )
    .fetch_one(pool)
    .await?;

    let items = sqlx::query_as!(
        TodoRow,
        r#"SELECT id, owner_id, title, created_at, completed_at FROM todos
           WHERE owner_id = $1 AND ($2::text IS NULL OR title ILIKE $2)
           ORDER BY created_at DESC, id"#,
        owner.into_uuid(),
        pattern.as_deref(),
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(TodoRow::into_domain)
    .collect::<Result<Vec<_>, _>>()?;

    Ok(Paged {
        items,
        page: query.page,
        total: total as u64,
    })
}

/// Returns true if a row was deleted.
pub async fn delete(pool: &PgPool, id: TodoId) -> Result<bool, StoreError> {
    let result = sqlx::query!("DELETE FROM todos WHERE id = $1", id.into_uuid())
        .execute(pool)
        .await?;
    Ok(result.rows_affected() == 1)
}

pub async fn find(pool: &PgPool, id: TodoId) -> Result<Option<Todo>, StoreError> {
    sqlx::query_as!(
        TodoRow,
        "SELECT id, owner_id, title, created_at, completed_at FROM todos WHERE id = $1",
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

    #[sqlx::test(migrations = "../../migrations")]
    async fn an_unknown_id_is_not_found(pool: PgPool) {
        let id = TodoId::from_uuid(Uuid::new_v4());
        assert_eq!(find(&pool, id).await.unwrap(), None);
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn a_row_the_domain_rejects_is_reported_as_corrupt(pool: PgPool) {
        // three spaces pass the SQL CHECK (length 3) but not TodoTitle::parse
        let id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO todos (id, owner_id, title, created_at) VALUES ($1, $2, '   ', now())",
        )
        .bind(id)
        .bind(user(1).into_uuid())
        .execute(&pool)
        .await
        .unwrap();

        let err = find(&pool, TodoId::from_uuid(id)).await.unwrap_err();
        assert!(matches!(err, StoreError::Corrupt { .. }), "got {err:?}");
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn the_database_refuses_an_overlong_title(pool: PgPool) {
        let result = sqlx::query(
            "INSERT INTO todos (id, owner_id, title, created_at) VALUES ($1, $2, $3, now())",
        )
        .bind(Uuid::new_v4())
        .bind(user(1).into_uuid())
        .bind("x".repeat(121))
        .execute(&pool)
        .await;
        assert!(
            result.is_err(),
            "CHECK constraint should reject 121 characters"
        );
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn saving_persists_completion_and_title(pool: PgPool) {
        let todo = new_todo(user(1), "Buy milk", 9);
        insert(&pool, &todo).await.unwrap();

        let done_at = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
        let done = Todo::Completed(domain::todos::complete_todo(todo, done_at).unwrap().todo);
        let renamed = domain::todos::rename_todo(done, "Buy oat milk")
            .unwrap()
            .todo;
        save(&pool, &renamed).await.unwrap();

        assert_eq!(find(&pool, renamed.info().id).await.unwrap(), Some(renamed));
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn deleting_removes_the_row_once(pool: PgPool) {
        let todo = new_todo(user(1), "Buy milk", 9);
        insert(&pool, &todo).await.unwrap();

        assert!(delete(&pool, todo.info().id).await.unwrap());
        assert!(!delete(&pool, todo.info().id).await.unwrap());
    }

    #[test]
    fn like_wildcards_in_user_input_are_escaped() {
        assert_eq!(escape_like(r"50% off_now\"), r"50\% off\_now\\");
    }

    fn query(filter: Option<&str>, q: Option<&str>, page: Option<u32>) -> domain::todos::TodoQuery {
        domain::todos::TodoQuery::parse(filter, q, page).unwrap()
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn search_matches_part_of_a_title_for_one_owner_newest_first(pool: PgPool) {
        for (owner, title, hour) in [
            (user(1), "Buy MILK", 9),
            (user(1), "Walk the dog", 10),
            (user(1), "Oat milk recipe", 11),
            (user(2), "Not my milk", 12),
        ] {
            insert(&pool, &new_todo(owner, title, hour)).await.unwrap();
        }

        let found = search_for_owner(&pool, user(1), &query(None, Some("milk"), None))
            .await
            .unwrap();
        let titles: Vec<&str> = found
            .items
            .iter()
            .map(|t| t.info().title.as_str())
            .collect();

        assert_eq!(
            titles,
            vec!["Oat milk recipe", "Buy MILK"],
            "case-insensitive, owner only, newest first"
        );
        assert_eq!(found.total, 2);
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn wildcards_typed_by_the_user_match_literally(pool: PgPool) {
        insert(&pool, &new_todo(user(1), "Save 50% on milk", 9))
            .await
            .unwrap();
        insert(&pool, &new_todo(user(1), "Buy 500 g of flour", 10))
            .await
            .unwrap();

        let found = search_for_owner(&pool, user(1), &query(None, Some("50%"), None))
            .await
            .unwrap();
        assert_eq!(
            found.total, 1,
            "'50%' must not behave like '50' followed by anything"
        );
    }
}
