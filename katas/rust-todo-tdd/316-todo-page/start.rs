use domain::todos::Todo;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoView {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
}

impl From<&Todo> for TodoView {
    fn from(todo: &Todo) -> Self {
        let info = todo.info();
        TodoView {
            id: info.id.into_uuid(),
            title: info.title.as_str().to_owned(),
            completed: todo.is_completed(),
        }
    }
}
