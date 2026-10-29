mod authenticate;
mod email;
mod password;
mod policy;
mod register;
mod role;

pub use authenticate::{LoginError, StoredCredentials, authenticate};
pub use email::{EmailAddress, EmailError};
pub use password::{Password, PasswordError};
pub use policy::{Action, Forbidden, authorize};
pub use register::{
    HashingFailed, PasswordHash, RegisterError, RegisterUser, RegistrationProblem, UserRegistered,
    register_user,
};
pub use role::{Actor, Role};
