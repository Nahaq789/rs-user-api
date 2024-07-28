use std::sync::Arc;

use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde::Deserialize;

use crate::{
    application::user_service::UserService,
    domain::entity::user::{self, User},
};
pub struct UserController {
    user_service: Arc<UserService>,
}

impl UserController {
    pub fn new(user_service: Arc<UserService>) -> UserController {
        UserController { user_service }
    }
}

pub async fn create(
    Extension(module): Extension<Arc<UserService>>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse {
    let user = User {
        id: 1,
        name: payload.name,
        email: payload.email,
        password: payload.password,
    };
    match module.create_user(&user).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn get_user_by_id(
    Extension(module): Extension<Arc<UserService>>,
    Path(id): Path<i32>,
) -> Response {
    if id <= 0 {
        return (StatusCode::BAD_REQUEST, "Invalid user id").into_response();
    }

    match module.find_user_by_id(id).await {
        Ok(Some(user)) => (StatusCode::OK, Json(user)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
#[derive(Deserialize)]
pub struct CreateUserRequest {
    name: String,
    email: String,
    password: String,
}
