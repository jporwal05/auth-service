use actix_web::{
    body::BoxBody, http::header::ContentType, web, HttpRequest, HttpResponse, Responder,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

use crate::schema::users::{self};

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Deserialize)]
pub struct CreateUserDto {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub username: String,
    pub message: Option<String>,
}

impl Responder for CreateUserResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

impl From<CreateUserDto> for CreateUserResponse {
    fn from(create_user_dto: CreateUserDto) -> Self {
        CreateUserResponse {
            username: create_user_dto.username,
            message: None,
        }
    }
}

impl From<web::Json<CreateUserRequest>> for CreateUserDto {
    fn from(create_user_request: actix_web::web::Json<CreateUserRequest>) -> Self {
        CreateUserDto {
            username: create_user_request.username.clone(),
            password: create_user_request.password.clone(),
        }
    }
}

impl From<User> for CreateUserDto {
    fn from(user: User) -> Self {
        CreateUserDto {
            username: user.username,
            password: user.password,
        }
    }
}
