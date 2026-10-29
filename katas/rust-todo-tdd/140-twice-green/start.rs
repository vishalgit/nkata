use chrono::{DateTime, Utc};
use thiserror::Error;

use crate::todos::{CompletedTodo, Todo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoCompleted {
    pub todo: CompletedTodo,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CompleteTodoError {}

pub fn complete_todo(todo: Todo, now: DateTime<Utc>) -> Result<TodoCompleted, CompleteTodoError> {
    match todo {
        Todo::Active(active) => Ok(TodoCompleted {
            todo: active.complete(now),
        }),
        Todo::Completed(_) => todo!("completing twice"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::todos::workflows::fixtures::*;

    #[test]
    fn completing_an_active_todo_emits_completed() {
        let event = complete_todo(active("Buy milk"), at(11)).unwrap();
        assert_eq!(event.todo.completed_at(), at(11));
    }

    #[test]
    fn completing_twice_is_an_error() {
        assert_eq!(
            complete_todo(completed("Buy milk"), at(11)),
            Err(CompleteTodoError::AlreadyCompleted)
        );
    }
}
