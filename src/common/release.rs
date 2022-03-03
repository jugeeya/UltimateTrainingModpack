use skyline_web::DialogOk;
use std::fs;

pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const VERSION_FILE_PATH: &str = "sd:/TrainingModpack/version.txt";

fn is_current_version(fpath: &str) -> bool {
    // Create a blank version file if it doesn't exists
    if fs::metadata(fpath).is_err() {
        let _ = fs::File::create(fpath).expect("Could not create version file!");
    }

    fs::read_to_string(fpath)
        .map(|content| content == CURRENT_VERSION)
        .unwrap_or(false)
}

fn record_current_version(fpath: &str) {
    // Write the current version to the version file
    fs::write(fpath, CURRENT_VERSION).expect("Could not record current version!")
}

pub fn version_check() {
    // Display dialog box on launch if changing versions
    if !is_current_version(VERSION_FILE_PATH) {
        DialogOk::ok(
            format!(
                "Thank you for installing version {} of the Training Modpack.\n\n\
                This version includes a change to the menu button combination, which is now SPECIAL+UPTAUNT.\n\
                Please refer to the Github page and the Discord server for a full list of recent changes.",
                CURRENT_VERSION
            )
        );
        record_current_version(VERSION_FILE_PATH);
    }
}
