use actix_web::{web, App, HttpServer};
use auth_service::establish_connection;
use auth_service::run_migration;
use handlers::signup::SignUp;
use handlers::signup::SIGN_UP_URL;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = &mut establish_connection();
    run_migration(connection);
    HttpServer::new(|| App::new().route(SIGN_UP_URL, web::post().to(SignUp::sign_up)))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
