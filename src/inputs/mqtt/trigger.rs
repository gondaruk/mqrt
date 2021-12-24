use crate::common::data::DataEventMeta::MqttMetadata;
use crate::common::data::{DataEvent, InputId, TriggerId, TriggeredEvent};
use crate::common::types::Result;
use bytes::Bytes;
use log::{error, warn};
use paho_mqtt::Message;
use rquickjs as rjs;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// MQTT
#[derive(Debug, Default, PartialEq, Serialize, Deserialize, Clone)]
pub struct MqttTriggerConfig {
    pub topic: String,
    #[serde(default)]
    filter: MqttTriggerFilterConfig,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
enum MqttTriggerFilterConfig {
    NoFilter,
    DropAll,
    Js { code: String },
    Json { field: String, exact: String },
}

impl Default for MqttTriggerFilterConfig {
    fn default() -> Self {
        MqttTriggerFilterConfig::NoFilter
    }
}

#[derive(Debug, Clone)]
pub struct MqttTrigger {
    input_id: InputId,
    trigger_id: TriggerId,
    config: MqttTriggerConfig,
}

impl Display for MqttTrigger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MqttTrigger[{}::{}]", self.input_id, self.trigger_id)
    }
}

impl MqttTrigger {
    pub fn new(input_id: InputId, trigger_id: TriggerId, config: MqttTriggerConfig) -> Self {
        Self {
            input_id,
            trigger_id,
            config,
        }
    }

    pub async fn process(&self, message: &Message) -> Option<TriggeredEvent> {
        let topic = String::from(message.topic());

        let should_process = match topic.eq(&self.config.topic) {
            false => false,
            true => match &self.config.filter {
                MqttTriggerFilterConfig::NoFilter => true,
                MqttTriggerFilterConfig::DropAll => false,
                MqttTriggerFilterConfig::Js { code } => {
                    process_js(code, &topic, message.payload().to_vec()).unwrap_or_else(|err| {
                        error!("Can not process javascript filter code={}: {:?}", code, err);
                        false
                    })
                }

                MqttTriggerFilterConfig::Json { field, exact } => {
                    process_json(field, exact, &topic, message.payload().to_vec()).unwrap_or_else(
                        |err| {
                            error!(
                                "Can not process json filter field={:?}, exact={:?}: {:?}",
                                field, exact, err
                            );
                            false
                        },
                    )
                }
            },
        };

        if should_process {
            let event = TriggeredEvent {
                input: self.input_id.clone(),
                trigger: self.trigger_id.clone(),
                data: DataEvent {
                    payload: Bytes::copy_from_slice(message.payload()),
                    meta: MqttMetadata { topic },
                },
            };

            Some(event)
        } else {
            None
        }
    }
}

fn process_js(code: &str, topic: &str, payload: Vec<u8>) -> Result<bool> {
    let rt = rjs::Runtime::new().unwrap();
    let ctx = rjs::Context::full(&rt).unwrap();

    let result: Result<bool> = ctx.with(|ctx| {
        let func: rjs::Function = ctx.eval(format!(
            "(topic, payload) => {{ return !!(() => {{ {} }})() }}",
            code
        ))?;

        let topic_string = topic;
        let payload_string = String::from_utf8(payload)?;

        let result = func.call((topic_string, payload_string))?;
        Ok(result)
    });

    result
}

fn process_json(field: &str, exact: &str, _topic: &str, payload: Vec<u8>) -> Result<bool> {
    let payload_str = String::from_utf8(payload)?;

    let json_value = serde_json::from_str(payload_str.as_str())?;

    let result = match json_value {
        Value::Null => {
            if !field.is_empty() {
                warn!("Field is not empty, but the value is Boolean");
            }
            Ok(exact.to_lowercase().eq("null"))
        }
        Value::Bool(value) => {
            if !field.is_empty() {
                warn!("Field is not empty, but the value is Boolean");
            }
            Ok(exact.to_lowercase().eq(value.to_string().as_str()))
        }
        Value::Number(value) => {
            if !field.is_empty() {
                warn!("Field is not empty, but the value is Number");
            }
            Ok(value.eq(&serde_json::Number::from_str(exact)?))
        }
        Value::String(value) => {
            if !field.is_empty() {
                warn!("Field is not empty, but the value is String");
            }
            Ok(value.eq(exact))
        }
        Value::Array(value) => value
            .get(field.parse::<usize>()?)
            .map(|value| value.eq(exact))
            .ok_or_else(|| String::from("Invalid array field key")),
        Value::Object(value) => value
            .get(field)
            .map(|value| value.eq(exact))
            .ok_or_else(|| String::from("Invalid object field key")),
    };

    Ok(result?)
}
