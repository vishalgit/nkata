#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use uuid::Uuid;

    fn at(hour: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 1, 1, hour, 0, 0).unwrap()
    }

    fn info() -> TodoInfo {
        TodoInfo {
            id: TodoId::from_uuid(Uuid::from_u128(1)),
            owner: UserId::from_uuid(Uuid::from_u128(2)),
            title: TodoTitle::parse("Write tests").unwrap(),
            created_at: at(9),
        }
    }

    #[test]
    fn a_new_todo_is_active() {
        let todo = Todo::Active(ActiveTodo::new(info()));
        assert!(!todo.is_completed());
        assert_eq!(todo.info(), &info());
    }
}
