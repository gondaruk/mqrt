use log::trace;
use std::time::Duration;
use structopt::StructOpt;
use tokio::runtime::Runtime;

use crate::config::opt::Opt;
use crate::config::Config;
use crate::coordinator::ChannelManager;

#[derive(Debug)]
pub struct Application {
    pub opt: Opt,
    pub config: Config,
    pub runtime: Runtime,
}

impl Application {
    pub fn new() -> Self {
        env_logger::init();

        let opt: Opt = Opt::from_args();

        let config = load_config(&opt.config_path);
        trace!("Loaded config from {}:\n{:#?}", &opt.config_path, config);

        let runtime = build_runtime();

        Self {
            opt,
            config,
            runtime,
        }
    }

    pub fn run(self) {
        let runtime = self.runtime;
        runtime.block_on(async move {
            ChannelManager::run(self.config.clone()).await;
            loop {
                // TODO: add signal handling to wait for it and gracefully exit
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}

fn build_runtime() -> Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("mqrt-worker")
        .build()
        .expect("Can not spawn runtime workers")
}

fn load_config(config_path: &str) -> Config {
    Config::load(config_path).unwrap_or_else(|_| panic!("Can not load config from {}", config_path))
}
