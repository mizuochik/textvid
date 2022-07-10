use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::env;
use std::future::Future;
use std::{convert::Infallible, sync::Arc};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let h = Handler {};
    if env::var("AWS_LAMBDA_RUNTIME_API").is_ok() {
        run_lambda(handler).await?;
    } else {
        run_server(handler, h).await?;
    }
    Ok(())
}

async fn run_lambda<F, R, T>(f: F) -> anyhow::Result<()>
where
    R: lambda_http::IntoResponse,
    F: Fn(lambda_http::Request) -> T,
    T: Future<Output = Result<R, lambda_http::Error>>,
{
    lambda_http::run(lambda_http::service_fn(f))
        .await
        .map_err(|e| anyhow::anyhow!("lambda: {}", e))
}

async fn run_server(h: Handler) -> anyhow::Result<()> {
    let addr = ([127, 0, 0, 1], 3000).into();
    let make_svc = make_service_fn(|_| async {
        Ok::<_, Infallible>(service_fn(|_: Request<Body>| async {
            let req = Request::builder().body(lambda_http::Body::Empty)?;
            h.handle(req);
            let res = Response::builder()
                .status(200)
                .body(Body::from("Hello Textvid"))?;
            Ok::<_, anyhow::Error>(res)
        }))
    });
    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}

pub struct Handler {}

impl Handler {
    pub async fn handle(
        &self,
        _req: lambda_http::Request,
    ) -> Result<impl lambda_http::IntoResponse, lambda_http::Error> {
        Ok(lambda_http::Response::builder()
            .status(200)
            .body("hoge")
            .unwrap())
    }
}

async fn handler(
    _req: lambda_http::Request,
) -> Result<impl lambda_http::IntoResponse, lambda_http::Error> {
    Ok(lambda_http::Response::builder()
        .status(200)
        .body("hoge")
        .unwrap())
}
