use axum::{
    Router,
    extract::Path,
    http::HeaderValue,
    response::{Html, IntoResponse},
    routing::get,
};
use reqwest::StatusCode;
use tokio::net::TcpListener;

pub mod build;
pub mod content;
pub mod secrets;
pub mod util;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let secrets = secrets::get_secrets().await?;
    // secrets::install_secrets(".secret".as_ref(), secrets.iter()).await?;

    // build::copy_files(".secret".as_ref(), ".secret-2".as_ref()).await?;

    let app = Router::new()
        .route("/", get(async || Html(content::hello_world().0)))
        .route(
            "/static/fonts/berkeley-mono/{font}",
            get(async |Path(font): Path<String>| {
                let data: Option<&[u8]> = match font.as_str() {
                    "regular.woff2" => Some(include_bytes!(
                        "../.secret/static/fonts/berkeley-mono/regular.woff2"
                    )),
                    "bold.woff2" => Some(include_bytes!(
                        "../.secret/static/fonts/berkeley-mono/bold.woff2"
                    )),

                    "italic.woff2" => Some(include_bytes!(
                        "../.secret/static/fonts/berkeley-mono/regular.woff2"
                    )),
                    "bold-italic.woff2" => Some(include_bytes!(
                        "../.secret/static/fonts/berkeley-mono/bold.woff2"
                    )),
                    _ => None,
                };

                match data {
                    Some(data) => (StatusCode::OK, data),
                    None => (StatusCode::NOT_FOUND, b"".as_slice()),
                }
            }),
        )
        .route(
            "/styles/main.css",
            get(async || {
                let mut resp = grass::include!("styles/main.scss").into_response();
                resp.headers_mut()
                    .insert("Content-Type", HeaderValue::from_static("text/css"));

                resp
            }),
        );
    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
