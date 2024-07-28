use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{application::user_service::UserService, domain::entity::user::User};
pub struct UserController {
    user_service: Arc<UserService>
}

impl UserController {
    pub fn new(user_service: Arc<UserService>) -> UserController {
        UserController {
            user_service
        }
    }

    pub async fn create(&self, Json(payload): Json<CreateUserRequest>) -> impl IntoResponse {
        let user = User {
            id: 1,
            name: payload.name,
            email: payload.email,
            password: payload.password
        };
        match self.user_service.create_user(&user).await {
            Ok(_) => StatusCode::OK,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    name: String,
    email: String,
    password: String
}