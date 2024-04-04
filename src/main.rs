use anyhow::Context;
use anyhow::Result;
use askama::Template;
use tower_livereload::LiveReloadLayer;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};

use tower_http::services::ServeDir;

use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(
                    |_| "blog=debug".into()),
                )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing();
    info!("Initializing router...");
    let router = Router::new()
        .route("/", get(root))
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(LiveReloadLayer::new());


    let port = 8000;
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .context("error while connecting to tcp port.")?;

    info!("router initialized now listening on port {}", port);

    axum::serve(listener, router)
        .await
        .context("error while initalizing server.")?;

    Ok(())
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate;

async fn root() -> impl IntoResponse {
    info!("getting root");
    HelloTemplate{}.into_response()
}


// struct HtmlTemplate<T>(T);

// impl<T> IntoResponse for HtmlTemplate<T>
// where
//     T: Template,
// {
//     fn into_response(self) -> Response {
//         match self.0.render() {
//             Ok(html) => Html(html).into_response(),
//             Err(err) => (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 format!("Failed to render template. Error: {}", err),
//             ).into_response()
//         }
//     }
// }
