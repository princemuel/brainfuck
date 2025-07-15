use std::sync::Mutex;

use oauth::auth_server::Auth;
use oauth::{
    SignInRequest,
    SignInResponse,
    SignOutRequest,
    SignOutResponse,
    SignUpRequest,
    SignUpResponse,
    StatusCode,
};
use tonic::{Request, Response, Status};

use crate::sessions::Sessions;
use crate::users::Users;

pub mod oauth {
    tonic::include_proto!("oauth");
}

// Re-exporting
pub use oauth::auth_server::AuthServer;
pub use tonic::transport::Server;

pub struct AuthService {
    users_service:    Box<Mutex<dyn Users + Send + Sync>>,
    sessions_service: Box<Mutex<dyn Sessions + Send + Sync>>,
}

impl AuthService {
    pub fn new(
        users_service: Box<Mutex<dyn Users + Send + Sync>>,
        sessions_service: Box<Mutex<dyn Sessions + Send + Sync>>,
    ) -> Self {
        Self { users_service, sessions_service }
    }
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn sign_in(
        &self,
        request: Request<SignInRequest>,
    ) -> Result<Response<SignInResponse>, Status> {
        println!("Got a request: {:?}", request);

        let request = request.into_inner();

        let result = self
            .users_service
            .lock()
            .expect("lock should not be poisoned")
            .get_user_uuid(&request.username, &request.password);

        let user_uuid = match result {
            Some(uuid) => uuid,
            None => {
                let response = SignInResponse {
                    user_id:       "".to_string(),
                    session_token: "".to_owned(),
                    status_code:   StatusCode::Failure.into(),
                };

                return Ok(Response::new(response));
            },
        };

        let session_token = self
            .sessions_service
            .lock()
            .expect("lock should not be poisoned")
            .create_session(user_uuid);

        let response = SignInResponse {
            user_id:       user_uuid.to_string(),
            session_token: session_token.to_string(),
            status_code:   StatusCode::Success.into(),
        };

        Ok(Response::new(response))
    }

    async fn sign_up(
        &self,
        request: Request<SignUpRequest>,
    ) -> Result<Response<SignUpResponse>, Status> {
        println!("Got a request: {:?}", request);

        let request = request.into_inner();

        let result = self
            .users_service
            .lock()
            .expect("lock should not be poisoned")
            .create_user(&request.username, &request.password);

        match result {
            Ok(_) => {
                let response = SignUpResponse { status_code: StatusCode::Success.into() };
                return Ok(Response::new(response));
            },
            Err(_) => {
                let response = SignUpResponse { status_code: StatusCode::Failure.into() };
                return Ok(Response::new(response));
            },
        };
    }

