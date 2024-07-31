use std::sync::Arc;

use app::adapter::controller::user_controller::{create, delete_by_id, get_user_by_id, update};
use app::adapter::database::config::establish_pool;
use app::application::user_service::UserService;
use app::infrastructure::user_repository::PgUserRepository;

use axum::routing::{delete, patch};
use axum::Extension;
use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool = establish_pool()
        .await
        .expect("Failed to create database pool");
    let user_repository = Arc::new(PgUserRepository::new(pool));
    let user_service = Arc::new(UserService::new(user_repository));

    let app = Router::new()
        .route("/user", post(create))
        .route("/user", get(get_user_by_id))
        .route("/user", delete(delete_by_id))
        .route("/user/:id", patch(update))
        .layer(Extension(user_service));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
