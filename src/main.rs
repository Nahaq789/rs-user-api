use std::sync::Arc;

use app::adapter::user_controller::UserController;
use app::application::user_service::UserService;
use app::config::establish_pool;
use app::infrastructure::user_repository::PgUserRepository;

use axum::{
    routing::post, Router
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool = establish_pool().await.expect("Failed to create database pool");
    let user_repository = Arc::new(PgUserRepository::new(pool));
    let user_service = Arc::new(UserService::new(user_repository));
    let user_controller = Arc::new(UserController::new(user_service));


    let app = Router::new()
        .route("/", post(move |payload| {
            let user_controller = Arc::clone(&user_controller);
            async move { user_controller.create(payload).await }
        }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}