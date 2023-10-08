use actix_web::{web, Responder};
use auth_service::{
    models::{SignUpUserDto, SignUpUserRequest, SignUpUserResponse},
    services::user::{SignUpUser, UserService},
};
use slog::{error, info, Logger};

pub const SIGN_UP_URL: &str = "/auth/users";

pub struct SignUp {}

impl SignUp {
    pub async fn sign_up(
        sign_up_user_request: web::Json<SignUpUserRequest>,
        root_logger: web::Data<Logger>,
        user_service: web::Data<UserService>,
    ) -> impl Responder {
        let username = sign_up_user_request.username.clone();
        info!(root_logger, "signing up user"; "username" => &username);
        let result = user_service.sign_up_user(SignUpUserDto::from(sign_up_user_request));
        if result.is_ok() {
            info!(root_logger, "sign up successful for user"; "username" => &username);
            SignUpUserResponse::new(result.ok().unwrap(), "success".to_owned())
        } else {
            error!(root_logger, "sign up failed for user: {}", result.unwrap_err(); "username" => &username);
            SignUpUserResponse::new(-1, "failed".to_owned())
        }
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
    use auth_service::{db::get_connection_pool, models::SignUpUserResponse};
    use serde_json::json;

    #[actix_web::test]
    async fn test_sign_up() {
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

        let body_bytes = test::read_body(resp).await;
        let body_str = String::from_utf8(body_bytes.to_vec())
            .expect("Failed to convert response body to string");
        let sign_up_user_response: SignUpUserResponse = serde_json::from_str(&body_str).unwrap();

        assert_eq!(1, sign_up_user_response.id);
    }
}
