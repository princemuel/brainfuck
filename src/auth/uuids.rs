use std::fmt::{self, Display};
use std::marker::PhantomData;
use std::ops::Deref;
use std::str::FromStr;

use uuid::Uuid;

pub type UserId = TypedUuid<UserIdMarker>;
pub type SessionId = TypedUuid<SessionIdMarker>;

// Zero-sized marker types to differentiate the wrapper types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserIdMarker;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SessionIdMarker;

/// Generic UUID wrapper that eliminates code duplication
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypedUuid<T>(Uuid, PhantomData<T>);

impl<T> TypedUuid<T> {
    pub fn new() -> Self { Self(Uuid::now_v7(), PhantomData) }

    pub fn as_bytes(&self) -> [u8; 16] { self.0.into_bytes() }

    pub fn as_str(&self) -> String { self.0.to_string() }
}

impl<T> Default for TypedUuid<T> {
    fn default() -> Self { Self::new() }
}

impl<T> Deref for TypedUuid<T> {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T> Display for TypedUuid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
}

impl<T> From<Uuid> for TypedUuid<T> {
    fn from(uuid: Uuid) -> Self { Self(uuid, std::marker::PhantomData) }
}

impl<T> From<TypedUuid<T>> for Uuid {
    fn from(wrapper: TypedUuid<T>) -> Self { wrapper.0 }
}

impl<T> TryFrom<String> for TypedUuid<T> {
    type Error = uuid::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let uuid = Uuid::parse_str(&s)?;
        Ok(Self(uuid, PhantomData))
    }
}

impl<T> FromStr for TypedUuid<T> {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::parse_str(s)?;
        Ok(Self(uuid, PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_deref_functionality() {
        let user_id = UserId::new();
        let session_id = SessionId::new();

        // Can call Uuid methods directly thanks to Deref
        assert_eq!(user_id.get_version(), Some(uuid::Version::SortRand));
        assert_eq!(session_id.get_version(), Some(uuid::Version::SortRand));

        // Can use in HashMap as keys
        let mut user_sessions: HashMap<UserId, SessionId> = HashMap::new();
        user_sessions.insert(user_id, session_id);

        assert_eq!(user_sessions.get(&user_id), Some(&session_id));
    }

    #[test]
    fn test_display_and_string_conversion() {
        let user_id = UserId::new();

        // Display trait works
        println!("User ID: {user_id}");

        // String conversion works
        let id_string = user_id.to_string();
        assert_eq!(id_string, user_id.as_str());
    }

    #[test]
    fn test_type_safety() {
        let user_id = UserId::new();
        let _session_id = SessionId::new();

        // These are different types despite having the same underlying structure
        // This would not compile: assert_eq!(user_id, session_id);

        // But they can be converted to/from Uuid
        let uuid_from_user: Uuid = user_id.into();
        let new_user_id: UserId = uuid_from_user.into();
        assert_eq!(user_id, new_user_id);
    }
}
