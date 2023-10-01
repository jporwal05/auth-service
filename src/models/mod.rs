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
pub struct CreateUserDto {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
}

impl From<web::Json<CreateUserRequest>> for CreateUserDto {
    fn from(create_user_request: actix_web::web::Json<CreateUserRequest>) -> Self {
        CreateUserDto {
            username: create_user_request.username.clone(),
            password: create_user_request.password.clone(),
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
