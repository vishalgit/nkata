use leptos::prelude::*;

use super::{
    complete::CompleteTodo, create::CreateTodo, delete::DeleteTodo, rename::RenameTodo,
    reopen::ReopenTodo,
};
use crate::error::AppError;

#[derive(Clone, Copy)]
pub struct TodoActions {
    pub create: ServerAction<CreateTodo>,
    pub complete: ServerAction<CompleteTodo>,
    pub reopen: ServerAction<ReopenTodo>,
    pub rename: ServerAction<RenameTodo>,
    pub delete: ServerAction<DeleteTodo>,
}

impl TodoActions {
    pub fn provide() -> Self {
        let actions = TodoActions {
            create: ServerAction::new(),
            complete: ServerAction::new(),
            reopen: ServerAction::new(),
            rename: ServerAction::new(),
            delete: ServerAction::new(),
        };
        provide_context(actions);
        actions
    }

    /// Changes whenever any action completes: use it as a Resource source.
    pub fn versions(&self) -> Vec<usize> {
        vec![
            self.create.version().get(),
            self.complete.version().get(),
            self.reopen.version().get(),
            self.rename.version().get(),
            self.delete.version().get(),
        ]
    }

    /// The first error any action is currently showing.
    pub fn error(&self) -> Option<AppError> {
        [
            self.create.value().get().and_then(Result::err),
            self.complete.value().get().and_then(Result::err),
            self.reopen.value().get().and_then(Result::err),
            self.rename.value().get().and_then(Result::err),
            self.delete.value().get().and_then(Result::err),
        ]
        .into_iter()
        .flatten()
        .next()
    }
}

pub fn use_todo_actions() -> TodoActions {
    expect_context::<TodoActions>()
}
