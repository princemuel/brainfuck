use std::collections::HashMap;

use crate::uuids::{SessionId, UserId};

pub trait Sessions {
    fn create_session(&mut self, user_uuid: UserId) -> SessionId;
    fn delete_session(&mut self, user_uuid: UserId);
}

#[derive(Default)]
pub struct SessionsImpl {
    uuid_to_session: HashMap<UserId, SessionId>,
}

impl Sessions for SessionsImpl {
    fn create_session(&mut self, user_uuid: UserId) -> SessionId {
        let session = SessionId::new();
        self.uuid_to_session.insert(user_uuid, session);
        session
    }

    fn delete_session(&mut self, user_uuid: UserId) { self.uuid_to_session.remove(&user_uuid); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_session() {
        let mut session_service = SessionsImpl::default();
        assert_eq!(session_service.uuid_to_session.len(), 0);

        let user_id = UserId::new();
        let session_id = session_service.create_session(user_id);

        assert_eq!(session_service.uuid_to_session.len(), 1);
        assert_eq!(session_service.uuid_to_session.get(&user_id).unwrap(), &session_id);
    }

    #[test]
    fn should_delete_session() {
        let mut session_service = SessionsImpl::default();

        let user_id = UserId::new();
        session_service.create_session(user_id);
        session_service.delete_session(user_id);

        assert_eq!(session_service.uuid_to_session.len(), 0);
    }

    #[test]
    fn should_handle_multiple_users() {
        let mut session_service = SessionsImpl::default();

        let user1 = UserId::new();
        let user2 = UserId::new();

        let session1 = session_service.create_session(user1);
        let session2 = session_service.create_session(user2);

        assert_eq!(session_service.uuid_to_session.len(), 2);
        assert_ne!(session1, session2); // Different sessions

        // Delete one user's session
        session_service.delete_session(user1);
        assert_eq!(session_service.uuid_to_session.len(), 1);
        assert_eq!(session_service.uuid_to_session.get(&user2).unwrap(), &session2);
    }
}
