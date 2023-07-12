use std::fs;

use skyline_web::DialogOk;

use crate::consts::{
    LEGACY_MENU_OPTIONS_PATH, MENU_DEFAULT_OPTIONS_PATH, MENU_OPTIONS_PATH, VERSION_TXT_PATH,
};
use crate::logging::*;

pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

enum VersionCheck {
    Current,
    NoFile,
    Update,
}

fn is_current_version(fpath: &str) -> VersionCheck {
    // Create a blank version file if it doesn't exists
    if fs::metadata(fpath).is_err() {
        fs::File::create(fpath).expect("Could not create version file!");
        return VersionCheck::NoFile;
    }

    if fs::read_to_string(fpath).unwrap_or_else(|_| "".to_string()) == CURRENT_VERSION {
        VersionCheck::Current
    } else {
        VersionCheck::Update
    }
}

fn record_current_version(fpath: &str) {
    // Write the current version to the version file
    fs::write(fpath, CURRENT_VERSION).expect("Could not record current version!")
}

pub fn version_check() {
    match is_current_version(VERSION_TXT_PATH) {
        VersionCheck::Current => {
            // Version is current, no need to take any action
        }
        VersionCheck::Update => {
            // Display dialog box on launch if changing versions
            DialogOk::ok(
                format!(
                    "Thank you for installing version {CURRENT_VERSION} of the Training Modpack.\n\n\
                    Due to a breaking change in this version, your menu selections and defaults must be reset once.\n\n\
                    Please refer to the Github page and the Discord server for a full list of recent features, bugfixes, and other changes."
                )
            );
            // Remove old menu selections, silently ignoring errors (i.e. if the file doesn't exist)
            [
                MENU_OPTIONS_PATH,
                MENU_DEFAULT_OPTIONS_PATH,
                LEGACY_MENU_OPTIONS_PATH,
            ]
            .iter()
            .for_each(|path| {
                fs::remove_file(path).unwrap_or_else(|_| error!("Couldn't remove {path}"))
            });
            record_current_version(VERSION_TXT_PATH);
        }
        VersionCheck::NoFile => {
            // Display dialog box on fresh installation
            DialogOk::ok(
                format!(
                    "Thank you for installing version {CURRENT_VERSION} of the Training Modpack.\n\n\
                    Please refer to the Github page and the Discord server for a full list of features and instructions on how to utilize the improved Training Mode."
                )
            );
            record_current_version(VERSION_TXT_PATH);
        }
    }
}
