use crate::outputs::mqtt::MqttOutputConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OutputConfig {
    Mqtt(MqttOutputConfig),
}
