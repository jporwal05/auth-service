use actix_web::{web, Result};
use auth_service::{
    models::{CreateUserDto, CreateUserRequest},
    services::{CreateUser, UserService},
};
use slog::{info, Logger};

pub const SIGN_UP_URL: &str = "/auth/signup";

pub struct SignUp {}

impl SignUp {
    pub async fn sign_up(
        create_user_request: web::Json<CreateUserRequest>,
        root_logger: web::Data<Logger>,
        user_service: web::Data<UserService>,
    ) -> Result<String> {
        info!(root_logger, "signing up user"; "username" => create_user_request.username.as_str());
        let create_user_dto = user_service.create_user(CreateUserDto::from(create_user_request));
        info!(root_logger, "sign up successful for user"; "username" => create_user_dto.username.as_str());
        Ok(format!(
            "{} sign up successful for user",
            create_user_dto.username
        ))
    }
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
        let user_service = UserService::new(root_logger.clone(), pool.clone());
        let payload = json!({"username": "some_user", "password": "plain_text_password"});

        let app = test::init_service(
            App::new()
                .app_data(Data::new(root_logger.clone()))
                .app_data(Data::new(user_service.clone()))
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
