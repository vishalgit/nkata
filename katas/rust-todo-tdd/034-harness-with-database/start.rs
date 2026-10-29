#![allow(dead_code)] // not every test file uses every helper

use std::time::Duration;

use app::state::AppState;
use axum::{
    Router,
    body::Body,
    http::{HeaderMap, Request, StatusCode, header::COOKIE},
};
use http_body_util::BodyExt;
use leptos::prelude::LeptosOptions;
use sqlx::{PgPool, postgres::PgPoolOptions};
use tower::ServiceExt; // .oneshot()

pub struct TestApp {
    router: Router,
    pub pool: PgPool,
}

pub struct TestResponse {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: String,
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

    pub async fn get(&self, path: &str, cookie: Option<&str>) -> TestResponse {
        let mut request = Request::get(path);
        if let Some(cookie) = cookie {
            request = request.header(COOKIE, cookie);
        }
        self.send(request.body(Body::empty()).unwrap()).await
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
