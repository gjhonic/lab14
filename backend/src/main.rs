use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde_json::json;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(home))
        .route("/rust/test", get(test))
        .route("/v1/todos/:id", get(get_json));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn home() -> &'static str {
    "Home page"
}

async fn test() -> &'static str {
    "Test query. lab14"
}


async fn get_json(Path(id): Path<String>) -> impl IntoResponse {

    let user_guid = id.as_str();
    (
        StatusCode::OK,
        Json(json!(
            {
                "id": user_guid,
                "message": "Just do it!",
                "assigned": "user@example.com",
                "priority": "A"
            }
        )),
    )
}