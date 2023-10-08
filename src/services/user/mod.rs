use diesel::prelude::*;
use models::User;

use slog::{info, Logger};

use crate::{
    db::PostgresPool,
    models::{self, AuthServiceError, NewUser, SignUpUserDto, UpdateUserDto, UserDto},
    schema::users::{self, id, password, username},
};

use crate::services::user::users::dsl::users as users_select;
use crate::services::user::users::dsl::users as users_update;

pub trait SignUpUser {
    fn new(logger: Logger, connection_pool: PostgresPool) -> Self;

    fn sign_up_user(&self, user_dto: SignUpUserDto) -> Result<i32, AuthServiceError>;
}

pub trait GetUser {
    fn new(logger: Logger, connection_pool: PostgresPool) -> Self;

    fn get_user(&self, id: i32) -> Result<UserDto, AuthServiceError>;
}

pub trait UpdateUser {
    fn new(logger: Logger, connection_pool: PostgresPool) -> Self;

    fn update_user(&self, user_dto: UpdateUserDto) -> Result<bool, AuthServiceError>;
}

#[derive(Clone)]
pub struct UserService {
    logger: Logger,
    connection_pool: PostgresPool,
}

impl SignUpUser for UserService {
    fn new(logger: Logger, connection_pool: PostgresPool) -> Self {
        UserService {
            logger: logger,
            connection_pool: connection_pool,
        }
    }

    fn sign_up_user(&self, user_dto: SignUpUserDto) -> Result<i32, AuthServiceError> {
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
            let user = diesel::insert_into(users::table)
                .values(&new_user)
                .returning(User::as_returning())
                .get_result(connection)
                .expect("Error creating new user");
            return Ok(user.id);
        } else if users.len() == 1 {
            let user: &User = &users[0];
            return Ok(user.id);
        }

        Err(AuthServiceError)
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

impl UpdateUser for UserService {
    fn new(logger: Logger, connection_pool: PostgresPool) -> Self {
        UserService {
            logger: logger,
            connection_pool: connection_pool,
        }
    }

    fn update_user(&self, user_dto: UpdateUserDto) -> Result<bool, AuthServiceError> {
        let connection = &mut self.connection_pool.get().unwrap();

        users_select
            .filter(id.eq(&user_dto.id))
            .select(User::as_select())
            .load(connection)
            .expect("Could not find user");

        diesel::update(users_update.filter(id.eq(&user_dto.id)))
            .set((
                username.eq(&user_dto.username.clone()),
                password.eq(&user_dto.password.clone()),
            ))
            .returning(User::as_returning())
            .get_result(connection)
            .expect("Could not update user");

        Ok(true)
    }
}
