#![allow(clippy::unnecessary_unwrap)]
use crate::consts::*;
use crate::dialog;
use crate::logging::*;
use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use serde_json::Value;
use std::io::{Error, ErrorKind};
use zip::ZipArchive;

lazy_static! {
    pub static ref CURRENT_VERSION: Mutex<String> =
        Mutex::new(get_current_version().expect("Could not determine current version!"));
}

#[derive(Debug)]
pub struct Release {
    pub url: String,
    pub tag: String,
    pub published_at: String,
}

impl Release {
    /// Downloads and installs the release
    pub fn install(self: &Release) -> Result<()> {
        info!("Installing asset from URL: {}", &self.url);
        let response = minreq::get(&self.url)
            .with_header("User-Agent", "UltimateTrainingModpack")
            .with_header("Accept", "application/octet-stream")
            .send_lazy()?;
        info!(
            "Ok response from Github. Status Code: {}",
            &response.status_code
        );
        let mut vec = Vec::new();
        for result in response {
            let (byte, length) = result?;
            vec.reserve(length);
            vec.push(byte);
        }
        info!("Finished receiving .zip file from GitHub.");
        info!("Unpacking .zip file...");
        let mut zip = ZipArchive::new(std::io::Cursor::new(vec))?;
        zip.extract(UNPACK_PATH)?;
        info!("Finished unpacking update");

        info!("Updating config file with last update time...");
        TrainingModpackConfig::change_last_update_version(&self.published_at)?;
        dialog::dialog_ok(
            "The Training Modpack has been updated.\n\n\
            Your game will now restart."
                .to_string(),
        );
        info!("Finished. Restarting...");
        unsafe {
            skyline::nn::oe::RequestToRelaunchApplication();
        }
        // Don't need a return type here because this area is unreachable
    }

    pub fn to_string(self: &Release) -> String {
        format!("{} - {}", self.tag, self.published_at)
    }

    pub fn is_older_than_installed(self: &Release) -> bool {
        // String comparison is good enough because for RFC3339 format,
        // alphabetical order == chronological order
        //
        // https://datatracker.ietf.org/doc/html/rfc3339#section-5.1
        let current_version = CURRENT_VERSION.lock();
        self.published_at.as_str() < current_version.as_str()
    }
}

/// Attempts to load the update policy from file
/// If the file does not exist, asks the user for their preference and saves it
fn get_update_policy() -> UpdatePolicy {
    let config =
        TrainingModpackConfig::load_or_create().expect("Could not load or create config file!");
    let policy = config.update.policy;
    policy.unwrap_or_else(|| {
        warn!("Update policy not found. Asking user for their preference...");
        let p = ask_for_update_policy();
        TrainingModpackConfig::change_update_policy(&p).expect("Could not change update policy!");
        p
    })
}

/// Asks the user what the update policy should be
/// Emulator users automatically returns UpdatePolicy::Stable
fn ask_for_update_policy() -> UpdatePolicy {
    if dialog::yes_no(
        "Would you like to receive automatic updates for the Training Modpack?".to_string(),
        true,
    ) {
        match dialog::left_right(
            "Which sort of automatic updates would you like to receive?".to_string(),
            "Stable".to_string(),
            "Beta".to_string(),
            "Disabled".to_string(),
            "Stable".to_string(),
        )
        .as_str()
        {
            "Stable" => UpdatePolicy::Stable,
            "Beta" => UpdatePolicy::Beta,
            _ => UpdatePolicy::Disabled,
        }
    } else {
        UpdatePolicy::Disabled
    }
}

