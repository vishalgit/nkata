use leptos::prelude::*;

use super::login::Login;
use super::logout::Logout;
use super::register::Register;
use super::{UserView, me::me};
use crate::error::AppError;

#[derive(Clone, Copy)]
pub struct Auth {
    pub login: ServerAction<Login>,
    pub logout: ServerAction<Logout>,
    pub register: ServerAction<Register>,
    pub user: Resource<Result<Option<UserView>, AppError>>,
}

pub fn provide_auth() {
    let login = ServerAction::<Login>::new();
    let logout = ServerAction::<Logout>::new();
    let register = ServerAction::<Register>::new();
    let user = Resource::new(
        move || {
            (
                login.version().get(),
                register.version().get(),
                logout.version().get(),
            )
        },
        |_| me(),
    );
    provide_context(Auth {
        login,
        logout,
        register,
        user,
    });
}

pub fn use_auth() -> Auth {
    expect_context::<Auth>()
}
