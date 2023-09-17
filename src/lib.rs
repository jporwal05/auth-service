pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use models::User;
use std::env;

use crate::models::NewUser;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn run_migration(connection: &mut PgConnection) {
    connection.run_pending_migrations(MIGRATIONS).unwrap();
}

pub trait CreateUser {
    fn create_user(conn: &mut PgConnection, username: &str) -> User;
}

pub struct UserService {}

impl CreateUser for UserService {
    fn create_user(conn: &mut PgConnection, username: &str) -> User {
        use crate::schema::users;

        let new_user = NewUser { username };

        diesel::insert_into(users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(conn)
            .expect("Error saving new user")
    }
}