fn get_release(beta: bool) -> Result<Release> {
    // Get the list of releases from Github
    let url = format!(
        "https://api.github.com/repos/{}/{}/releases",
        env!("AUTHOR"),
        env!("REPO_NAME")
    );
    let response = minreq::get(url)
        .with_header("User-Agent", env!("USER_AGENT"))
        .with_header("Accept", "application/json")
        .send()?;

    let json: Vec<Value> = serde_json::from_str(response.as_str()?)?;

    // Parse the list to determine the latest stable and beta release
    let mut stable_release: Option<Release> = None;
    let mut beta_release: Option<Release> = None;
    for release in json.into_iter() {
        // The list is ordered by date w/ most recent releases first
        // so we only need to get the first of each type
        let is_prerelease = release["prerelease"]
            .as_bool()
            .ok_or_else(|| anyhow!("prerelease is not a bool"))?;
        if is_prerelease && beta_release.is_none() {
            // Assumes that the first asset exists and is the right one
            let url = release["assets"][0]["url"]
                .as_str()
                .ok_or_else(|| anyhow!("Could not parse beta asset url"))?;
            let tag = release["tag_name"]
                .as_str()
                .ok_or_else(|| anyhow!("Could not parse beta asset tag_name"))?;
            let published_at = release["published_at"]
                .as_str()
                .ok_or_else(|| anyhow!("Could not parse beta asset published_at"))?;
            beta_release = Some(Release {
                url: url.to_string(),
                tag: tag.to_string(),
                published_at: published_at.to_string(),
            });
        } else if !is_prerelease && stable_release.is_none() {
            // Assumes that the first asset exists and is the right one
            let url = release["assets"][0]["url"]
                .as_str()
                .ok_or_else(|| anyhow!("Could not parse stable asset url"))?;
            let tag = release["tag_name"]
                .as_str()
                .ok_or_else(|| anyhow!("Could not parse stable asset tag_name"))?;
            let published_at = release["published_at"]
                .as_str()
                .ok_or_else(|| anyhow!("Could not parse stable asset published_at"))?;
            stable_release = Some(Release {
                url: url.to_string(),
                tag: tag.to_string(),
                published_at: published_at.to_string(),
            });
        }
        if beta_release.is_some() && stable_release.is_some() {
            // Don't iterate needlessly, we already found both releases
            break;
        }
    }
    if beta && beta_release.is_some() {
        Ok(beta_release.unwrap())
    } else if !beta && stable_release.is_some() {
        Ok(stable_release.unwrap())
    } else {
        Err(anyhow!(
            "The specified release was not found in the GitHub JSON response!"
        ))
    }
}

fn user_wants_to_install() -> bool {
    dialog::yes_no(
        "There is a new update available for the Training Modpack. \n\n\
        Do you want to install it?"
            .to_string(),
        true,
    )
}

fn get_current_version() -> Result<String> {
    let config = TrainingModpackConfig::load();
    match config {
        Ok(c) => {
            info!("Config file found and parsed. Loading...");
            Ok(c.update.last_update_version)
        }
        Err(e)
            if e.is::<Error>()
                && e.downcast_ref::<Error>().unwrap().kind() == ErrorKind::NotFound =>
        {
            warn!("No config file found, creating default...");
            TrainingModpackConfig::create_default()?;
            get_current_version()
        }
        Err(e) => {
            // Some other error, re-raise it
            Err(e)
        }
    }
}

pub fn perform_version_check() {
    let update_policy = get_update_policy();
    info!("Update Policy is {}", update_policy.to_str());
    let mut release_to_apply = match update_policy {
        UpdatePolicy::Stable => get_release(false),
        UpdatePolicy::Beta => get_release(true),
        UpdatePolicy::Disabled => {
            // User does not want to update at all
            Err(anyhow!("Updates are disabled per UpdatePolicy"))
        }
    };
    if release_to_apply.is_ok() {
        let published_at = release_to_apply.as_ref().unwrap().published_at.clone();
        let current_version = CURRENT_VERSION.lock();
        info!("Current version: {}", current_version);
        info!("Github  version: {}", published_at);
        drop(current_version); // Explicitly unlock, since we also acquire a lock in is_older_than_installed()
        if release_to_apply.as_ref().unwrap().is_older_than_installed() {
            release_to_apply = Err(anyhow!(
                "Update is older than the current installed version.",
            ))
        }
    }

    // Perform Update
    match release_to_apply {
        Ok(release) => {
            if user_wants_to_install() {
                info!("Installing update: {}", &release.to_string());
                if let Err(e) = release.install() {
                    error!("Failed to install the update. Reason: {:?}", e);
                }
            } else {
                info!("User declined the update.");
            }
        }
        Err(e) => {
            warn!("Did not install update. Reason: {:?}", e);
        }
    }
}
