use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use auth_service::db::get_connection_pool;
use auth_service::db::run_migration;
use handlers::signup::SignUp;
use handlers::signup::SIGN_UP_URL;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = get_connection_pool();
    let connection = &mut pool.get().unwrap();
    run_migration(connection);
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .route(SIGN_UP_URL, web::post().to(SignUp::sign_up))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
