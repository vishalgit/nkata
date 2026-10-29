use leptos::prelude::*;

use super::UserView;
use crate::error::AppError;

#[server(endpoint = "auth/register")]
pub async fn register(email: String, password: String) -> Result<UserView, AppError> {
    shell::handle(email, password)
        .await
        .map_err(crate::effects::fail)
}

#[cfg(feature = "ssr")]
mod shell {
    use chrono::Utc;
    use domain::identity::{RegisterUser, register_user};
    use domain::shared::UserId;
    use uuid::Uuid;

    use super::*;
    use crate::effects;
    use crate::features::auth::{crypto, store};

    pub async fn handle(email: String, password: String) -> Result<UserView, AppError> {
        // impure: gather
        let state = effects::state()?;
        let (id, now) = (UserId::from_uuid(Uuid::new_v4()), Utc::now());
        // pure: decide
        let user = register_user(
            RegisterUser { email, password },
            id,
            now,
            crypto::hash_password,
        )?;
        // impure: persist and respond
        store::insert_user(&state.pool, &user).await?;
        effects::start_session(&state, user.id, now).await?;
        Ok(UserView {
            email: user.email.as_str().to_owned(),
            is_admin: false,
        })
    }
}

use leptos_router::{components::A, hooks::use_navigate};

use super::context::use_auth;
use crate::ui::ErrorMessage;

#[component]
pub fn RegisterPage() -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();
    Effect::new(move |_| {
        if let Some(Ok(_)) = auth.register.value().get() {
            navigate("/", Default::default());
        }
    });
    let error = Signal::derive(move || auth.register.value().get().and_then(Result::err));

    view! {
        <section class="auth-card">
            <h1 class="auth-card__title">"Create an account"</h1>
            <ActionForm action=auth.register attr:class="form">
                <label class="form__field">
                    <span class="form__label">"Email"</span>
                    <input class="form__input" type="email" name="email" required autocomplete="email"/>
                </label>
                <label class="form__field">
                    <span class="form__label">"Password (12+ characters)"</span>
                    <input class="form__input" type="password" name="password" required minlength="12" autocomplete="new-password"/>
                </label>
                <ErrorMessage error/>
                <button class="button button--primary" type="submit">"Register"</button>
            </ActionForm>
            <p class="auth-card__alt">"Already registered? " <A href="/login">"Log in"</A></p>
        </section>
    }
}
