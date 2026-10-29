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
            completed_at: at.max(self.0.created_at),
            info: self.0,
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

    /// State transition: consumes the completed todo.
    pub fn reopen(self) -> ActiveTodo {
        ActiveTodo(self.info)
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

    pub fn with_title(self, title: TodoTitle) -> Todo {
        match self {
            Todo::Active(ActiveTodo(mut info)) => {
                info.title = title;
                Todo::Active(ActiveTodo(info))
            }
            Todo::Completed(CompletedTodo {
                mut info,
                completed_at,
            }) => {
                info.title = title;
                Todo::Completed(CompletedTodo { info, completed_at })
            }
        }
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

    #[test]
    fn completion_never_precedes_creation() {
        // a skewed clock must not finish a todo before it was created
        let done = ActiveTodo::new(info()).complete(at(8));
        assert_eq!(done.completed_at(), at(9));
    }

    #[test]
    fn reopening_gives_back_an_active_todo_with_the_same_info() {
        let active = ActiveTodo::new(info()).complete(at(10)).reopen();
        assert_eq!(active.info(), &info());
    }

    #[test]
    fn retitling_keeps_the_state() {
        let new_title = TodoTitle::parse("Write more tests").unwrap();
        let todo = Todo::Completed(ActiveTodo::new(info()).complete(at(10)));
        let renamed = todo.with_title(new_title.clone());
        assert!(renamed.is_completed());
        assert_eq!(renamed.info().title, new_title);
    }
}
