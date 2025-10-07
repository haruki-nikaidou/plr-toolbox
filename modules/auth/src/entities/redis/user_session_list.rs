use kanaeru::redis::{KeyValue, KeyValueRead, KeyValueWrite, RedisKey};
use kanau::{RkyvMessageDe, RkyvMessageSer};
use uuid::Uuid;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    RkyvMessageDe,
    RkyvMessageSer,
)]
pub struct UserSessions {
    pub user_id: Uuid,
    pub session_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct UserIdIndex(pub Uuid);

impl From<Uuid> for UserIdIndex {
    fn from(v: Uuid) -> Self {
        Self(v)
    }
}

impl From<UserIdIndex> for RedisKey {
    fn from(v: UserIdIndex) -> Self {
        let string = format!("user_sessions_list:{}", v.0);
        Self::from(string)
    }
}

impl KeyValue for UserSessions {
    type Key = UserIdIndex;
    type Value = Self;

    fn key(&self) -> Self::Key {
        UserIdIndex(self.user_id)
    }

    fn value(&self) -> Self::Value {
        self.clone()
    }

    fn into_value(self) -> Self::Value {
        self
    }

    fn new(key: Self::Key, mut value: Self::Value) -> Self {
        value.user_id = key.0;
        value
    }
}

impl KeyValueRead for UserSessions {}
impl KeyValueWrite for UserSessions {}
