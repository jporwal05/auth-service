pub mod db;
pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use models::User;

use crate::models::NewUser;

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
