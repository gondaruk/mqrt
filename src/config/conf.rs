use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

use crate::common::data::ElId;
use crate::common::types::Result as AsyncResult;
use crate::config::handler::HandlerConfig;
use crate::config::input::InputConfig;
use crate::config::output::OutputConfig;
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Config {
    #[serde(rename = "input")]
    #[serde(default)]
    pub inputs: HashMap<ElId, InputConfig>,

    #[serde(rename = "output")]
    #[serde(default)]
    pub outputs: HashMap<ElId, OutputConfig>,

    #[serde(rename = "handler")]
    #[serde(default)]
    pub handlers: Vec<HandlerConfig>,
}

impl Config {
    pub(crate) fn load(config_file: &str) -> AsyncResult<Self> {
        let mut data = String::new();

        debug!("Reading config from {:?}", config_file);

        let mut file = File::open(config_file)?;
        file.read_to_string(&mut data)?;

        let config: Config = toml::from_str(&data)?;

        Ok(config)
    }

    pub fn dump_to_string(&self) -> AsyncResult<String> {
        let data = toml::to_string(&self)?;
        Ok(data)
    }

    pub fn dump_to_file(&self, config_file: &str) -> AsyncResult<()> {
        let data = toml::to_string(&self)?;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(config_file)
            .unwrap();
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}
