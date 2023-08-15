use crate::consts::*;
use crate::logging::*;
use anyhow::{anyhow, Result};
use serde_json::Value;
use std::io::{Error, ErrorKind};
use zip::ZipArchive;

pub const CURRENT_VERSION: &str = "1979-05-27T07:32:00Z";

#[derive(Debug)]
pub struct Release {
    pub url: String,
    pub tag: String,
    pub published_at: String,
}

impl Release {
    /// Downloads and installs the release
    pub fn install(self: &Release) -> Result<()> {
        info!("URL: {}", &self.url);
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
        let mut zip = ZipArchive::new(std::io::Cursor::new(vec))?;
        zip.extract(UNPACK_PATH)?;
        info!("Installed, restarting...");
        unsafe {
            skyline::nn::oe::RequestToRelaunchApplication();
        }
        Ok(()) // Unreachable but whatever
    }

    pub fn to_string(self: &Release) -> String {
        format!("{} - {}", self.tag, self.published_at)
    }
}

/// Attempts to load the update policy from file
/// If the file does not exist, creates a default and loads that
fn get_update_policy() -> Result<UpdatePolicy> {
    let config = TrainingModpackConfig::load();
    match config {
        Ok(c) => {
            info!("Config file found and parsed. Loading...");
            Ok(c.update.policy)
        }
        Err(e)
            if e.is::<Error>()
                && e.downcast_ref::<Error>().unwrap().kind() == ErrorKind::NotFound =>
        {
            warn!("No config file found, creating default...");
            TrainingModpackConfig::create_new()?;
            get_update_policy()
        }
        Err(e) => {
            // Some other error, re-raise it
            Err(e)
        },
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
            // Don't iterate needlessly
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
    // TODO
    true
}

pub fn perform_version_check() {
    let update_policy = get_update_policy().expect("Could not get update policy!");
    info!("Update Policy is {}", update_policy.to_str());
    let release_to_apply = match update_policy {
        UpdatePolicy::Stable => get_release(false),
        UpdatePolicy::Beta => get_release(true),
        UpdatePolicy::Disabled => {
            // User does not want to update at all
            Err(anyhow!("Updates are disabled per UpdatePolicy"))
        }
    };

    // Perform Update
    match release_to_apply {
        Ok(release) => {
            if user_wants_to_install() {
                info!("Installing update: {}", &release.to_string());
                if let Err(e) = release.install() {
                    error!("Failed to install the update. Reason: {:?}", e);
                }
            }
        }
        Err(e) => {
            error!("Could not get release from github! Reason: {:?}", e);
        }
    }
}
