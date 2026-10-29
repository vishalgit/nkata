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
