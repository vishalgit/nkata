#[cfg(test)]
mod tests {
    use super::*;
    use crate::todos::workflows::fixtures::*;

    #[test]
    fn completing_an_active_todo_emits_completed() {
        let event = complete_todo(active("Buy milk"), at(11)).unwrap();
        assert_eq!(event.todo.completed_at(), at(11));
    }
}
