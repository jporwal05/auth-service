use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use auth_service::db::get_connection_pool;
use auth_service::db::run_migration;
use auth_service::services::{CreateUser, UserService};
use handlers::signup::SignUp;
use handlers::signup::SIGN_UP_URL;
use log::init_logger;
mod handlers;
mod log;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let root_logger = init_logger();
    let pool = get_connection_pool();
    let connection = &mut pool.get().unwrap();
    run_migration(connection, root_logger.clone());
    let user_service = UserService::new(root_logger.clone(), pool.clone());
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(root_logger.clone()))
            .app_data(Data::new(user_service.clone()))
            .route(SIGN_UP_URL, web::post().to(SignUp::sign_up))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
