use diesel::prelude::*;
use models::User;

use slog::{info, Logger};

use crate::{
    db::PostgresPool,
    models::{self, AuthServiceError, CreateUserDto, NewUser, UserDto},
    schema::users::{self, id, username},
};

use crate::services::user::users::dsl::users as users_select;

pub trait CreateUser {
    fn new(logger: Logger, connection_pool: PostgresPool) -> Self;

    fn create_user(&self, user_dto: CreateUserDto) -> Result<bool, AuthServiceError>;
}

pub trait GetUser {
    fn new(logger: Logger, connection_pool: PostgresPool) -> Self;

    fn get_user(&self, id: i32) -> Result<UserDto, AuthServiceError>;
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

    fn create_user(&self, user_dto: CreateUserDto) -> Result<bool, AuthServiceError> {
        info!(self.logger, "creating user"; "username" => &user_dto.username);

        let new_user = NewUser {
            username: &user_dto.username,
            password: &user_dto.password,
        };

        let connection = &mut self.connection_pool.get().unwrap();

        let users = users_select
            .filter(username.eq(&user_dto.username))
            .select(User::as_select())
            .load(connection)
            .expect("Could not find user");

        if users.len() == 0 {
            diesel::insert_into(users::table)
                .values(&new_user)
                .returning(User::as_returning())
                .get_result(connection)
                .expect("Error creating new user");
        }

        Ok(true)
    }
}

impl GetUser for UserService {
    fn new(logger: Logger, connection_pool: PostgresPool) -> Self {
        UserService {
            logger: logger,
            connection_pool: connection_pool,
        }
    }

    fn get_user(&self, identifier: i32) -> Result<UserDto, AuthServiceError> {
        let connection = &mut self.connection_pool.get().unwrap();

        let users: Vec<User> = users_select
            .filter(id.eq(identifier))
            .select(User::as_select())
            .load(connection)
            .expect("Could not find user");

        if users.len() > 0 {
            Ok(UserDto::from(&users[0]))
        } else {
            Err(AuthServiceError)
        }
    }
}