use actix_web::{web, Result};
use auth_service::{db::PostgresPool, CreateUser, UserService};
use serde::Deserialize;

pub const SIGN_UP_URL: &str = "/auth/signup";

pub struct SignUp {}

impl SignUp {
    pub async fn sign_up(
        user: web::Json<User>,
        connection_pool: web::Data<PostgresPool>,
    ) -> Result<String> {
        let connection = &mut connection_pool.get().unwrap();
        let user = UserService::create_user(connection, user.username.as_str());
        Ok(format!("{} singed up successfully!", user.username))
    }
}

#[derive(Deserialize)]
pub struct User {
    username: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run_migration;
    use actix_web::{
        test,
        web::{self, Data},
        App,
    };
    use auth_service::db::get_connection_pool;
    use serde_json::json;

    #[actix_web::test]
    async fn test_signup() {
        let pool = get_connection_pool();
        let connection = &mut pool.get().unwrap();
        run_migration(connection);
        let payload = json!({"username": "some_user"});

        let app = test::init_service(
            App::new()
                .app_data(Data::new(pool.clone()))
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
