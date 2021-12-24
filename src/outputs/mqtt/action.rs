use crate::common::data::{ActionId, ActionableEvent, OutputId};
use crate::common::types::Result;
use log::{error, info};
use paho_mqtt;
use paho_mqtt::Message;
use rquickjs as rjs;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

// Config
#[derive(Debug, Default, PartialEq, Serialize, Deserialize, Clone)]
pub struct MqttActionConfig {
    topic: String,
    #[serde(default)]
    payload: MqttActionPayloadConfig,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
enum MqttActionPayloadConfig {
    Passthrough,
    Drop,
    Static { data: String },
    Js { code: String },
}

impl Default for MqttActionPayloadConfig {
    fn default() -> Self {
        MqttActionPayloadConfig::Passthrough
    }
}

// Action
#[derive(Debug, Clone)]
pub struct MqttAction {
    output_id: OutputId,
    pub action_id: ActionId,
    config: MqttActionConfig,
}

impl Display for MqttAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MqttAction[{}::{}]", self.output_id, self.action_id)
    }
}

impl MqttAction {
    pub fn new(output_id: OutputId, action_id: ActionId, config: MqttActionConfig) -> Self {
        Self {
            output_id,
            action_id,
            config,
        }
    }

    pub async fn process(&self, event: &ActionableEvent) -> Option<Message> {
        info!("Mqtt Action {} received {:?}", self.action_id, event);

        let topic = self.config.topic.clone();
        let payload = match &self.config.payload {
            MqttActionPayloadConfig::Passthrough => event.data.payload.to_vec(),
            MqttActionPayloadConfig::Drop => Vec::new(),
            MqttActionPayloadConfig::Static { data } => data.as_bytes().to_vec(),
            MqttActionPayloadConfig::Js { code } => process_js(code, event.data.payload.to_vec())
                .unwrap_or_else(|err| {
                    error!("Can not process javascript code={}: {:?}", code, err);
                    Vec::new()
                }),
        };

        let message = paho_mqtt::Message::new(topic, payload, paho_mqtt::QOS_1);
        Some(message)
    }
}

fn process_js(code: &str, payload: Vec<u8>) -> Result<Vec<u8>> {
    let rt = rjs::Runtime::new().unwrap();
    let ctx = rjs::Context::full(&rt).unwrap();

    let result: Result<String> = ctx.with(|ctx| {
        let func: rjs::Function = ctx.eval(format!("(payload) => {{ {} }}", code))?;

        let payload_string: String = String::from_utf8(payload)?;

        let result = func.call((payload_string,))?;
        Ok(result)
    });

    result.map(Vec::from)
}
