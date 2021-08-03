use std::fs;
use std::io::Write;
use skyline_web::DialogOk;
const CURRENT_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const VERSION_FILE_PATH: &str = "sd:/TrainingModpack/version.txt";

fn is_current_version(fpath: &str) -> bool {
    // Create a blank version file if it doesn't exists
    if fs::metadata(fpath).is_err() {
        let _ = fs::File::create(fpath).expect("Could not create version file!");
    }
    let content = fs::read_to_string(fpath).unwrap_or("".to_string());
    content == CURRENT_VERSION
}

fn record_current_version(fpath: &str) {
    // Write the current version to the version file
    let mut f = fs::File::create(fpath).unwrap();
    write!(f, "{}", CURRENT_VERSION.to_owned()).expect("Could not record current version!");
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