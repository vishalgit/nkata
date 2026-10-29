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

    /// State transition: consumes the active todo.
    pub fn complete(self, at: DateTime<Utc>) -> CompletedTodo {
        CompletedTodo {
            info: self.0,
            completed_at: at,
        }
    }
}

/// A finished todo. Fields are private to protect `completed_at >= created_at`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletedTodo {
    info: TodoInfo,
    completed_at: DateTime<Utc>,
}

impl CompletedTodo {
    pub fn info(&self) -> &TodoInfo {
        &self.info
    }

    pub fn completed_at(&self) -> DateTime<Utc> {
        self.completed_at
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Todo {
    Active(ActiveTodo),
    Completed(CompletedTodo),
}

impl Todo {
    pub fn info(&self) -> &TodoInfo {
        match self {
            Todo::Active(t) => t.info(),
            Todo::Completed(t) => t.info(),
        }
    }

    pub fn is_completed(&self) -> bool {
        matches!(self, Todo::Completed(_))
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

    #[test]
    fn completing_records_when_it_happened() {
        let done = ActiveTodo::new(info()).complete(at(10));
        assert_eq!(done.completed_at(), at(10));
        assert_eq!(done.info(), &info());
    }
}
