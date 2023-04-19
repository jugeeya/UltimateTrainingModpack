use crate::consts::*;
use crate::logging::*;
use minreq;
use regex::Regex;
use serde_json::Value;
use std::fs;
use std::io::Cursor;
use toml;
use zip::ZipArchive;
use skyline::nn::swkbd;

pub const CURRENT_VERSION: &str = include_str!("hash.txt"); // VERSION_TXT_PATH?

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
    pub commit: String,
    pub published_at: String,
}

impl Release {
    /// Downloads and installs the release
    pub fn install(self: &Release) {
        let asset = minreq::get(&self.url)
            .with_header("User-Agent", env!("USER_AGENT"))
            .with_header("Accept", "application/octet-stream")
            .send()
            .expect("Could not fetch asset from Github!")
            .into_bytes();
        let mut zip = ZipArchive::new(Cursor::new(asset)).unwrap();
        zip.extract(UNPACK_PATH)
            .expect("Could not unzip asset");
        unsafe {
            skyline::nn::oe::RequestToRelaunchApplication();
        }
    }
}

#[derive(Debug)]
pub struct CurrentReleases {
    pub stable: Option<Release>,
    pub beta: Option<Release>,
}

impl CurrentReleases {
    // Fetches the latest stable and beta release from Github
    pub fn get() -> Result<CurrentReleases, minreq::Error> {
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
        let json: Vec<Value> = match serde_json::from_str(response.as_str()?) {
            Ok(response) => response,
            Err(_) => return Err(minreq::Error::Other("Failed to parse Github JSON Response")),
        };

        // Parse the list to determine the latest stable and beta release
        let mut stable_release: Option<Release> = None;
        let mut beta_release: Option<Release> = None;
        for release in json.into_iter() {
            // The list is ordered by date w/ most recent releases first
            // so we only need to get the first of each type
            let is_prerelease = release["prerelease"].as_bool().unwrap();
            if is_prerelease && beta_release.is_none() {
                let commit = parse_body_for_commit(release["body"].as_str().unwrap());
                // Assumes that the first asset exists and is the right one
                // TODO: Make this more robust
                let url = release["assets"][0]["url"].as_str().unwrap();
                let tag = release["tag_name"].as_str().unwrap();
                let published_at = release["published_at"].as_str().unwrap();
                beta_release = Some(Release {
                    url: url.to_string(),
                    tag: tag.to_string(),
                    commit: commit.unwrap_or("No commit found").to_string(),
                    published_at: published_at.to_string(),
                });
            } else if !is_prerelease && stable_release.is_none() {
                let commit = parse_body_for_commit(release["body"].as_str().unwrap());
                // Assumes that the first asset exists and is the right one
                // TODO: Make this more robust
                let url = release["assets"][0]["url"].as_str().unwrap();
                let tag = release["tag_name"].as_str().unwrap();
                let published_at = release["published_at"].as_str().unwrap();
                stable_release = Some(Release {
                    url: url.to_string(),
                    tag: tag.to_string(),
                    commit: commit.unwrap_or("No commit found").to_string(),
                    published_at: published_at.to_string(),
                });
            }
            if beta_release.is_some() && stable_release.is_some() {
                // Don't iterate needlessly
                break;
            }
        }
        Ok(CurrentReleases {
            stable: stable_release,
            beta: beta_release,
        })
    }
}

pub fn remove_legacy_files() {
    // TODO
}

/// Return the git commit of the release from the release page body
/// Requires a 40-character hex commit somewhere in the body, e.g.:
/// Commit: ec8587aa13379de3acdf142b66eadeb02972d9a1
fn parse_body_for_commit(body: &str) -> Option<&str> {
    let commit_format = Regex::new(r"[0-9a-fA-F]{40}").unwrap();
    match commit_format.find(body) {
        Some(x) => Some(x.as_str()),
        None => None
    }
}

/// Ask the user if they'd like to install the update
fn user_wants_to_install() -> bool {
    // TODO: investigate restricting the keyboard set
    // let response = swkbd::ShowKeyboardArg::new()
    //     .header_text("There is a new update available. Would you like to install it?\nyes (y)/no (n)")
    //     .show();
    let response = Some("y"); // TODO: Remove this
    if response.is_some() {
        match response.unwrap() {
            "y" => true,
            "n" => false,
            _ => user_wants_to_install(),
        }
    } else {
        user_wants_to_install()
    }
}

