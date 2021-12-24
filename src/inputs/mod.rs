pub mod mqtt;

use crate::common::data::TriggeredEvent;
use async_trait::async_trait;
use tokio::sync::mpsc::Sender;

#[async_trait]
pub trait InputTask: std::fmt::Debug + std::fmt::Display + Send + Sync {
    async fn run(self: Box<Self>, chan: Sender<TriggeredEvent>);
}