    async fn sign_out(
        &self,
        request: Request<SignOutRequest>,
    ) -> Result<Response<SignOutResponse>, Status> {
        println!("Got a request: {:?}", request);

        let request = request.into_inner();

        let status_code = match request.session_token.try_into() {
            Ok(token) => {
                self.sessions_service
                    .lock()
                    .expect("lock should not be poisoned")
                    .delete_session(token);
                StatusCode::Success.into()
            },
            Err(_) => StatusCode::Failure.into(),
        };

        let response = SignOutResponse { status_code };

        Ok(Response::new(response))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sessions::SessionsImpl;
    use crate::users::UsersImpl;

    #[tokio::test]
    async fn sign_in_should_fail_if_user_not_found() {
        let users_service = Box::new(Mutex::new(UsersImpl::default()));
        let sessions_service = Box::new(Mutex::new(SessionsImpl::default()));

        let auth_service = AuthService::new(users_service, sessions_service);

        let request = tonic::Request::new(SignInRequest {
            username: "123456".to_owned(),
            password: "654321".to_owned(),
        });

        let result = auth_service.sign_in(request).await.unwrap().into_inner();

        assert_eq!(result.status_code, StatusCode::Failure.into());
        assert!(result.user_id.is_empty());
        assert!(result.session_token.is_empty());
    }

    #[tokio::test]
    async fn sign_in_should_fail_if_incorrect_password() {
        let mut users_service = UsersImpl::default();
        let _ = users_service.create_user("123456", "654321");

        let users_service = Box::new(Mutex::new(users_service));
        let sessions_service = Box::new(Mutex::new(SessionsImpl::default()));

        let auth_service = AuthService::new(users_service, sessions_service);

        let request = tonic::Request::new(SignInRequest {
            username: "123456".to_owned(),
            password: "wrong password".to_owned(),
        });

        let result = auth_service.sign_in(request).await.unwrap().into_inner();

        assert_eq!(result.status_code, StatusCode::Failure.into());
        assert!(result.user_id.is_empty());
        assert!(result.session_token.is_empty());
    }

    #[tokio::test]
    async fn sign_in_should_succeed() {
        let mut users_service = UsersImpl::default();
        let _ = users_service.create_user("123456", "654321");

        let users_service = Box::new(Mutex::new(users_service));
        let sessions_service = Box::new(Mutex::new(SessionsImpl::default()));

        let auth_service = AuthService::new(users_service, sessions_service);

        let request = tonic::Request::new(SignInRequest {
            username: "123456".to_owned(),
            password: "654321".to_owned(),
        });

        let result = auth_service.sign_in(request).await.unwrap().into_inner();

        assert_eq!(result.status_code, StatusCode::Success.into());
        assert!(!result.user_id.is_empty());
        assert!(!result.session_token.is_empty());
    }

    #[tokio::test]
    async fn sign_up_should_fail_if_username_exists() {
        let mut users_service = UsersImpl::default();
        let _ = users_service.create_user("123456", "654321");

        let users_service = Box::new(Mutex::new(users_service));
        let sessions_service = Box::new(Mutex::new(SessionsImpl::default()));

        let auth_service = AuthService::new(users_service, sessions_service);

        let request = tonic::Request::new(SignUpRequest {
            username: "123456".to_owned(),
            password: "654321".to_owned(),
        });

        let result = auth_service.sign_up(request).await.unwrap();

        assert_eq!(result.into_inner().status_code, StatusCode::Failure.into());
    }

    #[tokio::test]
    async fn sign_up_should_succeed() {
        let users_service = Box::new(Mutex::new(UsersImpl::default()));
        let sessions_service = Box::new(Mutex::new(SessionsImpl::default()));

        let auth_service = AuthService::new(users_service, sessions_service);

        let request = tonic::Request::new(SignUpRequest {
            username: "123456".to_owned(),
            password: "654321".to_owned(),
        });

        let result = auth_service.sign_up(request).await.unwrap();

        assert_eq!(result.into_inner().status_code, StatusCode::Success.into());
    }

    #[tokio::test]
    async fn sign_out_should_fail_if_incorrect_token() {
        let users_service = Box::new(Mutex::new(UsersImpl::default()));
        let sessions_service = Box::new(Mutex::new(SessionsImpl::default()));

        let auth_service = AuthService::new(users_service, sessions_service);

        let request = tonic::Request::new(SignOutRequest { session_token: "".to_owned() });

        let result = auth_service.sign_out(request).await.unwrap();

        assert_eq!(result.into_inner().status_code, StatusCode::Failure.into());
    }

    #[tokio::test]
    async fn sign_out_should_succeed() {
        let mut users_service = UsersImpl::default();
        let _ = users_service.create_user("123456", "654321");

        let users_service = Box::new(Mutex::new(users_service));
        let sessions_service = Box::new(Mutex::new(SessionsImpl::default()));

        let auth_service = AuthService::new(users_service, sessions_service);

        let request = tonic::Request::new(SignInRequest {
            username: "123456".to_owned(),
            password: "654321".to_owned(),
        });
        let result = auth_service.sign_in(request).await.unwrap().into_inner();

        let request =
            tonic::Request::new(SignOutRequest { session_token: result.session_token });
        let result = auth_service.sign_out(request).await.unwrap();

        assert_eq!(result.into_inner().status_code, StatusCode::Success.into());
    }
}
