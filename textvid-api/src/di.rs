use crate::infra;
use axum::{routing, Router};

pub struct DI {
    pub server: infra::Server,
}

impl DI {
    pub fn new() -> Self {
        let _h = infra::Handler {};
        let r = Router::new().route("/", routing::get(|| async { "hello" }));
        let s = infra::Server { router: r };
        DI { server: s }
    }
}
