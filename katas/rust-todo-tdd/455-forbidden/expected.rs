use leptos::server_fn::{
    codec::JsonEncoding,
    error::{FromServerFnError, ServerFnErrorErr},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
pub enum AppError {
    #[error("something went wrong, please try again")]
    Unexpected,
    #[error("request failed: {0}")]
    Transport(String),
    #[error("you are not allowed to do that")]
    Forbidden,
    #[error("not found")]
    NotFound,
    #[error("please log in")]
    Unauthenticated,
    #[error("invalid email or password")]
    InvalidCredentials,
    #[error("{0}")]
    Conflict(String),
    #[error("{}", .0.join("; "))]
    Validation(Vec<String>),
}

/// Lets #[server] functions return Result<T, AppError> directly.
impl FromServerFnError for AppError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        AppError::Transport(value.to_string())
    }
}

#[cfg(feature = "ssr")]
impl AppError {
    pub fn status(&self) -> axum::http::StatusCode {
        use axum::http::StatusCode as S;
        match self {
            AppError::Transport(_) => S::BAD_REQUEST,
            AppError::Validation(_) => S::UNPROCESSABLE_ENTITY,
            AppError::Conflict(_) => S::CONFLICT,
            AppError::InvalidCredentials => S::UNAUTHORIZED,
            AppError::Unauthenticated => S::UNAUTHORIZED,
            AppError::NotFound => S::NOT_FOUND,
            AppError::Forbidden => S::FORBIDDEN,
            AppError::Unexpected => S::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<domain::identity::RegisterError> for AppError {
    fn from(e: domain::identity::RegisterError) -> Self {
        use domain::identity::RegisterError as E;
        match e {
            E::Invalid(problems) => {
                AppError::Validation(problems.iter().map(|p| p.to_string()).collect())
            }
            E::Hashing => AppError::Unexpected,
        }
    }
}

impl From<domain::identity::LoginError> for AppError {
    fn from(_: domain::identity::LoginError) -> Self {
        AppError::InvalidCredentials
    }
}

impl From<domain::todos::CreateTodoError> for AppError {
    fn from(e: domain::todos::CreateTodoError) -> Self {
        AppError::Validation(vec![e.to_string()])
    }
}

impl From<domain::todos::SearchError> for AppError {
    fn from(e: domain::todos::SearchError) -> Self {
        AppError::Validation(vec![e.to_string()])
    }
}

impl From<domain::todos::CompleteTodoError> for AppError {
    fn from(e: domain::todos::CompleteTodoError) -> Self {
        AppError::Conflict(e.to_string())
    }
}

impl From<domain::todos::ReopenTodoError> for AppError {
    fn from(e: domain::todos::ReopenTodoError) -> Self {
        AppError::Conflict(e.to_string())
    }
}

impl From<domain::todos::RenameTodoError> for AppError {
    fn from(e: domain::todos::RenameTodoError) -> Self {
        AppError::Validation(vec![e.to_string()])
    }
}

impl From<domain::identity::Forbidden> for AppError {
    fn from(_: domain::identity::Forbidden) -> Self {
        AppError::Forbidden
    }
}

// Infrastructure errors: details are logged, never sent to the browser.
#[cfg(feature = "ssr")]
impl From<crate::features::auth::store::AuthStoreError> for AppError {
    fn from(e: crate::features::auth::store::AuthStoreError) -> Self {
        use crate::features::auth::store::AuthStoreError as E;
        match e {
            E::EmailTaken => AppError::Conflict("that email is already registered".into()),
            other => {
                tracing::error!(error = %other, "auth store failure");
                AppError::Unexpected
            }
        }
    }
}

#[cfg(feature = "ssr")]
impl From<crate::features::todos::store::StoreError> for AppError {
    fn from(e: crate::features::todos::store::StoreError) -> Self {
        tracing::error!(error = %e, "todo store failure");
        AppError::Unexpected
    }
}
