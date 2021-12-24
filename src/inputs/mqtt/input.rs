use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::common::data::{InputId, TriggerId, TriggeredEvent};
use crate::common::types::Result;
use crate::inputs::mqtt::trigger::{MqttTrigger, MqttTriggerConfig};
use crate::inputs::InputTask;
use async_trait::async_trait;
use bytes::Bytes;
use log::{error, trace, warn};
use tokio::sync::mpsc::Sender;

use crate::common::utils::random_alphanumeric;
use tokio_stream::StreamExt;

use itertools::Itertools;
use paho_mqtt;
use paho_mqtt::{ConnectOptions, Message};

// MQTT
#[derive(Debug, Default, PartialEq, Serialize, Deserialize, Clone)]
pub struct MqttInputConfig {
    host: String,
    port: u16,
    username: Option<String>,
    password: Option<String>,
    #[serde(rename = "trigger")]
    #[serde(default)]
    pub triggers: HashMap<TriggerId, MqttTriggerConfig>,
}

#[derive(Debug)]
pub struct MqttInput {
    id: InputId,
    triggers: Vec<MqttTrigger>,
    config: MqttInputConfig,
}

impl MqttInput {
    pub fn new(id: InputId, config: MqttInputConfig) -> Self {
        let triggers = config
            .triggers
            .clone()
            .into_iter()
            .map(|(trigger_id, trigger_config)| {
                MqttTrigger::new(id.clone(), trigger_id, trigger_config)
            })
            .collect_vec();
        Self {
            id,
            triggers,
            config,
        }
    }

    fn get_connect_options(&self) -> ConnectOptions {
        let mut connect_opts = paho_mqtt::ConnectOptionsBuilder::new();
        connect_opts
            .keep_alive_interval(Duration::from_secs(30))
            .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(16))
            .mqtt_version(paho_mqtt::MQTT_VERSION_3_1_1)
            .clean_session(true);

        if let Some(username) = &self.config.username {
            connect_opts.user_name(username);
        }

        if let Some(password) = &self.config.password {
            connect_opts.password(password);
        }

        connect_opts.finalize()
    }
}

impl Display for MqttInput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MqttInput[{}]", self.id)
    }
}

#[async_trait]
impl InputTask for MqttInput {
    async fn run(self: Box<Self>, chan: Sender<TriggeredEvent>) {
        let host = format!("tcp://{}:{}", self.config.host, self.config.port);

        let create_opts = paho_mqtt::CreateOptionsBuilder::new()
            .server_uri(host)
            .client_id(format!("mqrt-input-{}-{}", self.id, random_alphanumeric()))
            .finalize();

        let mut cli = paho_mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
            error!("Error creating the client: {:?}", e);
            panic!("Can not create MQTT client")
        });

        let listen_topics: Vec<String> = self
            .config
            .triggers
            .values()
            .map(|x| x.topic.clone())
            .collect();
        let qos: Vec<i32> = [1].repeat(listen_topics.len());

        let mut strm = cli.get_stream(25);

        trace!("Connecting to the MQTT server...");
        cli.connect(self.get_connect_options())
            .await
            .expect("Can not connect to MQTT");
        trace!("Subscribing to topics: {:?}", listen_topics);

        cli.subscribe_many(&listen_topics, &qos)
            .await
            .expect("Can not subscribe to topics");

        trace!("Waiting for messages...");

        while let Some(some_mqtt_message) = strm.next().await {
            if let Some(mqtt_message) = some_mqtt_message {
                trace!("{} received {}", self, mqtt_message);
                let triggers = self.triggers.clone();
                let chan = chan.clone();
                tokio::spawn(async move { process_message(&triggers, chan, mqtt_message).await });
            } else {
                // A "None" means we were disconnected. Try to reconnect...
                trace!("{} received None", self);

                warn!("Lost mqtt connection...");
                todo!();
            }
        }
    }
}

async fn process_message(
    triggers: &[MqttTrigger],
    chan: Sender<TriggeredEvent>,
    mqtt_message: Message,
) -> Result<()> {
    let _topic = mqtt_message.topic().to_string();
    let _payload = Bytes::from(mqtt_message.payload().to_vec()).clone();

    let mut triggers_stream = tokio_stream::iter(triggers);

    while let Some(trigger) = triggers_stream.next().await {
        if let Some(triggered_event) = trigger.process(&mqtt_message).await {
            trace!("{} processed message {}", trigger, mqtt_message);
            chan.send(triggered_event)
                .await
                .unwrap_or_else(|err| warn!("Can not send TriggeredEvent {:?}", &err));
        } else {
            trace!("{} skipped message {}", trigger, mqtt_message);
        }
    }
    Ok(())
}
