use skyline_web::DialogOk;
use std::fs;

pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const VERSION_FILE_PATH: &str = "sd:/TrainingModpack/version.txt";

fn is_current_version(fpath: &str) -> Option<bool> {
    // Returns:
    //     Some(true) if it is the current version
    //     Some(false) if it is upgrading from a prior version
    //     None if it is a fresh install (i.e. no prior version.txt exists)

    // Create a blank version file if it doesn't exists
    if fs::metadata(fpath).is_err() {
        fs::File::create(fpath).expect("Could not create version file!");
        return None;
    }

    Some(
        fs::read_to_string(fpath)
            .map(|content| content == CURRENT_VERSION)
            .unwrap_or(false),
    )
}

fn record_current_version(fpath: &str) {
    // Write the current version to the version file
    fs::write(fpath, CURRENT_VERSION).expect("Could not record current version!")
}

pub fn version_check() {
    match is_current_version(VERSION_FILE_PATH) {
        Some(true) => {
            // Version is current, no need to take any action
        }
        Some(false) => {
            // Display dialog box on launch if changing versions
            DialogOk::ok(
                format!(
                    "Thank you for installing version {} of the Training Modpack.\n\n\
                    Due to a breaking change in this version, your menu selections and defaults must be reset once.\n\n\
                    Please refer to the Github page and the Discord server for a full list of recent features, bugfixes, and other changes.",
                    CURRENT_VERSION
                )
            );
            // Remove old menu selections, silently ignoring errors (i.e. if the file doesn't exist)
            fs::remove_file("sd:/TrainingModpack/training_modpack_menu.conf").unwrap_or({});
            fs::remove_file("sd:/TrainingModpack/training_modpack_menu_defaults.conf")
                .unwrap_or({});
            record_current_version(VERSION_FILE_PATH);
        }
        None => {
            // Display dialog box on fresh installation
            DialogOk::ok(
                format!(
                    "Thank you for installing version {} of the Training Modpack.\n\n\
                    Please refer to the Github page and the Discord server for a full list of features and instructions on how to utilize the improved Training Mode.",
                    CURRENT_VERSION
                )
            );
            record_current_version(VERSION_FILE_PATH);
        }
    }
}
