use chrono::{DateTime, Utc};

use super::TodoTitle;
use crate::shared::{TodoId, UserId};

/// Data every todo has, whatever its state.
/// Fields are public because each one is already a constrained type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoInfo {
    pub id: TodoId,
    pub owner: UserId,
    pub title: TodoTitle,
    pub created_at: DateTime<Utc>,
}

/// A todo that still needs doing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActiveTodo(TodoInfo);

impl ActiveTodo {
    pub fn new(info: TodoInfo) -> Self {
        Self(info)
    }

    pub fn info(&self) -> &TodoInfo {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Todo {
    Active(ActiveTodo),
}

impl Todo {
    pub fn info(&self) -> &TodoInfo {
        match self {
            Todo::Active(t) => t.info(),
        }
    }

    pub fn is_completed(&self) -> bool {
        false // the only state so far
    }
}

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
