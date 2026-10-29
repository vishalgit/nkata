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
            AppError::Unexpected => S::INTERNAL_SERVER_ERROR,
        }
    }
}

// Refined by test 2.
impl From<domain::identity::RegisterError> for AppError {
    fn from(_: domain::identity::RegisterError) -> Self {
        AppError::Unexpected
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
