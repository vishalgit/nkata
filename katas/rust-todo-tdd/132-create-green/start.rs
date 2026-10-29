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
