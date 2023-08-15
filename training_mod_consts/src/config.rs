use crate::files::*;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use toml::value::Datetime;

use std::fs;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

/// Top level struct which represents the entirety of the modpack config
/// (Does not include in-game menu settings)
/// Each field here is a section of training_modpack.toml
#[derive(Serialize, Deserialize, Debug)]
pub struct TrainingModpackConfig {
    pub update: UpdaterConfig,
}

impl TrainingModpackConfig {
    pub fn default() -> TrainingModpackConfig {
        TrainingModpackConfig {
            update: UpdaterConfig::default(),
        }
    }

    /// Attempts to load the config from file
    pub fn load() -> Result<TrainingModpackConfig> {
    if fs::metadata(TRAINING_MODPACK_TOML_PATH).is_ok() {
        let toml_config_str = fs::read_to_string(TRAINING_MODPACK_TOML_PATH)?;
        let parsed = toml::from_str::<TrainingModpackConfig> (&toml_config_str)?;
        Ok(parsed)
    } else {
        Err(Error::from(ErrorKind::NotFound).into())
    }}

    /// Creates a default config and saves to file
    /// Returns Err if the file already exists
    /// TODO!() Ask user for preference instead of using default
    pub fn create_new() -> Result<()> {
        if fs::metadata(TRAINING_MODPACK_TOML_PATH).is_ok() {
            Err(Error::from(ErrorKind::AlreadyExists).into())
        } else {
            let default_config: TrainingModpackConfig = TrainingModpackConfig::default();
            let contents = toml::to_string(&default_config)?;
            fs::write(TRAINING_MODPACK_TOML_PATH, contents)?;
            Ok(())
        }
    }
}

/// Config section for the automatic updater
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdaterConfig {
    pub policy: UpdatePolicy,
    pub last_update_version: Datetime,
}

impl UpdaterConfig {
    pub fn default() -> UpdaterConfig {
        UpdaterConfig {
            policy: UpdatePolicy::default(),
            last_update_version: Datetime::from_str("1970-01-01T00:00:00Z").unwrap(),
        }
    }
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub enum UpdatePolicy {
    Stable,
    Beta,
    Disabled,
}

impl UpdatePolicy {
    pub fn default() -> UpdatePolicy {
        UpdatePolicy::Stable
    }

    pub fn to_str(self: &UpdatePolicy) -> &str {
        match self {
            UpdatePolicy::Stable => "Stable",
            UpdatePolicy::Beta => "Beta",
            UpdatePolicy::Disabled => "Disabled",
        }
    }
}