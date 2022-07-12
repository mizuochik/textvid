use std::sync::Arc;

use crate::infra;
use axum::{routing, Router};

pub struct Di {}

impl Di {
    pub fn new() -> Self {
        Di {}
    }

    pub fn server(&self) -> infra::Server {
        infra::Server {
            router: self.router(),
        }
    }

    pub fn lambda(&self) -> infra::Lambda {
        infra::Lambda {
            router: self.router(),
        }
    }

    fn handler(&self) -> infra::Handler {
        infra::Handler {}
    }

    fn router(&self) -> Router {
        let h = Arc::new(self.handler());
        Router::new().route("/", routing::get(|| async move { h.root().await }))
    }
}
