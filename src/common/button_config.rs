use std::fs;

use crate::common::menu::P1_CONTROLLER_STATE;
use crate::consts::TRAINING_MODPACK_TOML_PATH;
use crate::input::{ControllerStyle::*, *};

use log::info;
use serde::Deserialize;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use toml;

const BUTTON_MAPPINGS: [&str; 12] = [
    "A",
    "B",
    "X",
    "Y",
    "L",
    "R",
    "ZL",
    "ZR",
    "DPAD_UP",
    "DPAD_DOWN",
    "DPAD_LEFT",
    "DPAD_RIGHT",
];

fn button_mapping(name: &str, style: ControllerStyle, b: ButtonBitfield) -> bool {
    match name {
        "A" => b.a(),
        "B" => b.b(),
        "X" => b.x(),
        "Y" => b.y(),
        "L" => match style {
            GCController => false,
            _ => b.l(),
        },
        "R" => match style {
            GCController => b.zr(),
            _ => b.r(),
        },
        "ZL" => match style {
            GCController => b.l() || b.real_digital_l(),
            _ => b.zl(),
        },
        "ZR" => match style {
            GCController => b.r() || b.real_digital_r(),
            _ => b.zr(),
        },
        "DPAD_UP" => b.dpad_up(),
        "DPAD_DOWN" => b.dpad_down(),
        "DPAD_LEFT" => b.dpad_left(),
        "DPAD_RIGHT" => b.dpad_right(),
        _ => panic!("Invalid button name: {}", name),
    }
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
    input_record: BtnList {
        hold: vec![],
        press: vec![],
    },
    input_playback: BtnList {
        hold: vec![],
        press: vec![],
    },
};

#[derive(Debug, EnumIter, PartialEq)]
pub enum ButtonCombo {
    OpenMenu,
    SaveState,
    LoadState,
    InputRecord,
    InputPlayback,
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
    input_record: BtnList,
    input_playback: BtnList,
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
                hold: vec!["B".to_string()],
                press: vec!["DPAD_UP".to_string()],
            },
            save_state: BtnList {
                hold: vec!["ZL".to_string()],
                press: vec!["DPAD_DOWN".to_string()],
            },
            load_state: BtnList {
                hold: vec!["ZL".to_string()],
                press: vec!["DPAD_UP".to_string()],
            },
            input_record: BtnList {
                hold: vec!["ZR".to_string()],
                press: vec!["DPAD_LEFT".to_string()],
            },
            input_playback: BtnList {
                hold: vec!["ZR".to_string()],
                press: vec!["DPAD_RIGHT".to_string()],
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
        conf.input_record,
        conf.input_playback,
    ];
    let bad_keys = configs
        .iter()
        .flat_map(|btn_list| {
            btn_list
                .hold
                .iter()
                .chain(btn_list.press.iter())
                .filter(|x| !BUTTON_MAPPINGS.contains(&x.to_uppercase().as_str()))
        })
        .collect::<Vec<&String>>();

    if !bad_keys.is_empty() {
        skyline::error::show_error(
            0x71,
            "Training Modpack custom button\nconfiguration is invalid!\0",
            &format!(
                "The following keys are invalid in\n{}:\n\
                {:?}\n\nPossible Keys: {:#?}\0",
                TRAINING_MODPACK_TOML_PATH, &bad_keys, BUTTON_MAPPINGS
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
        ButtonCombo::InputRecord => (
            &BUTTON_COMBO_CONFIG.input_record.hold,
            &BUTTON_COMBO_CONFIG.input_record.press,
        ),
        ButtonCombo::InputPlayback => (
            &BUTTON_COMBO_CONFIG.input_playback.hold,
            &BUTTON_COMBO_CONFIG.input_playback.press,
        ),
    }
}

fn combo_passes(combo: ButtonCombo) -> bool {
    unsafe {
        let (hold, press) = get_combo_keys(combo);
        let p1_controller_state = *P1_CONTROLLER_STATE.data_ptr();
        let this_combo_passes = hold.iter().all(|hold| {
            button_mapping(
                &hold.to_uppercase(),
                p1_controller_state.style,
                p1_controller_state.current_buttons,
            )
        }) && press.iter().all(|hold| {
            button_mapping(
                &hold.to_uppercase(),
                p1_controller_state.style,
                p1_controller_state.just_down,
            )
        });

        this_combo_passes
    }
}

pub fn combo_passes_exclusive(combo: ButtonCombo) -> bool {
    let other_combo_passes = ButtonCombo::iter()
        .filter(|other_combo| *other_combo != combo)
        .any(combo_passes);
    combo_passes(combo) && !other_combo_passes
}

const DEFAULT_BTN_CONFIG: &str = r#"[button_config]
# Available Options:
#
# DPAD_UP
# DPAD_RIGHT
# DPAD_DOWN
# DPAD_LEFT
# A
# B
# X
# Y
# L
# R
# ZL
# ZR
#
# It is recommended to only put one button in the "press" section for each button
# combination, but you can add several buttons to "hold" like this:
# hold=["A", "B",]
#
[button_config.open_menu]
hold=["B",]
press=["DPAD_UP",]

[button_config.save_state]
hold=["ZL",]
press=["DPAD_DOWN",]

[button_config.load_state]
hold=["ZL",]
press=["DPAD_UP",]

[button_config.input_record]
hold=["ZR",]
press=["DPAD_LEFT",]

[button_config.input_playback]
hold=["ZR",]
press=["DPAD_RIGHT",]
"#;
