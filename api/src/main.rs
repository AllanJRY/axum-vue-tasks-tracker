use axum::{
    http::{HeaderValue, Method},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/api/tasks", get(get_tasks))
        .layer(
            CorsLayer::new()
                .allow_origin("http://127.0.0.1:5173".parse::<HeaderValue>().unwrap())
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::DELETE,
                    Method::HEAD,
                ]),
        );

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_tasks() -> Json<Vec<Task>> {
    let tasks = vec![
        Task {
            title: "Apprendre Rust".to_string(),
            content: "Coder un maximum de projet et souvent pour apprendre rust".to_string(),
            status: TaskStatus::Todo,
        },
        Task {
            title: "Apprendre VueJS 3".to_string(),
            content: "Coder un maximum de projet et souvent avec vue js 3".to_string(),
            status: TaskStatus::Todo,
        },
        Task {
            title: "Apprendre Axum".to_string(),
            content: "Coder un maximum de projet et souvent avec axum".to_string(),
            status: TaskStatus::Todo,
        },
        Task {
            title: "Apprendre Tauri".to_string(),
            content: "Coder un maximum de projet et souvent pour apprendre tauri".to_string(),
            status: TaskStatus::Todo,
        },
    ];

    Json(tasks)
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    title: String,
    content: String,
    status: TaskStatus,
}
#[derive(Debug, Serialize, Deserialize)]
enum TaskStatus {
    Todo,
    Doing,
    Done,
}
