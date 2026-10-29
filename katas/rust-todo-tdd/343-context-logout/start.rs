use leptos::prelude::*;

use super::login::Login;
use super::{UserView, me::me};
use crate::error::AppError;

#[derive(Clone, Copy)]
pub struct Auth {
    pub login: ServerAction<Login>,
    pub user: Resource<Result<Option<UserView>, AppError>>,
}

pub fn provide_auth() {
    let login = ServerAction::<Login>::new();
    let user = Resource::new(move || login.version().get(), |_| me());
    provide_context(Auth { login, user });
}

pub fn use_auth() -> Auth {
    expect_context::<Auth>()
}
