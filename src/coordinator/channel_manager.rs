use crate::common::data::ElId;
use crate::config::input::InputConfig;
use crate::config::output::OutputConfig;
use crate::config::Config;
use crate::coordinator::ChannelDispatcher;
use crate::inputs::mqtt::MqttInput;
use crate::inputs::InputTask;
use crate::outputs::mqtt::MqttOutput;
use crate::outputs::OutputTask;
use log::trace;
use std::default::Default;

#[derive(Debug, Default)]
pub struct ChannelManager {}

impl ChannelManager {
    pub async fn run(config: Config) {
        let mut dispatcher = ChannelDispatcher::new();

        {
            // spawn outputs
            for (id, config) in config.outputs.into_iter() {
                let task = Self::config_to_output(&id, &config);
                let rx = dispatcher.create_channel_for_output(&id);

                trace!("Spawning {}", task);
                tokio::spawn(async move {
                    task.run(rx).await;
                });
            }
        }

        {
            // spawn inputs
            for (id, config) in config.inputs.into_iter() {
                let task = Self::config_to_input(&id, &config);
                let tx = dispatcher.create_channel_for_input(&id);

                trace!("Spawning {}", task);
                tokio::spawn(async move {
                    task.run(tx).await;
                });
            }
        }

        {
            // spawn relations
            // TODO: no need to spawn here
            tokio::spawn(async move { dispatcher.run_handlers(config.handlers).await });
        }
    }

    fn config_to_input(id: &ElId, config: &InputConfig) -> Box<dyn InputTask> {
        let task = match config {
            InputConfig::Mqtt(config) => MqttInput::new(id.clone(), config.clone()),
        };

        Box::new(task)
    }

    fn config_to_output(id: &ElId, config: &OutputConfig) -> Box<dyn OutputTask> {
        let task = match config {
            OutputConfig::Mqtt(config) => MqttOutput::new(id.clone(), config.clone()),
        };

        Box::new(task)
    }
}
