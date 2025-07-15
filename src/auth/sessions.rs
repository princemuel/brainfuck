use std::collections::HashMap;

use crate::uuids::{SessionId, UserId};

pub trait Sessions {
    fn create_session(&mut self, user_uuid: UserId) -> SessionId;
    fn delete_session(&mut self, session_token: SessionId);
}

#[derive(Default)]
pub struct SessionsImpl {
    session_to_uuid: HashMap<SessionId, UserId>,
}

impl Sessions for SessionsImpl {
    fn create_session(&mut self, user_uuid: UserId) -> SessionId {
        let session = SessionId::new();
        self.session_to_uuid.insert(session, user_uuid);
        session
    }

    fn delete_session(&mut self, session_token: SessionId) {
        self.session_to_uuid.remove(&session_token);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_session() {
        let mut session_service = SessionsImpl::default();
        assert_eq!(session_service.session_to_uuid.len(), 0);

        let user_id = UserId::new();
        let session_id = session_service.create_session(user_id);

        assert_eq!(session_service.session_to_uuid.len(), 1);
        assert_eq!(session_service.session_to_uuid.get(&session_id).unwrap(), &user_id);
    }

    #[test]
    fn should_delete_session() {
        let mut session_service = SessionsImpl::default();

        let user_id = UserId::new();
        let session = session_service.create_session(user_id);
        session_service.delete_session(session);

        assert_eq!(session_service.session_to_uuid.len(), 0);
    }

    #[test]
    fn should_handle_multiple_users() {
        let mut session_service = SessionsImpl::default();

        let user1 = UserId::new();
        let user2 = UserId::new();

        let session1 = session_service.create_session(user1);
        let session2 = session_service.create_session(user2);

        assert_eq!(session_service.session_to_uuid.len(), 2);
        assert_ne!(session1, session2); // Different sessions

        // Delete one user's session
        session_service.delete_session(session1);
        assert_eq!(session_service.session_to_uuid.len(), 1);
        assert_eq!(session_service.session_to_uuid.get(&session2).unwrap(), &user2);
    }
}
