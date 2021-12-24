use paho_mqtt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::time::Duration;

use crate::common::data::{ActionId, ActionableEvent, ElId};
use crate::common::utils::random_alphanumeric;
use crate::outputs::mqtt::action::{MqttAction, MqttActionConfig};
use crate::outputs::OutputTask;
use async_trait::async_trait;

use itertools::Itertools;
use log::{error, trace};
use paho_mqtt::{ConnectOptions, Message};
use tokio::sync::mpsc::{channel, Receiver};
use tokio_stream::StreamExt;

// Config
#[derive(Debug, Default, PartialEq, Serialize, Deserialize, Clone)]
pub struct MqttOutputConfig {
    host: String,
    port: u16,
    username: Option<String>,
    password: Option<String>,
    #[serde(rename = "action")]
    #[serde(default)]
    actions: HashMap<ActionId, MqttActionConfig>,
}

#[derive(Debug)]
pub struct MqttOutput {
    id: ElId,
    actions: Vec<MqttAction>,
    config: MqttOutputConfig,
}

impl MqttOutput {
    pub fn new(id: ElId, config: MqttOutputConfig) -> Self {
        let actions = config
            .actions
            .clone()
            .into_iter()
            .map(|(action_id, action_config)| MqttAction::new(id.clone(), action_id, action_config))
            .collect_vec();

        Self {
            id,
            actions,
            config,
        }
    }
}

impl Display for MqttOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MqttOutput[{}]", self.id)
    }
}

#[async_trait]
impl OutputTask for MqttOutput {
    async fn run(self: Box<Self>, mut chan: Receiver<ActionableEvent>) {
        let (tx, rx) = channel(128);
        {
            let writer = Box::new(MqttOutputWriter::new(self.id.clone(), self.config.clone()));
            tokio::spawn(async move {
                writer.run(rx).await;
            });
        }

        while let Some(actionable_event) = chan.recv().await {
            let tx = tx.clone();
            trace!("{} received {:?}", &self, actionable_event);

            let mut actions_stream = tokio_stream::iter(self.actions.clone());

            tokio::spawn(async move {
                while let Some(action) = actions_stream.next().await {
                    if action.action_id == actionable_event.action {
                        trace!("{} will process the event", action);
                        if let Some(message) = action.process(&actionable_event).await {
                            trace!("{} processed the event", action);
                            tx.send(message).await.unwrap_or_else(|err| {
                                error!(
                                    "Can not send ActionableEvent {:?} to MqttOutputWriter: {:?}",
                                    &action, err
                                );
                            });
                        } else {
                            trace!("{} skipped the event", action);
                        };
                    };
                }
            });
        }
    }
}

// Writer
pub struct MqttOutputWriter {
    id: ElId,
    config: MqttOutputConfig,
}

impl MqttOutputWriter {
    pub fn new(id: ElId, config: MqttOutputConfig) -> Self {
        Self { id, config }
    }

    pub fn get_connect_options(&self) -> ConnectOptions {
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

    async fn run(self: Box<Self>, mut chan: Receiver<Message>) {
        let host = format!("tcp://{}:{}", self.config.host, self.config.port);

        let create_opts = paho_mqtt::CreateOptionsBuilder::new()
            .server_uri(host)
            .client_id(format!("mqrt-output-{}-{}", self.id, random_alphanumeric()))
            .finalize();

        let cli = paho_mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
            error!("Error creating the client: {:?}", e);
            panic!("Can not create MQTT client")
        });

        trace!("Connecting to the MQTT server...");
        cli.connect(self.get_connect_options())
            .await
            .expect("Can not connect to MQTT");

        while let Some(message) = chan.recv().await {
            trace!("{} received {:?}", self, message);
            cli.publish(message).await.unwrap_or_else(|err| {
                error!("Can not send message to Mqtt: {:?}", err);
            });
        }
    }
}

impl Display for MqttOutputWriter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MqttOutputWriter[{}]", self.id)
    }
}
