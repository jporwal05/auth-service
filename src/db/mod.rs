use diesel::{r2d2::ConnectionManager, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use r2d2::Pool;
use std::env;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_connection_pool() -> PostgresPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection_manager = ConnectionManager::new(database_url);
    Pool::builder()
        .max_size(10)
        .build(connection_manager)
        .unwrap()
}

pub fn run_migration(connection: &mut PgConnection) {
    connection.run_pending_migrations(MIGRATIONS).unwrap();
}