/// Attempt to load UpdatePolicy from file.
/// If it cannot load from file, ask the user
fn get_update_policy() -> UpdatePolicy {
    if fs::metadata(TRAINING_MODPACK_TOML_PATH).is_ok() {
        info!("Previous config toml file found. Loading...");
        let toml_contents = fs::read_to_string(TRAINING_MODPACK_TOML_PATH)
            .unwrap_or_else(|_| panic!("Could not read {}", TRAINING_MODPACK_TOML_PATH));
        let mut conf: toml::Table = (&toml_contents).parse::<toml::Table>().unwrap();
        // TODO: gracefully handle nested lookup
        let policy_str = conf["update"]["UpdatePolicy"].as_str().unwrap_or("");
        let policy = UpdatePolicy::from_str(policy_str);
        
        if policy.is_some() {
            info!("Found UpdatePolicy: {}", policy_str);
            policy.unwrap()
        } else {
            info!("Asking for update policy");
            let p = prompt_update_policy();
            conf["update"]["UpdatePolicy"] = p.to_str().into();
            // TODO: use toml_edit crate instead to preserve comments
            fs::write(
                TRAINING_MODPACK_TOML_PATH,
                toml::to_string_pretty(&conf).unwrap(),
            ).expect("Could not write update policy in config file!");
            p
        }
    } else {
        error!(
            "Config toml metadata not ok: {}",
            TRAINING_MODPACK_TOML_PATH
        );
        UpdatePolicy::default()
    }
}

/// Ask the user for the UpdatePolicy
fn prompt_update_policy() -> UpdatePolicy {
    // let response = nn::swkbd::ShowKeyboardArg::new()
    //     .header_text("Which updates would you like to receive?\nbeta (b)/stable (s)/disabled (d)")
    //     .show();
    let response = Some("d"); // TODO: Remove this
    if let Some(r) = response {
        match r {
            "b" => UpdatePolicy::Beta,
            "s" => UpdatePolicy::Stable,
            "d" => UpdatePolicy::Disabled,
            _ => prompt_update_policy(),
        }
    } else {
        UpdatePolicy::default()
    }
}

/// Determine which update should be applied, if any
fn select_update_to_apply(
    update_policy: UpdatePolicy,
    local_version: &str,
    current_releases: CurrentReleases,
) -> Option<Release> {
    let update_to_apply = match update_policy {
        UpdatePolicy::Stable => {
            let stable_available = if let Some(ref stable_release) = current_releases.stable {
                &stable_release.commit != &local_version
            } else {
                false
            };
            info!("Stable Version Available: {}", &stable_available);
            if stable_available {
                current_releases.stable
            } else {
                None
            }
        }
        UpdatePolicy::Beta => {
            let beta_available = if let Some(ref beta_release) = current_releases.beta {
                &beta_release.commit != &local_version
            } else {
                false
            };
            info!("Beta Version Available: {}", &beta_available);
            if beta_available {
                current_releases.beta
            } else {
                None
            }
        }
        UpdatePolicy::Disabled => None,
    };
    update_to_apply
}

pub fn version_check() {
    let update_policy = get_update_policy();
    info!("Update Policy is {}", update_policy.to_str());
    if update_policy == UpdatePolicy::Disabled {
        // User does not want to update at all
        info!("Updates are disabled per UpdatePolicy");
        return;
    }

    // Compare commit hashes
    let local_version = include_str!("hash.txt");
    let current_releases = CurrentReleases::get();
    if !current_releases.is_ok() {
        error!("Couldn't get current releases from Github, or the release body isn't properly formatted!");
        return;
    }

    let current_releases = current_releases.unwrap();
    let update_to_apply = select_update_to_apply(update_policy, local_version, current_releases);

    // Perform Update
    if let Some(update) = update_to_apply {
        if user_wants_to_install() {
            info!("Installing update: {}", &update.commit);
            update.install();
        }
    }
}
