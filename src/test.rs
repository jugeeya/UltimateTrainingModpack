/// Recommended to run with the following command:
/// `RUSTFLAGS=-Awarnings cargo test --target=x86_64-unknown-linux-gnu`
/// But you can replace the target with your PC's target.

/// This will run and render the default menu in your default HTML opening program, ideally Chrome.
#[test]
fn write_menu() {
    unsafe {
        use crate::common::menu::write_menu;
        use std::process::Command;

        let folder_path = "../trainingmodpack.htdocs";
        let path = "../trainingmodpack.htdocs/training_menu.html";

        assert!(
            std::path::Path::new(folder_path).exists(),
            "Needs required folder: ../trainingmodpack.htdocs!"
        );

        std::fs::write(path, write_menu()).unwrap();

        let (cmd, args) = if wsl::is_wsl() || cfg!(target_os = "windows") {
            ("cmd.exe", ["/C", "start", path])
        } else {
            ("sh", ["-c", "open", path])
        };

        Command::new(cmd)
            .args(&args)
            .output()
            .expect("failed to open rendered HTML file");
    }
}
