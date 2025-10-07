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
    RkyvMessageSer,
    RkyvMessageDe,
)]
pub struct Session {
    pub id: SessionId,
    pub user_id: Uuid,
    pub terminated: bool,
    pub last_refreshed: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct SessionId(pub Uuid);

impl From<SessionId> for RedisKey {
    fn from(value: SessionId) -> Self {
        let string = format!("session:{}", value.0);
        Self::from(string)
    }
}

impl KeyValue for Session {
    type Key = SessionId;
    type Value = Self;

    fn key(&self) -> Self::Key {
        self.id
    }

    fn value(&self) -> Self::Value {
        self.clone()
    }

    fn into_value(self) -> Self::Value {
        self
    }

    fn new(key: Self::Key, mut value: Self::Value) -> Self {
        value.id = key;
        value
    }
}

impl KeyValueRead for Session {}
impl KeyValueWrite for Session {}
