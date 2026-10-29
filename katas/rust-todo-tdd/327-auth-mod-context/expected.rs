pub mod context;
#[cfg(feature = "ssr")]
pub mod cookie;
#[cfg(feature = "ssr")]
pub mod crypto;
pub mod login;
pub mod logout;
pub mod me;
pub mod register;
#[cfg(feature = "ssr")]
pub mod store;

use serde::{Deserialize, Serialize};

/// What the browser may know about the logged-in user.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserView {
    pub email: String,
    pub is_admin: bool,
}

#[cfg(feature = "ssr")]
impl From<store::SessionUser> for UserView {
    fn from(user: store::SessionUser) -> Self {
        UserView {
            email: user.email.as_str().to_owned(),
            is_admin: user.actor.role == domain::identity::Role::Admin,
        }
    }
}
