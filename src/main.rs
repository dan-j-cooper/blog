use anyhow::Context;
use anyhow::Result;
use askama::Template;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};


use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(
                    |_| "blog=debug".into()),
                )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Initializing router...");

    let router = Router::new().route("/", get(hello));
    let port = 8000_u16;
    let addr = std::net::SocketAddr::from(([0,0,0,0], port));

    info!("router initialized now listening on port {}", port);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .context("error while connecting to tcp port.")?;

    axum::serve(listener, router)
        .await
        .context("error while initalizing server.")?;

    Ok(())
}

async fn hello() -> impl IntoResponse {
    let template = HelloTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate;

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            ).into_response()
        }
    }
}
