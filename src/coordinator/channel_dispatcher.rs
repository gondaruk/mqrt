use crate::common::data::{
    ActionId, ActionableEvent, ElId, InputId, OutputId, TriggerId, TriggeredEvent,
};
use std::collections::HashMap;

use crate::config::handler::HandlerConfig;
use tokio::sync::mpsc;

use futures::future::join_all;
use itertools::Itertools;
use log::{error, trace};
#[allow(unused_imports)]
use tokio_stream::StreamExt;

type RoutesByTriggerIdMap =
    HashMap<TriggerId, Vec<(mpsc::Sender<ActionableEvent>, OutputId, ActionId)>>;

#[derive(Debug, Default)]
pub struct ChannelDispatcher {
    inputs: HashMap<InputId, mpsc::Receiver<TriggeredEvent>>,
    outputs: HashMap<OutputId, mpsc::Sender<ActionableEvent>>,
}

impl ChannelDispatcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_channel_for_input(&mut self, id: &ElId) -> mpsc::Sender<TriggeredEvent> {
        let (tx, rx) = mpsc::channel(128);
        self.inputs.insert(id.clone(), rx);

        tx
    }

    pub fn create_channel_for_output(&mut self, id: &ElId) -> mpsc::Receiver<ActionableEvent> {
        let (tx, rx) = mpsc::channel(128);
        self.outputs.insert(id.clone(), tx);

        rx
    }

    pub async fn run_handlers(self, handlers: Vec<HandlerConfig>) {
        for (input_id, mut input_rx) in self.inputs.into_iter() {
            // Useless conversion is allowed here due to issue with Rust JetBrains extension :(
            #[allow(clippy::useless_conversion)]
            let actions_by_trigger: RoutesByTriggerIdMap = handlers
                .clone()
                .iter()
                .filter(|conf| conf.trigger.input_id == input_id)
                .map(|conf| {
                    trace!(
                        "Found route from Trigger[{}::{}] to Action[{}::{}]",
                        conf.trigger.input_id,
                        conf.trigger.trigger_id,
                        conf.action.output_id,
                        conf.action.action_id
                    );
                    conf
                })
                .group_by(|conf| conf.trigger.trigger_id.clone())
                .into_iter()
                .map(|(trigger_id, group)| {
                    (
                        trigger_id,
                        group
                            .map(|conf| {
                                (conf.action.output_id.clone(), conf.action.action_id.clone())
                            })
                            .map(|(output_id, action_id)| {
                                (
                                    self.outputs
                                        .get(&output_id)
                                        .unwrap_or_else(|| {
                                            panic!("Can not find output with id={}", &output_id)
                                        })
                                        .clone(),
                                    output_id,
                                    action_id,
                                )
                            })
                            .collect(),
                    )
                })
                .collect::<RoutesByTriggerIdMap>()
                .try_into()
                .unwrap();

            tokio::spawn(async move {
                while let Some(triggered_event) = input_rx.recv().await {
                    trace!(
                        "Router received event from Trigger[{}::{}]",
                        triggered_event.input,
                        triggered_event.trigger
                    );
                    let trigger_id = &triggered_event.trigger;

                    if let Some(actions) = actions_by_trigger.get(trigger_id) {
                        let send_futures = actions
                            .iter()
                            .map(|(tx, output_id, action_id)| {
                                trace!(
                                    "Found Action[{}::{}] for event from Trigger[{}::{}]",
                                    output_id,
                                    action_id,
                                    triggered_event.input,
                                    triggered_event.trigger
                                );
                                let actionable_event = ActionableEvent {
                                    output: output_id.clone(),
                                    action: action_id.clone(),
                                    data: triggered_event.to_owned().data,
                                };
                                (tx, actionable_event)
                            })
                            .map(|(tx, actionable_event)| tx.send(actionable_event));

                        join_all(send_futures)
                            .await
                            .iter()
                            .filter_map(|x| x.as_ref().err())
                            .for_each(|err| error!("Error sending ActionableEvent: {:?}", err));
                    }
                }
            });
        }
    }
}
