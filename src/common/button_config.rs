use std::collections::HashMap;
use std::fs;

use crate::consts::TRAINING_MODPACK_TOML_PATH;

use lazy_static::lazy_static;
use log::info;
use serde::Deserialize;
use smash::app::lua_bind::ControlModule;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use toml;

lazy_static! {
    // Using the LuaConst names wasn't working for some reason...
    static ref BUTTON_MAPPING: HashMap<&'static str, i32> = HashMap::from([
        ("ATTACK", 0),  // *CONTROL_PAD_BUTTON_ATTACK
        ("SPECIAL", 1), // *CONTROL_PAD_BUTTON_SPECIAL
        ("SHIELD", 3), // *CONTROL_PAD_BUTTON_GUARD
        ("GRAB", 9), // *CONTROL_PAD_BUTTON_CATCH
        ("JUMP", 2), // *CONTROL_PAD_BUTTON_JUMP
        ("UPTAUNT", 5), // *CONTROL_PAD_BUTTON_APPEAL_HI
        ("DOWNTAUNT", 6), // *CONTROL_PAD_BUTTON_APPEAL_LW
        ("LEFTTAUNT", 7), // *CONTROL_PAD_BUTTON_APPEAL_S_L
        ("RIGHTTAUNT", 8), // *CONTROL_PAD_BUTTON_APPEAL_S_R
        ("SHARESTOCK", 0xD), // *CONTROL_PAD_BUTTON_STOCK_SHARE
        ("JUMPMINI", 0xA), // *CONTROL_PAD_BUTTON_JUMP_MINI
    ]);
}
static mut BUTTON_COMBO_CONFIG: BtnComboConfig = BtnComboConfig {
    open_menu: BtnList {
        hold: vec![],
        press: vec![],
    },
    save_state: BtnList {
        hold: vec![],
        press: vec![],
    },
    load_state: BtnList {
        hold: vec![],
        press: vec![],
    },
    previous_save_state_slot: BtnList {
        hold: vec![],
        press: vec![],
    },
    next_save_state_slot: BtnList {
        hold: vec![],
        press: vec![],
    },
};

#[derive(Debug, EnumIter, PartialEq)]
pub enum ButtonCombo {
    OpenMenu,
    SaveState,
    LoadState,
    PrevSaveStateSlot,
    NextSaveStateSlot,
}

#[derive(Deserialize, Default)]
struct BtnList {
    hold: Vec<String>,
    press: Vec<String>,
}

#[derive(Deserialize, Default)]
struct BtnComboConfig {
    open_menu: BtnList,
    save_state: BtnList,
    load_state: BtnList,
    previous_save_state_slot: BtnList,
    next_save_state_slot: BtnList,
}

#[derive(Deserialize)]
pub struct TopLevelBtnComboConfig {
    button_config: BtnComboConfig,
}

pub fn load_from_file() {
    let combo_path = TRAINING_MODPACK_TOML_PATH;
    info!("Checking for previous button combo settings in {TRAINING_MODPACK_TOML_PATH}...");
    let mut valid_button_config = false;
    if fs::metadata(combo_path).is_ok() {
        info!("Previous button combo settings found. Loading...");
        let combo_conf = fs::read_to_string(combo_path)
            .unwrap_or_else(|_| panic!("Could not read {}", combo_path));
        let conf: Result<TopLevelBtnComboConfig, toml::de::Error> = toml::from_str(&combo_conf);
        if let Ok(conf) = conf {
            if validate_config(conf) {
                save_all_btn_config_from_toml(&combo_conf);
                valid_button_config = true;
            }
        }
    }

    if !valid_button_config {
        info!("No previous button combo file found. Creating...");
        fs::write(combo_path, DEFAULT_BTN_CONFIG).expect("Failed to write button config conf file");
        save_all_btn_config_from_defaults();
    }
}

fn save_all_btn_config_from_defaults() {
    let conf = TopLevelBtnComboConfig {
        button_config: BtnComboConfig {
            open_menu: BtnList {
                hold: vec!["SPECIAL".to_string()],
                press: vec!["UPTAUNT".to_string()],
            },
            save_state: BtnList {
                hold: vec!["GRAB".to_string()],
                press: vec!["DOWNTAUNT".to_string()],
            },
            load_state: BtnList {
                hold: vec!["GRAB".to_string()],
                press: vec!["UPTAUNT".to_string()],
            },
            previous_save_state_slot: BtnList {
                hold: vec!["SHIELD".to_string()],
                press: vec!["LEFTTAUNT".to_string()],
            },
            next_save_state_slot: BtnList {
                hold: vec!["SHIELD".to_string()],
                press: vec!["RIGHTTAUNT".to_string()],
            },
        },
    };
    unsafe {
        // This println is necessary. Why?.......
        println!("{:?}", &conf.button_config.load_state.press);
        BUTTON_COMBO_CONFIG = conf.button_config;
    }
}

