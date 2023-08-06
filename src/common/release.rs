use crate::consts::*;
use crate::logging::*;
use zip::ZipArchive;
use std::io::Cursor;
use serde_json::Value;

pub const CURRENT_VERSION: &str = "stable";

#[derive(PartialEq)]
enum UpdatePolicy {
    Stable,
    Beta,
    Disabled,
}

impl UpdatePolicy {
    fn default() -> UpdatePolicy {
        UpdatePolicy::Stable
    }

    fn from_str(s: &str) -> Option<UpdatePolicy> {
        match s.to_lowercase().as_ref() {
            "stable" => Some(UpdatePolicy::Stable),
            "beta" => Some(UpdatePolicy::Beta),
            "disabled" => Some(UpdatePolicy::Disabled),
            _ => None,
        }
    }

    fn to_str(self: &UpdatePolicy) -> &str {
        match self {
            UpdatePolicy::Stable => "stable",
            UpdatePolicy::Beta => "beta",
            UpdatePolicy::Disabled => "disabled",
        }
    }
}

#[derive(Debug)]
pub struct Release {
    pub url: String,
    pub tag: String,
    pub published_at: String,
}

impl Release {
    /// Downloads and installs the release
    // TODO: switch off of minreq to ureq or reqwest?
    pub fn install(self: &Release) {
        info!("Installing {}", &self.to_string());
        info!("URL: {}", &self.url);
        let asset = minreq::get(&self.url)
            .with_header("User-Agent", env!("USER_AGENT"))
            .with_header("Accept", "application/octet-stream")
            .send()
            .expect("Could not fetch asset from Github!")
            .into_bytes();
        let mut zip = ZipArchive::new(Cursor::new(asset)).unwrap();
        zip.extract(UNPACK_PATH)
            .expect("Could not unzip asset");
        // unsafe {
        //     skyline::nn::oe::RequestToRelaunchApplication();
        // }
    }

    pub fn to_string(self: &Release) -> String {
        format!("{} - {}", self.tag, self.published_at)
    }
}

fn get_update_policy() -> UpdatePolicy {
    // TODO
    UpdatePolicy::default()
}

fn get_release(beta: bool) -> Option<Release> {
        // Get the list of releases from Github
        // let url = format!(
        //     "https://api.github.com/repos/{}/{}/releases",
        //     env!("AUTHOR"),
        //     env!("REPO_NAME")
        // );
        // let response = minreq::get(url)
        //     .with_header("User-Agent", env!("USER_AGENT"))
        //     .with_header("Accept", "application/json")
        //     .send();
        // if response.is_err() {
        //     error!("{}", response.unwrap_err());
        //     return None;
        // }
        let response_text = include_str!("release_example.json");
        //let json: Vec<Value> = match serde_json::from_str(response.unwrap().as_str().unwrap()) {
        let json: Vec<Value> = match serde_json::from_str(response_text) {
            Ok(response) => response,
            Err(_) => {
                error!("Failed to parse Github JSON Response");
                return None;
            }
        };

        // Parse the list to determine the latest stable and beta release
        let mut stable_release: Option<Release> = None;
        let mut beta_release: Option<Release> = None;
        for release in json.into_iter() {
            // The list is ordered by date w/ most recent releases first
            // so we only need to get the first of each type
            let is_prerelease = release["prerelease"].as_bool().unwrap();
            if is_prerelease && beta_release.is_none() {
                // Assumes that the first asset exists and is the right one
                let url = release["assets"][0]["url"].as_str().unwrap();
                let tag = release["tag_name"].as_str().unwrap();
                let published_at = release["published_at"].as_str().unwrap();
                beta_release = Some(Release {
                    url: url.to_string(),
                    tag: tag.to_string(),
                    published_at: published_at.to_string(),
                });
            } else if !is_prerelease && stable_release.is_none() {
                // Assumes that the first asset exists and is the right one
                let url = release["assets"][0]["url"].as_str().unwrap();
                let tag = release["tag_name"].as_str().unwrap();
                let published_at = release["published_at"].as_str().unwrap();
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
            Some(beta_release.unwrap())
        } else if !beta && stable_release.is_some() {
            Some(stable_release.unwrap())
        } else {
            error!("The specified release was not found in the GitHub JSON response!");
            None
        }
}

fn user_wants_to_install() -> bool {
    // TODO
    true
}

pub fn version_check() {
    let update_policy = get_update_policy();
    info!("Update Policy is {}", update_policy.to_str());
    let release_to_apply: Option<Release> = match update_policy {
        UpdatePolicy::Stable => {get_release(false)},
        UpdatePolicy::Beta => {get_release(true)},
        UpdatePolicy::Disabled => {
            // User does not want to update at all
            info!("Updates are disabled per UpdatePolicy");
            None
        },
    };


    // Perform Update
    if let Some(release) = release_to_apply {
        if user_wants_to_install() {
            info!("Installing update: {}", &release.to_string());
            release.install();
        }
    } else { error!("Could not get release from github!"); }
}