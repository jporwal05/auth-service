use diesel::prelude::*;
use models::User;

use slog::{info, Logger};

use crate::{
    db::PostgresPool,
    models::{self, CreateUserDto, NewUser},
};

pub trait CreateUser {
    fn new(logger: Logger, connection_pool: PostgresPool) -> Self;

    fn create_user(&self, user_dto: CreateUserDto) -> CreateUserDto;
}

#[derive(Clone)]
pub struct UserService {
    logger: Logger,
    connection_pool: PostgresPool,
}

impl CreateUser for UserService {
    fn new(logger: Logger, connection_pool: PostgresPool) -> Self {
        UserService {
            logger: logger,
            connection_pool: connection_pool,
        }
    }

    fn create_user(&self, user_dto: CreateUserDto) -> CreateUserDto {
        use crate::schema::users;
        info!(self.logger, "creating user"; "username" => &user_dto.username);

        let new_user = NewUser {
            username: &user_dto.username,
            password: &user_dto.password,
        };

        let user = diesel::insert_into(users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(&mut self.connection_pool.get().unwrap())
            .expect("Error saving new user");

        CreateUserDto::from(user)
    }
}
