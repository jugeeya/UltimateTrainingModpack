use lazy_static::lazy_static;
use parking_lot::Mutex;
use serde::Deserialize;
use skyline::nn::hid::NpadGcState;
use crate::logging::info;
use std::fs;
use toml;


/// Hot-reloadable configs for quicker development
///
/// In game, press L+R+A at any point to reread these configs from
/// the file in sd:/TrainingModpack/dev.toml
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
#[derive(Deserialize, Default)]
pub struct DevConfig {
}

pub fn config() -> &'static DevConfig {
    &*DEV_CONFIG.data_ptr()
}

lazy_static! {
    pub static ref DEV_CONFIG : Mutex<DevConfig> = Mutex::new(DevConfig::load_from_toml());
    pub static ref DEV_CONFIG_STR : Mutex<String> = Mutex::new("".to_string());
}

impl DevConfig {
    fn load_from_toml() -> DevConfig {
        let dev_path = "sd:/TrainingModpack/dev.toml";
        if fs::metadata(dev_path).is_ok() {
            info!("Loading dev.toml configs...");
            let dev_config_str = fs::read_to_string(dev_path).unwrap_or_else(|_| panic!("Could not read {}", dev_path));
            return toml::from_str(&dev_config_str).expect("Could not parse dev config");
        }
        
        DevConfig::default()
    }
}

pub fn handle_get_npad_state(state: *mut NpadGcState, _controller_id: *const u32) {
    let a_press = 1 << 0;
    let l_press = 1 << 6;
    let r_press = 1 << 7;
    let buttons;
    unsafe {
        buttons = (*state).Buttons;
    }

    // Occurs on L+R+A
    if (buttons & a_press > 0) && (buttons & l_press > 0) && (buttons & r_press > 0) {
        let mut dev_config = DEV_CONFIG.lock();
        *dev_config = DevConfig::load_from_toml();
    }
}
