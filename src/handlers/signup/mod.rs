use actix_web::{web, Result};
use auth_service::{
    db::PostgresPool,
    services::{CreateUser, UserService},
};
use serde::Deserialize;
use slog::{info, Logger};

pub const SIGN_UP_URL: &str = "/auth/signup";

pub struct SignUp {}

impl SignUp {
    pub async fn sign_up(
        user: web::Json<User>,
        connection_pool: web::Data<PostgresPool>,
        root_logger: web::Data<Logger>,
    ) -> Result<String> {
        info!(root_logger, "signing up user"; "username" => user.username.as_str());
        let connection = &mut connection_pool.get().unwrap();
        let user = UserService::create_user(connection, user.username.as_str());
        info!(root_logger, "sign up successful for user"; "username" => user.username.as_str());
        Ok(format!("{} sign up successful for user", user.username))
    }
}

#[derive(Deserialize)]
pub struct User {
    username: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{log::init_logger, run_migration};
    use actix_web::{
        test,
        web::{self, Data},
        App,
    };
    use auth_service::db::get_connection_pool;
    use serde_json::json;

    #[actix_web::test]
    async fn test_signup() {
        let root_logger = init_logger();
        let pool = get_connection_pool();
        let connection = &mut pool.get().unwrap();
        run_migration(connection, root_logger.clone());
        let payload = json!({"username": "some_user"});

        let app = test::init_service(
            App::new()
                .app_data(Data::new(pool.clone()))
                .app_data(Data::new(root_logger.clone()))
                .route(SIGN_UP_URL, web::post().to(SignUp::sign_up)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri(SIGN_UP_URL)
            .set_json(&payload)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
