#[cfg(feature = "ssr")]
pub mod cookie;
#[cfg(feature = "ssr")]
pub mod crypto;
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
