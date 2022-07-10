use std::sync::Arc;

use crate::infra;
use axum::{routing, Router};

pub struct DI {
    pub server: infra::Server,
    pub lambda: infra::Lambda,
}

impl DI {
    pub fn new() -> Self {
        let h = Arc::new(infra::Handler {});
        let r = Router::new().route("/", routing::get(|| async move { h.root().await }));
        DI {
            server: infra::Server { router: r.clone() },
            lambda: infra::Lambda { router: r },
        }
    }
}