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
}
