use crate::common::data::ElId;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HandlerTriggerConfig {
    #[serde(rename = "input")]
    pub input_id: ElId,
    #[serde(rename = "trigger")]
    pub trigger_id: ElId,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HandlerActionConfig {
    #[serde(rename = "output")]
    pub output_id: ElId,
    #[serde(rename = "action")]
    pub action_id: ElId,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HandlerConfig {
    #[serde(rename = "on")]
    pub trigger: HandlerTriggerConfig,

    #[serde(rename = "do")]
    pub action: HandlerActionConfig,
}
