//! Effects used by server functions: state, clock-dependent sessions, cookies, HTTP status.

use axum::http::{
    HeaderMap, HeaderValue,
    header::{COOKIE, SET_COOKIE},
};
use chrono::{DateTime, Duration, Utc};
use domain::shared::UserId;
use leptos::{config::Env, prelude::*};
use leptos_axum::ResponseOptions;

use crate::error::AppError;
use crate::features::auth::store::SessionUser;
use crate::features::auth::{cookie, crypto, store};
use crate::state::AppState;

pub fn state() -> Result<AppState, AppError> {
    use_context::<AppState>().ok_or_else(|| {
        tracing::error!("AppState missing from context");
        AppError::Unexpected
    })
}

/// Every server function ends with `.map_err(effects::fail)`:
/// it sets the HTTP status matching the error and passes the error on.
pub fn fail(err: AppError) -> AppError {
    if let Some(response) = use_context::<ResponseOptions>() {
        response.set_status(err.status());
    }
    err
}

fn set_cookie(value: String) {
    if let (Some(response), Ok(value)) = (
        use_context::<ResponseOptions>(),
        HeaderValue::from_str(&value),
    ) {
        response.append_header(SET_COOKIE, value);
    }
}

pub async fn start_session(
    state: &AppState,
    user: UserId,
    now: DateTime<Utc>,
) -> Result<(), AppError> {
    let token = crypto::generate_session_token();
    let expires_at = now + Duration::days(cookie::SESSION_TTL_DAYS);
    store::create_session(
        &state.pool,
        &crypto::hash_session_token(&token),
        user,
        now,
        expires_at,
    )
    .await?;
    set_cookie(cookie::session_cookie(
        &token,
        state.leptos_options.env == Env::PROD,
    ));
    Ok(())
}

pub async fn session_token() -> Result<Option<String>, AppError> {
    let headers: HeaderMap = leptos_axum::extract()
        .await
        .map_err(|_| AppError::Unexpected)?;
    Ok(headers
        .get(COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(cookie::read_session_token)
        .map(str::to_owned))
}

pub async fn current_user() -> Result<Option<SessionUser>, AppError> {
    let Some(token) = session_token().await? else {
        return Ok(None);
    };
    let state = state()?;
    Ok(
        store::find_session_user(&state.pool, &crypto::hash_session_token(&token), Utc::now())
            .await?,
    )
}
