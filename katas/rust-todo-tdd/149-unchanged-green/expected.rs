use thiserror::Error;

use crate::todos::{TitleError, Todo, TodoTitle};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoRenamed {
    pub todo: Todo,
    pub old_title: TodoTitle,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum RenameTodoError {
    #[error(transparent)]
    InvalidTitle(#[from] TitleError),
    #[error("the new title is the same as the old one")]
    Unchanged,
}

pub fn rename_todo(todo: Todo, new_title: &str) -> Result<TodoRenamed, RenameTodoError> {
    TodoTitle::parse(new_title)
        .map_err(RenameTodoError::from)
        .and_then(|title| ensure_changed(&todo, title))
        .map(|title| apply(todo, title))
}

fn ensure_changed(todo: &Todo, title: TodoTitle) -> Result<TodoTitle, RenameTodoError> {
    if todo.info().title == title {
        Err(RenameTodoError::Unchanged)
    } else {
        Ok(title)
    }
}

fn apply(todo: Todo, title: TodoTitle) -> TodoRenamed {
    let old_title = todo.info().title.clone();
    TodoRenamed {
        todo: todo.with_title(title),
        old_title,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::todos::workflows::fixtures::*;

    #[test]
    fn renaming_emits_the_new_todo_and_the_old_title() {
        let event = rename_todo(active("Buy milk"), "Buy oat milk").unwrap();
        assert_eq!(event.todo.info().title.as_str(), "Buy oat milk");
        assert_eq!(event.old_title.as_str(), "Buy milk");
    }

    #[test]
    fn renaming_keeps_a_completed_todo_completed() {
        let event = rename_todo(completed("Buy milk"), "Buy oat milk").unwrap();
        assert!(event.todo.is_completed());
    }

    #[test]
    fn an_invalid_title_is_rejected() {
        assert_eq!(
            rename_todo(active("Buy milk"), ""),
            Err(RenameTodoError::InvalidTitle(TitleError::Empty))
        );
    }

    #[test]
    fn the_same_title_after_trimming_is_unchanged() {
        assert_eq!(
            rename_todo(active("Buy milk"), "  Buy milk "),
            Err(RenameTodoError::Unchanged)
        );
    }
}
