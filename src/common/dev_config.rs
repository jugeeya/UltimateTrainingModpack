use std::fs;

use serde::Deserialize;

use crate::common::input::*;
use crate::consts::DEV_TOML_PATH;
use crate::logging::info;
use training_mod_sync::*;

/// Hot-reloadable configs for quicker development
///
/// In game, press L+R+A at any point to reread these configs from
/// the file in DEV_TOML_PATH on the SD card
///
/// Example usage:
///
/// In this file:
/// ```rust
/// pub struct DevConfig {
///     pub quit_menu_title: String,
///     pub quit_menu_pos_y: i32,
/// }
/// ```
///
/// In another file such as `ui2d/menu.rs`:
/// ```rust
/// let dev_config = crate::dev_config::config();
/// quit_menu_button.pos_y = dev_config.quit_menu_pos_y;
/// quit_menu_text.as_textbox().set_text_string(&dev_config.quit_menu_title);
/// ```
#[derive(Deserialize, Default, Clone)]
pub struct DevConfig {}

pub unsafe fn config() -> DevConfig {
    read_clone(&(*DEV_CONFIG))
}

pub static DEV_CONFIG: LazyLock<RwLock<DevConfig>> =
    LazyLock::new(|| RwLock::new(DevConfig::load_from_toml()));

impl DevConfig {
    fn load_from_toml() -> DevConfig {
        let dev_path = DEV_TOML_PATH;
        if fs::metadata(dev_path).is_ok() {
            info!("Loading dev.toml configs...");
            let dev_config_str = fs::read_to_string(dev_path)
                .unwrap_or_else(|_| panic!("Could not read {}", dev_path));
            return toml::from_str(&dev_config_str).expect("Could not parse dev config");
        }

        DevConfig::default()
    }
}

pub fn handle_final_input_mapping(player_idx: i32, controller_struct: &SomeControllerStruct) {
    let current_buttons = controller_struct.controller.current_buttons;
    if player_idx == 0 && current_buttons.l() && current_buttons.r() && current_buttons.a() {
        assign(&(*DEV_CONFIG), DevConfig::load_from_toml());
    }
}
