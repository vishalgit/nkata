#![allow(dead_code)] // not every test file uses every helper

use std::time::Duration;

use app::state::AppState;
use axum::{
    Router,
    body::Body,
    http::{
        HeaderMap, Request, StatusCode,
        header::{ACCEPT, CONTENT_TYPE, COOKIE, SET_COOKIE},
    },
};
use http_body_util::BodyExt;
use leptos::prelude::LeptosOptions;
use serde::de::DeserializeOwned;
use sqlx::{PgPool, postgres::PgPoolOptions};
use tower::ServiceExt; // .oneshot()

pub const PASSWORD: &str = "correct horse battery";

pub struct TestApp {
    router: Router,
    pub pool: PgPool,
}

pub struct TestResponse {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: String,
}

impl TestResponse {
    pub fn json<T: DeserializeOwned>(&self) -> T {
        serde_json::from_str(&self.body).unwrap_or_else(|e| panic!("bad JSON ({e}): {}", self.body))
    }

    /// `todo_session=<token>` from Set-Cookie, ready to send back as a Cookie header.
    pub fn session_cookie(&self) -> Option<String> {
        self.headers
            .get_all(SET_COOKIE)
            .iter()
            .filter_map(|v| v.to_str().ok())
            .find(|c| c.starts_with("todo_session="))
            .and_then(|c| c.split(';').next())
            .map(str::to_owned)
    }
}

impl TestApp {
    pub fn new(pool: PgPool) -> Self {
        let leptos_options = LeptosOptions::builder().output_name("todo").build();
        let router = server::router(AppState {
            leptos_options,
            pool: pool.clone(),
        });
        Self { router, pool }
    }

    /// An app whose database can never be reached.
    pub fn without_database() -> Self {
        let pool = PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(1))
            .connect_lazy("postgres://nobody:nothing@127.0.0.1:1/none")
            .unwrap();
        Self::new(pool)
    }

    /// An app connected to the database in DATABASE_URL.
    pub async fn with_database() -> Self {
        let url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
        Self::new(PgPool::connect(&url).await.unwrap())
    }

    pub async fn get(&self, path: &str, cookie: Option<&str>) -> TestResponse {
        let mut request = Request::get(path).header(ACCEPT, "text/html"); // like a browser navigation
        if let Some(cookie) = cookie {
            request = request.header(COOKIE, cookie);
        }
        self.send(request.body(Body::empty()).unwrap()).await
    }

    pub async fn post(
        &self,
        path: &str,
        form: &[(&str, &str)],
        cookie: Option<&str>,
    ) -> TestResponse {
        let mut request =
            Request::post(path).header(CONTENT_TYPE, "application/x-www-form-urlencoded");
        if let Some(cookie) = cookie {
            request = request.header(COOKIE, cookie);
        }
        let body = Body::from(serde_urlencoded::to_string(form).unwrap());
        self.send(request.body(body).unwrap()).await
    }

    /// Registers a user and returns their session cookie.
    pub async fn register(&self, email: &str) -> String {
        let res = self
            .post(
                "/api/auth/register",
                &[("email", email), ("password", PASSWORD)],
                None,
            )
            .await;
        assert_eq!(res.status, StatusCode::OK, "register failed: {}", res.body);
        res.session_cookie()
            .expect("register must set a session cookie")
    }

    pub async fn make_admin(&self, email: &str) {
        sqlx::query("UPDATE users SET role = 'admin' WHERE email = $1")
            .bind(email)
            .execute(&self.pool)
            .await
            .unwrap();
    }

    pub async fn send(&self, request: Request<Body>) -> TestResponse {
        let response = self.router.clone().oneshot(request).await.unwrap();
        let status = response.status();
        let headers = response.headers().clone();
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        TestResponse {
            status,
            headers,
            body: String::from_utf8(bytes.to_vec()).unwrap(),
        }
    }
}

/// Scrapes the metrics router, like Prometheus would.
pub async fn scrape_metrics() -> String {
    let response = server::metrics_router()
        .oneshot(Request::get("/metrics").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    String::from_utf8(bytes.to_vec()).unwrap()
}
