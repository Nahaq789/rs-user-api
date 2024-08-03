use std::sync::Arc;

use axum::extract::Query;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde::Deserialize;

use crate::application::user_service::UserService;
use crate::{application::user_service::UserServiceImpl, domain::entity::user::User};

pub async fn create(
    Extension(module): Extension<Arc<dyn UserService>>,
    Json(payload): Json<CreateUserRequest>,
) -> Response {
    let user = User {
        id: 0,
        name: payload.name,
        email: payload.email,
        password: payload.password,
        salt: String::from(""),
    };
    match module.create_user(user).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_user_by_id(
    Extension(module): Extension<Arc<UserServiceImpl>>,
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
    Extension(module): Extension<Arc<UserServiceImpl>>,
    Query(param): Query<UserIdParam>,
) -> Response {
    if param.id <= 0 {
        return (StatusCode::BAD_REQUEST, "Invalid user id").into_response();
    };

    match module.delete_by_id(param.id).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn update(
    Extension(module): Extension<Arc<UserServiceImpl>>,
    Path(params): Path<UserIdParam>,
    Json(payload): Json<UpdateUserRequest>,
) -> Response {
    let user = User {
        id: params.id,
        name: payload.name,
        email: payload.email,
        password: payload.password,
        salt: String::from(""),
    };

    match module.update(&user).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
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
    id: i32,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    name: String,
    email: String,
    password: String,
}
