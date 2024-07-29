use std::sync::Arc;

use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use axum::extract::Query;
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
) -> Response {
    let user = User {
        id: 0,
        name: payload.name,
        email: payload.email,
        password: payload.password,
    };
    match module.create_user(&user).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_user_by_id(
    Extension(module): Extension<Arc<UserService>>,
    Query(param): Query<UserIdParam>,
) -> Response {
    if param.id <= 0 {
        return (StatusCode::BAD_REQUEST, "Invalid user id").into_response();
    }

    match module.find_user_by_id(param.id).await {
        Ok(Some(user)) => (StatusCode::OK, Json(user)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn delete_by_id(
    Extension(module): Extension<Arc<UserService>>,
    Query(param): Query<UserIdParam>,
) -> Response {
    if param.id <= 0 {
        return (StatusCode::BAD_REQUEST, "Invalid user id").into_response()
    };

    match module.delete_by_id(param.id).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

pub async fn update(
    Extension(module): Extension<Arc<UserService>>,
    Path(params): Path<UserIdParam>,
    Json(payload): Json<UpdateUserRequest>
) -> Response {
    let user = User {
        id: params.id,
        name: payload.name,
        email: payload.email,
        password: payload.password
    };

    match module.update(&user).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    name: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct UserIdParam {
    id: i32
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    name: String,
    email: String,
    password: String
}