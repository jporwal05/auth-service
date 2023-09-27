use diesel::pg::PgConnection;
use diesel::prelude::*;
use models::User;
use slog::{info, Logger};

use crate::models::{self, NewUser};

pub trait CreateUser {
    fn new(logger: Logger) -> Self;

    fn create_user(&self, conn: &mut PgConnection, username: &str) -> User;
}

pub struct UserService {
    logger: Logger,
}

impl CreateUser for UserService {
    fn new(logger: Logger) -> Self {
        UserService { logger: logger }
    }

    fn create_user(&self, conn: &mut PgConnection, username: &str) -> User {
        use crate::schema::users;
        info!(self.logger, "creating user"; "username" => username);

        let new_user = NewUser { username };

        diesel::insert_into(users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(conn)
            .expect("Error saving new user")
    }
}
