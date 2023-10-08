use actix_web::{web, HttpResponse, Responder};
use auth_service::{
    models::{UpdateUserDto, UpdateUserRequest},
    services::user::{UpdateUser, UserService},
};
use slog::{error, info, Logger};

pub const UPDATE_URL: &str = "/auth/users";

pub struct Update {}

impl Update {
    pub async fn update(
        update_user_request: web::Json<UpdateUserRequest>,
        root_logger: web::Data<Logger>,
        user_service: web::Data<UserService>,
    ) -> impl Responder {
        let id = update_user_request.id;
        info!(root_logger, "updating user"; "id" => id);
        let result = user_service.update_user(UpdateUserDto::from(update_user_request));
        if result.is_ok() {
            info!(root_logger, "update successful for user"; "id" => id);
            HttpResponse::Ok()
        } else {
            error!(root_logger, "update failed for user: {}", result.unwrap_err(); "id" => id);
            HttpResponse::InternalServerError()
        }
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{
        test,
        web::{self, Data},
        App,
    };
    use auth_service::{
        db::{get_connection_pool, run_migration},
        models::UserDto,
        services::user::{UpdateUser, UserService},
    };
    use serde_json::json;
    use slog::info;

    use crate::{
        handlers::{
            getbyid::{GetById, GET_BY_ID_URL},
            signup::{SignUp, SIGN_UP_URL},
        },
        log::init_logger,
    };

    use super::{Update, UPDATE_URL};

    #[actix_web::test]
    async fn test_update() {
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
                .route(SIGN_UP_URL, web::post().to(SignUp::sign_up))
                .route(
                    format!("{}/{{id}}", GET_BY_ID_URL).as_str(),
                    web::get().to(GetById::get_by_id),
                )
                .route(UPDATE_URL, web::put().to(Update::update)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri(SIGN_UP_URL)
            .set_json(&payload)
            .to_request();
        test::call_service(&app, req).await;
        let req = test::TestRequest::get()
            .uri(format!("{}/1", GET_BY_ID_URL).as_str())
            .to_request();
        let resp = test::call_service(&app, req).await;
        // Extract and parse the response body as JSON
        let body_bytes = test::read_body(resp).await;
        let body_str = String::from_utf8(body_bytes.to_vec())
            .expect("Failed to convert response body to string");
        let mut user_dto: UserDto = serde_json::from_str(&body_str).unwrap();
        user_dto.username = "some_other_username".to_string();

        let req = test::TestRequest::put()
            .uri(UPDATE_URL)
            .set_json(serde_json::to_value(&user_dto).unwrap())
            .to_request();
        let resp = test::call_service(&app, req).await;
        info!(root_logger, "{}", resp.status());
        assert!(resp.status().is_success());
    }
}