fn save_all_btn_config_from_toml(data: &str) {
    let conf: TopLevelBtnComboConfig = toml::from_str(data).expect("Could not parse button config");
    unsafe {
        // This println is necessary. Why?.......
        println!("{:?}", &conf.button_config.load_state.press);
        BUTTON_COMBO_CONFIG = conf.button_config;
    }
}

fn validate_config(conf: TopLevelBtnComboConfig) -> bool {
    let conf = conf.button_config;
    let configs = [
        conf.open_menu,
        conf.save_state,
        conf.load_state,
        conf.previous_save_state_slot,
        conf.next_save_state_slot,
    ];
    let bad_keys = configs
        .iter()
        .flat_map(|btn_list| {
            btn_list
                .hold
                .iter()
                .chain(btn_list.press.iter())
                .filter(|x| !BUTTON_MAPPING.contains_key(x.to_uppercase().as_str()))
        })
        .collect::<Vec<&String>>();

    if !bad_keys.is_empty() {
        skyline::error::show_error(
            0x71,
            "Training Modpack custom button\nconfiguration is invalid!\0",
            &format!(
                "The following keys are invalid in\n{}:\n\
                {:?}\n\nPossible Keys: {:#?}\0",
                TRAINING_MODPACK_TOML_PATH,
                &bad_keys,
                BUTTON_MAPPING.keys()
            ),
        );
        false
    } else {
        true
    }
}

unsafe fn get_combo_keys(combo: ButtonCombo) -> (&'static Vec<String>, &'static Vec<String>) {
    match combo {
        ButtonCombo::OpenMenu => (
            &BUTTON_COMBO_CONFIG.open_menu.hold,
            &BUTTON_COMBO_CONFIG.open_menu.press,
        ),
        ButtonCombo::SaveState => (
            &BUTTON_COMBO_CONFIG.save_state.hold,
            &BUTTON_COMBO_CONFIG.save_state.press,
        ),
        ButtonCombo::LoadState => (
            &BUTTON_COMBO_CONFIG.load_state.hold,
            &BUTTON_COMBO_CONFIG.load_state.press,
        ),
        ButtonCombo::PrevSaveStateSlot => (
            &BUTTON_COMBO_CONFIG.previous_save_state_slot.hold,
            &BUTTON_COMBO_CONFIG.previous_save_state_slot.press,
        ),
        ButtonCombo::NextSaveStateSlot => (
            &BUTTON_COMBO_CONFIG.next_save_state_slot.hold,
            &BUTTON_COMBO_CONFIG.next_save_state_slot.press,
        ),
    }
}

fn combo_passes(
    module_accessor: *mut smash::app::BattleObjectModuleAccessor,
    combo: ButtonCombo,
) -> bool {
    unsafe {
        let (hold, press) = get_combo_keys(combo);
        let this_combo_passes = hold
            .iter()
            .map(|hold| *BUTTON_MAPPING.get(&*hold.to_uppercase()).unwrap())
            .all(|hold| ControlModule::check_button_on(module_accessor, hold))
            && press
                .iter()
                .map(|press| *BUTTON_MAPPING.get(&*press.to_uppercase()).unwrap())
                .all(|press| ControlModule::check_button_trigger(module_accessor, press));

        this_combo_passes
    }
}

pub fn combo_passes_exclusive(
    module_accessor: *mut smash::app::BattleObjectModuleAccessor,
    combo: ButtonCombo,
) -> bool {
    let other_combo_passes = ButtonCombo::iter()
        .filter(|other_combo| *other_combo != combo)
        .any(|other_combo| combo_passes(module_accessor, other_combo));
    combo_passes(module_accessor, combo) && !other_combo_passes
}

const DEFAULT_BTN_CONFIG: &str = r#"[button_config]
# Available Options:
#
# ATTACK
# SPECIAL
# SHIELD
# GRAB
# JUMP
# UPTAUNT
# DOWNTAUNT
# LEFTTAUNT
# RIGHTTAUNT
# SHARESTOCK
# JUMPMINI
#
# It is recommended to only put one button in the "press" section for each button
# combination, but you can add several buttons to "hold" like this:
# hold=["ATTACK", "SPECIAL",]
#
# SHARESTOCK is typically A+B
# JUMPMINI is the combination of two jump buttons
[button_config.open_menu]
hold=["SPECIAL",]
press=["UPTAUNT",]

[button_config.save_state]
hold=["GRAB",]
press=["DOWNTAUNT",]

[button_config.load_state]
hold=["GRAB",]
press=["UPTAUNT",]

[button_config.previous_save_state_slot]
hold=["SHIELD",]
press=["LEFTTAUNT",]

[button_config.next_save_state_slot]
hold=["SHIELD",]
press=["RIGHTTAUNT",]
"#;
