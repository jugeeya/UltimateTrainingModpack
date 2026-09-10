use std::fs::read_to_string;
use std::sync::mpsc::{self, Sender};
use std::sync::LazyLock;

use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};

use log::{error, info};

static WRITER: LazyLock<Sender<WriteMessage>> = LazyLock::new(|| {
    let (tx, rx) = mpsc::channel::<WriteMessage>();
    std::thread::spawn(move || {
        for job in rx {
            match std::fs::write(job.path, job.contents) {
                Ok(()) => info!("Saved data to {}", job.path),
                Err(e) => error!("Could not write data to {}: {e}", job.path),
            }
        }
    });
    tx
});

pub fn init() {
    LazyLock::force(&WRITER);
    info!("Initialized writer thread");
}

pub fn read_toml<T: DeserializeOwned>(path: &str) -> Result<T> {
    let contents = read_to_string(path)?;
    let parsed = toml::from_str::<T>(&contents)?;
    Ok(parsed)
}

pub fn read_json<T: DeserializeOwned>(path: &str) -> Result<T> {
    let contents = read_to_string(path)?;
    let parsed = serde_json::from_str::<T>(&contents)?;
    Ok(parsed)
}

struct WriteMessage {
    path: &'static str,
    contents: String,
}

pub fn write_toml<T: Serialize>(path: &'static str, data: &T) {
    match toml::to_string_pretty(data) {
        Ok(contents) => send_write_message(path, contents),
        Err(e) => error!("Could not serialize data for {path}: {e}"),
    }
}

pub fn write_json<T: Serialize>(path: &'static str, data: &T) {
    match serde_json::to_string_pretty(data) {
        Ok(contents) => send_write_message(path, contents),
        Err(e) => error!("Could not serialize data for {path}: {e}"),
    }
}

fn send_write_message(path: &'static str, contents: String) {
    if let Err(e) = WRITER.send(WriteMessage { path, contents }) {
        error!("Could not queue write to {path}: {e}");
    }
}
