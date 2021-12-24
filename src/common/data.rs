use bytes::Bytes;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::fmt::{Display, Error, Formatter};

use std::str::FromStr;

// ElId
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ElId {
    pub id: String,
}

impl Serialize for ElId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        [&self.id].serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ElId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s: String = Deserialize::deserialize(deserializer)?;
        Ok(ElId { id: s })
    }
}

impl Display for ElId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl FromStr for ElId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ElId {
            id: s.parse().unwrap(),
        })
    }
}

pub type InputId = ElId;
pub type TriggerId = ElId;
pub type ActionId = ElId;
pub type OutputId = ElId;

#[derive(Debug, Clone)]
pub struct DataEvent {
    pub meta: DataEventMeta,
    pub payload: Bytes,
}

#[derive(Debug, Clone)]
pub enum DataEventMeta {
    None,
    MqttMetadata { topic: String },
}

impl Default for DataEventMeta {
    fn default() -> Self {
        DataEventMeta::None
    }
}

#[derive(Debug, Clone)]
pub struct TriggeredEvent {
    pub input: InputId,
    pub trigger: TriggerId,
    pub data: DataEvent,
}

#[derive(Debug, Clone)]
pub struct ActionableEvent {
    pub output: OutputId,
    pub action: ActionId,
    pub data: DataEvent,
}
