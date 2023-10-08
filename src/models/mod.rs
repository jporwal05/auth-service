use std::fmt;

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
pub struct SignUpUserDto {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct SignUpUserRequest {
    pub username: String,
    pub password: String,
}

impl From<web::Json<SignUpUserRequest>> for SignUpUserDto {
    fn from(sign_up_user_request: actix_web::web::Json<SignUpUserRequest>) -> Self {
        SignUpUserDto {
            username: sign_up_user_request.username.clone(),
            password: sign_up_user_request.password.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SignUpUserResponse {
    pub id: i32,
    pub status: String,
}

impl SignUpUserResponse {
    pub fn new(id: i32, status: String) -> Self {
        SignUpUserResponse {
            id: id,
            status: status,
        }
    }
}

impl Responder for SignUpUserResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[derive(Deserialize)]
pub struct UpdateUserDto {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub id: i32,
    pub username: String,
    pub password: String,
}

impl From<web::Json<UpdateUserRequest>> for UpdateUserDto {
    fn from(update_user_request: actix_web::web::Json<UpdateUserRequest>) -> Self {
        UpdateUserDto {
            id: update_user_request.id,
            username: update_user_request.username.clone(),
            password: update_user_request.password.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserDto {
    pub id: i32,
    pub username: String,
    pub password: String,
}

impl From<&User> for UserDto {
    fn from(user: &User) -> Self {
        UserDto {
            id: user.id,
            username: user.username.clone(),
            password: user.password.clone(),
        }
    }
}

impl Responder for UserDto {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[derive(Debug, Clone)]
pub struct AuthServiceError;

impl fmt::Display for AuthServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Some error here")
    }
}
