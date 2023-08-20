use crate::files::*;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use skyline::nn::time;

use std::fs;
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

/// Top level struct which represents the entirety of the modpack config
/// (Does not include in-game menu settings)
/// Each field here is a section of training_modpack.toml
#[derive(Serialize, Deserialize)]
pub struct TrainingModpackConfig {
    pub update: UpdaterConfig,
}

impl TrainingModpackConfig {
    pub fn new() -> TrainingModpackConfig {
        TrainingModpackConfig {
            update: UpdaterConfig::default(),
        }
    }

    /// Attempts to load the config from file
    pub fn load() -> Result<TrainingModpackConfig> {
        if fs::metadata(TRAINING_MODPACK_TOML_PATH).is_ok() {
            let toml_config_str = fs::read_to_string(TRAINING_MODPACK_TOML_PATH)?;
            let parsed = toml::from_str::<TrainingModpackConfig>(&toml_config_str)?;
            Ok(parsed)
        } else {
            Err(io::Error::from(io::ErrorKind::NotFound).into())
        }
    }

    pub fn load_or_create() -> Result<TrainingModpackConfig> {
        match TrainingModpackConfig::load() {
            Ok(c) => Ok(c),
            Err(e) => {
                if e.is::<io::Error>()
                    && e.downcast_ref::<io::Error>().unwrap().kind() == io::ErrorKind::NotFound
                {
                    // No config file exists already
                    TrainingModpackConfig::create_default()?;
                    TrainingModpackConfig::load()
                } else if e.is::<toml::de::Error>() {
                    // A config file exists but its not in the right format
                    fs::remove_file(TRAINING_MODPACK_TOML_PATH)?;
                    TrainingModpackConfig::create_default()?;
                    TrainingModpackConfig::load()
                } else {
                    // Some other error, re-raise it
                    Err(e)
                }
            }
        }
    }

    /// Creates a default config and saves to file
    /// Returns Err if the file already exists
    pub fn create_default() -> Result<()> {
        if fs::metadata(TRAINING_MODPACK_TOML_PATH).is_ok() {
            Err(io::Error::from(io::ErrorKind::AlreadyExists).into())
        } else {
            let default_config: TrainingModpackConfig = TrainingModpackConfig::new();
            let contents = toml::to_string(&default_config)?;
            fs::write(TRAINING_MODPACK_TOML_PATH, contents)?;
            Ok(())
        }
    }

    pub fn change_last_update_version(last_update_version: &str) -> Result<()> {
        let mut config = TrainingModpackConfig::load()?;
        config.update.last_update_version = last_update_version.to_string();
        let contents = toml::to_string(&config)?;
        fs::write(TRAINING_MODPACK_TOML_PATH, contents)?;
        Ok(())
    }
}

/// Since we can't rely on most time based libraries, this is a seconds -> date/time string based on the `chrono` crates implementation
/// God bless blujay and Raytwo
/// https://github.com/Raytwo/ARCropolis/blob/9dc1d59d1e8a3dcac433b10a90bb5b3fabad6c00/src/logging.rs#L15-L49
fn format_time_string(seconds: u64) -> String {
    let leapyear = |year| -> bool { year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) };

    static YEAR_TABLE: [[u64; 12]; 2] = [
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
    ];

    let mut year = 1970;

    let seconds_in_day = seconds % 86400;
    let mut day_number = seconds / 86400;

    let sec = seconds_in_day % 60;
    let min = (seconds_in_day % 3600) / 60;
    let hours = seconds_in_day / 3600;
    loop {
        let year_length = if leapyear(year) { 366 } else { 365 };

        if day_number >= year_length {
            day_number -= year_length;
            year += 1;
        } else {
            break;
        }
    }
    let mut month = 0;
    while day_number >= YEAR_TABLE[if leapyear(year) { 1 } else { 0 }][month] {
        day_number -= YEAR_TABLE[if leapyear(year) { 1 } else { 0 }][month];
        month += 1;
    }

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year,
        month + 1,
        day_number + 1,
        hours,
        min,
        sec
    )
}

fn now_utc() -> String {
    unsafe {
        time::Initialize();
    }
    let current_epoch_seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    format_time_string(current_epoch_seconds)
}

/// Config section for the automatic updater
#[derive(Serialize, Deserialize, Clone)]
pub struct UpdaterConfig {
    pub last_update_version: String,
}

impl UpdaterConfig {
    pub fn default() -> UpdaterConfig {
        UpdaterConfig {
            last_update_version: now_utc(),
        }
    }
}
