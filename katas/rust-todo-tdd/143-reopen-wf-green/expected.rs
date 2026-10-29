use thiserror::Error;

use crate::todos::{ActiveTodo, Todo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoReopened {
    pub todo: ActiveTodo,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ReopenTodoError {
    #[error("todo is not completed")]
    NotCompleted,
}

pub fn reopen_todo(todo: Todo) -> Result<TodoReopened, ReopenTodoError> {
    match todo {
        Todo::Completed(done) => Ok(TodoReopened {
            todo: done.reopen(),
        }),
        Todo::Active(_) => Err(ReopenTodoError::NotCompleted),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::todos::workflows::fixtures::*;

    #[test]
    fn reopening_a_completed_todo_emits_reopened() {
        let event = reopen_todo(completed("Buy milk")).unwrap();
        assert_eq!(event.todo.info().title.as_str(), "Buy milk");
    }

    #[test]
    fn reopening_an_active_todo_is_an_error() {
        assert_eq!(
            reopen_todo(active("Buy milk")),
            Err(ReopenTodoError::NotCompleted)
        );
    }
}
