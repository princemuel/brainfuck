use std::collections::HashMap;

use pbkdf2::Pbkdf2;
use pbkdf2::password_hash::rand_core::OsRng;
use pbkdf2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};

use crate::uuids::UserId;

pub trait Users {
    fn create_user(&mut self, username: String, password: String) -> Result<(), String>;
    fn get_user_uuid(&self, username: String, password: String) -> Option<UserId>;
    fn delete_user(&mut self, user_uuid: UserId);
}

#[derive(Debug, Clone)]
pub struct User {
    uuid:     UserId,
    username: String,
    password: String,
}

#[derive(Default, Debug, Clone)]
pub struct UsersImpl {
    uuid_to_user:     HashMap<UserId, User>,
    username_to_user: HashMap<String, User>,
}

impl Users for UsersImpl {
    fn create_user(&mut self, username: String, password: String) -> Result<(), String> {
        if self.username_to_user.contains_key(&username) {
            return Err(format!("Username '{username}' already exists"));
        }

        let salt = SaltString::generate(&mut OsRng);
        let password = Pbkdf2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| format!("Failed to hash password.\n{e:?}"))?
            .to_string();

        let id = UserId::new();
        let user = User { uuid: id, username: username.clone(), password };

        self.uuid_to_user.insert(id, user.clone());
        self.username_to_user.insert(username, user);

        Ok(())
    }

    fn get_user_uuid(&self, username: String, password: String) -> Option<UserId> {
        let user = self.username_to_user.get(&username)?;

        let parsed_hash = PasswordHash::new(&user.password).ok()?;

        Pbkdf2.verify_password(password.as_bytes(), &parsed_hash).ok()?;

        (username == user.username).then(|| user.uuid)
    }

    fn delete_user(&mut self, user_uuid: UserId) {
        self.username_to_user.retain(|_username, user| user.uuid != user_uuid);
        self.uuid_to_user.remove(&user_uuid);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_user() {
        let mut user_service = UsersImpl::default();
        user_service
            .create_user("username".to_owned(), "password".to_owned())
            .expect("should create user");

        assert_eq!(user_service.uuid_to_user.len(), 1);
        assert_eq!(user_service.username_to_user.len(), 1);
    }

    #[test]
    fn should_fail_creating_user_with_existing_username() {
        let mut user_service = UsersImpl::default();
        user_service
            .create_user("username".to_owned(), "password".to_owned())
            .expect("should create user");

        let result = user_service.create_user("username".to_owned(), "password".to_owned());

        assert!(result.is_err());
    }

    #[test]
    fn should_retrieve_user_uuid() {
        let mut user_service = UsersImpl::default();
        user_service
            .create_user("username".to_owned(), "password".to_owned())
            .expect("should create user");

        assert!(
            user_service.get_user_uuid("username".to_owned(), "password".to_owned()).is_some()
        );
    }

    #[test]
    fn should_fail_to_retrieve_user_uuid_with_incorrect_password() {
        let mut user_service = UsersImpl::default();
        user_service
            .create_user("username".to_owned(), "password".to_owned())
            .expect("should create user");

        assert!(
            user_service
                .get_user_uuid("username".to_owned(), "incorrect password".to_owned())
                .is_none()
        );
    }

    #[test]
    fn should_delete_user() {
        let mut user_service = UsersImpl::default();
        user_service
            .create_user("username".to_owned(), "password".to_owned())
            .expect("should create user");

        let user_uuid =
            user_service.get_user_uuid("username".to_owned(), "password".to_owned()).unwrap();

        user_service.delete_user(user_uuid);

        assert_eq!(user_service.uuid_to_user.len(), 0);
        assert_eq!(user_service.username_to_user.len(), 0);
    }
}
