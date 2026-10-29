use domain::todos::{Paged, Todo};
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoPage {
    pub items: Vec<TodoView>,
    pub page: u32,
    pub total_pages: u32,
    pub total: u64,
}

impl From<Paged<Todo>> for TodoPage {
    fn from(paged: Paged<Todo>) -> Self {
        TodoPage {
            page: paged.page.number(),
            total_pages: paged.total_pages(),
            total: paged.total,
            items: paged.items.iter().map(TodoView::from).collect(),
        }
    }
}
