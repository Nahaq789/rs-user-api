use std::sync::Arc;

use app::adapter::user_controller::{create, get_user_by_id, UserController};
use app::application::user_service::UserService;
use app::config::establish_pool;
use app::infrastructure::user_repository::PgUserRepository;

use axum::Extension;
use axum::{
    extract::Path,
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
    // let user_controller = Arc::new(UserController::new(user_service));

    let modules = user_service;

    let app = Router::new()
        .route("/user", post(create))
        .route("/user:id", get(get_user_by_id))
        .layer(Extension(modules));
    // .route("/user", post(move |payload| {
    //     let user_controller = Arc::clone(&user_controller);
    //     async move { user_controller.create(payload).await }
    // }))
    // .route("/user:id", get(move |Path(id): Path<i32>| {
    //     let user_controller = Arc::clone(&user_controller);
    //     async move { user_controller.get_user_by_id(id).await}
    // }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
