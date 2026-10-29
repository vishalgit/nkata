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

// Infrastructure errors: details are logged, never sent to the browser.
#[cfg(feature = "ssr")]
impl From<crate::features::auth::store::AuthStoreError> for AppError {
    fn from(e: crate::features::auth::store::AuthStoreError) -> Self {
        tracing::error!(error = %e, "auth store failure");
        AppError::Unexpected
    }
}
