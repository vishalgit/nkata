use leptos::prelude::*;

use super::{UserView, me::me};
use crate::error::AppError;

#[derive(Clone, Copy)]
pub struct Auth {
    pub user: Resource<Result<Option<UserView>, AppError>>,
}

pub fn provide_auth() {
    let user = Resource::new(|| (), |_| me());
    provide_context(Auth { user });
}

pub fn use_auth() -> Auth {
    expect_context::<Auth>()
}
