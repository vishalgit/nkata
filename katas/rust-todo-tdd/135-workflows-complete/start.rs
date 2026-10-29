mod create;

pub use create::{CreateTodo, CreateTodoError, TodoCreated, create_todo};

#[cfg(test)]
pub(crate) mod fixtures {
    use chrono::{DateTime, TimeZone, Utc};
    use uuid::Uuid;

    use crate::shared::{TodoId, UserId};

    pub fn at(hour: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 1, 1, hour, 0, 0).unwrap()
    }
    pub fn todo_id() -> TodoId {
        TodoId::from_uuid(Uuid::from_u128(1))
    }
    pub fn alice() -> UserId {
        UserId::from_uuid(Uuid::from_u128(100))
    }
}
