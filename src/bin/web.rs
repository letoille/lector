use axum::{
    extract::Json,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use lector::scrape;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/scrape", post(scrape_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn root() -> impl IntoResponse {
    let content = include_str!("../../static/index.html");
    Html(content)
}

#[derive(Deserialize)]
struct ScrapeRequest {
    url: String,
}

#[derive(Serialize)]
struct ScrapeResponse {
    title: String,
    markdown: String,
    author: Option<String>,
    site_name: Option<String>,
    published_time: Option<String>,
}

async fn scrape_handler(Json(payload): Json<ScrapeRequest>) -> impl IntoResponse {
    match scrape(&payload.url).await {
        Ok(scraped_data) => {
            let response = ScrapeResponse {
                title: scraped_data.title,
                markdown: scraped_data.markdown,
                author: scraped_data.author,
                site_name: scraped_data.site_name,
                published_time: scraped_data.published_time,
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        ).into_response(),
    }
}