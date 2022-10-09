use lazy_static::lazy_static;
use parking_lot::Mutex;
use serde::Deserialize;
use smash::lib::lua_const::*;
use std::collections::HashMap;
use toml;

lazy_static! {
    // Using the LuaConst names wasn't working for some reason...
    static ref BUTTON_MAPPING: HashMap<&'static str, i32> = HashMap::from([
        ("ATTACK", 0xE),  // *CONTROL_PAD_BUTTON_ATTACK_RAW
        ("SPECIAL", 0xF), // *CONTROL_PAD_BUTTON_SPECIAL_RAW
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
    pub static ref OPEN_MENU_BTN_HOLD: Mutex<Vec<i32>> = Mutex::new(vec![BUTTON_MAPPING["SPECIAL"]]);
    pub static ref OPEN_MENU_BTN_PRESS: Mutex<Vec<i32>> = Mutex::new(vec![BUTTON_MAPPING["UPTAUNT"]]);
    pub static ref SAVE_STATE_BTN_HOLD: Mutex<Vec<i32>> = Mutex::new(vec![BUTTON_MAPPING["GRAB"]]);
    pub static ref SAVE_STATE_BTN_PRESS: Mutex<Vec<i32>> = Mutex::new(vec![BUTTON_MAPPING["DOWNTAUNT"]]);
    pub static ref LOAD_STATE_BTN_HOLD: Mutex<Vec<i32>> = Mutex::new(vec![BUTTON_MAPPING["GRAB"]]);
    pub static ref LOAD_STATE_BTN_PRESS: Mutex<Vec<i32>> = Mutex::new(vec![BUTTON_MAPPING["UPTAUNT"]]);
}

#[derive(Deserialize)]
struct BtnList {
    hold: Vec<String>,
    press: Vec<String>,
}

#[derive(Deserialize)]
struct BtnComboConfig {
    open_menu: BtnList,
    save_state: BtnList,
    load_state: BtnList,
}

#[derive(Deserialize)]
struct TopLevelBtnComboConfig {
    button_config: BtnComboConfig,
}

fn save_btn_config(btnlist: BtnList, mutex_hold: &Mutex<Vec<i32>>, mutex_press: &Mutex<Vec<i32>>) {
    // HOLD
    let mut global_hold = mutex_hold.lock();
    let vecopt_hold: Vec<Option<&i32>> = btnlist
        .hold
        .into_iter()
        .map(|x| BUTTON_MAPPING.get(x.as_str()))
        .collect();
    if vecopt_hold.iter().all(|x| x.is_some()) {
        // All entries valid keys of BUTTON_MAPPING
        global_hold.clear();
        global_hold.extend_from_slice(
            &vecopt_hold
                .into_iter()
                .map(|x| *x.unwrap())
                .collect::<Vec<i32>>(),
        );
    } else {
        // Invalid config. Sticking with default.
        // TODO: Should we panic here instead of silently continuing?
    }

    // PRESS
    let mut global_press = mutex_press.lock();
    let vecopt_press: Vec<Option<&i32>> = btnlist
        .press
        .into_iter()
        .map(|x| BUTTON_MAPPING.get(x.as_str()))
        .collect();
    if vecopt_press.iter().all(|x| x.is_some()) {
        // All entries valid keys of BUTTON_MAPPING
        global_press.clear();
        global_press.extend_from_slice(
            &vecopt_press
                .into_iter()
                .map(|x| *x.unwrap())
                .collect::<Vec<i32>>(),
        );
    } else {
        // Invalid config. Sticking with default.
        // TODO: Should we panic here instead of silently continuing?
    }
}

pub fn save_all_btn_config_from_toml(data: &str) {
    let conf: TopLevelBtnComboConfig = toml::from_str(data).unwrap();
    let open_menu_conf: BtnList = conf.button_config.open_menu;
    let save_state_conf: BtnList = conf.button_config.save_state;
    let load_state_conf: BtnList = conf.button_config.load_state;

    save_btn_config(open_menu_conf, &OPEN_MENU_BTN_HOLD, &OPEN_MENU_BTN_PRESS);
    save_btn_config(save_state_conf, &SAVE_STATE_BTN_HOLD, &SAVE_STATE_BTN_PRESS);
    save_btn_config(load_state_conf, &LOAD_STATE_BTN_HOLD, &LOAD_STATE_BTN_PRESS);
}

pub const DEFAULT_BTN_CONFIG: &'static str = r#"[button_config]
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
"#;
