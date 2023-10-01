use actix_web::{web, Responder};
use auth_service::services::user::{GetUser, UserService};
use slog::{info, Logger};

pub const GET_BY_ID_URL: &str = "/auth/users";

pub struct GetById {}

impl GetById {
    pub async fn get_by_id(
        path: web::Path<i32>,
        root_logger: web::Data<Logger>,
        user_service: web::Data<UserService>,
    ) -> impl Responder {
        let id = path.into_inner();
        info!(root_logger, "getting user"; "id" => id);
        user_service.get_user(id).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        handlers::signup::{SignUp, SIGN_UP_URL},
        log::init_logger,
        run_migration,
    };
    use actix_web::{
        test,
        web::{self, Data},
        App,
    };
    use auth_service::db::get_connection_pool;
    use serde_json::json;

    #[actix_web::test]
    async fn test_get_by_id() {
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
                ),
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
        assert!(resp.status().is_success());
    }
}
