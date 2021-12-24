pub mod mqtt;

use crate::common::data::ActionableEvent;
use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;

#[async_trait]
pub trait OutputTask: std::fmt::Debug + std::fmt::Display + Send + Sync {
    async fn run(self: Box<Self>, chan: Receiver<ActionableEvent>);
}
