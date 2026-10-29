use chrono::{DateTime, Utc};
use thiserror::Error;

use crate::shared::{TodoId, UserId};
use crate::todos::{ActiveTodo, TodoInfo, TodoTitle};

/// Command: raw input from the outside world. Nothing validated yet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateTodo {
    pub owner: UserId,
    pub title: String,
}

/// Event: a fact, in the past tense.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoCreated {
    pub todo: ActiveTodo,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CreateTodoError {}

/// `id` and `now` are dependencies, passed in as plain values.
pub fn create_todo(
    cmd: CreateTodo,
    id: TodoId,
    now: DateTime<Utc>,
) -> Result<TodoCreated, CreateTodoError> {
    let title = TodoTitle::parse(&cmd.title).expect("next test handles bad titles");
    let info = TodoInfo {
        id,
        owner: cmd.owner,
        title,
        created_at: now,
    };
    Ok(TodoCreated {
        todo: ActiveTodo::new(info),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::todos::workflows::fixtures::*;

    fn cmd(title: &str) -> CreateTodo {
        CreateTodo {
            owner: alice(),
            title: title.to_owned(),
        }
    }

    #[test]
    fn a_valid_command_produces_a_created_event() {
        let event = create_todo(cmd("Buy milk"), todo_id(), at(9)).unwrap();
        let info = event.todo.info();
        assert_eq!(info.id, todo_id());
        assert_eq!(info.owner, alice());
        assert_eq!(info.title.as_str(), "Buy milk");
        assert_eq!(info.created_at, at(9));
    }
}
