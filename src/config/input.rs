use crate::inputs::mqtt::MqttInputConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputConfig {
    Mqtt(MqttInputConfig),
}
