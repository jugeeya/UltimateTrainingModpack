

use crate::common::*;
use skyline::nn::hid::NpadHandheldState;

use skyline_web::{Background, BootDisplay, Webpage};
use ramhorns::{Template, Content};

#[derive(Content)]
struct Toggle<'a> {
    title: &'a str,
    checked: &'a str,
    index: usize,
    value: usize,
}

impl<'a> Toggle<'a> {
    pub fn new(title: &'a str, checked: bool, value: usize) -> Toggle<'a> {
        Toggle{
            title: title,
            checked: if checked { "is-appear "} else { "is-hidden" },
            index: 0,
            value
        }
    }
}

#[derive(Content)]
struct SubMenu<'a> {
    title: &'a str,
    id: &'a str,
    toggles: Vec<Toggle<'a>>,
    index: usize,
    check_against: usize
}

impl<'a> SubMenu<'a> {
    pub fn max_idx(&self) -> usize {
        self.toggles
            .iter()
            .max_by(|t1,t2| t1.index.cmp(&t2.index))
            .map(|t| t.index)
            .unwrap_or(self.index)
    }

    pub fn add_toggle(&mut self, title: &'a str, checked: bool, value: usize) {
        self.toggles.push(Toggle{
            title: title,
            checked: if checked { "is-appear "} else { "is-hidden" },
            index: self.max_idx() + 1,
            value
        });
    }
}

#[derive(Content)]
struct Menu<'a> {
    sub_menus: Vec<SubMenu<'a>>
}

impl<'a> Menu<'a> {
    pub fn max_idx(&self) -> usize {
        self.sub_menus
            .iter()
            .max_by(|x, y| x.max_idx().cmp(&y.max_idx()))
            .map(|sub_menu| sub_menu.max_idx())
            .unwrap_or(0)
    }

    pub fn add_sub_menu(&mut self, title: &'a str, id: &'a str, check_against: usize, toggles: Vec<(&'a str, usize)>) {
        let mut sub_menu = SubMenu {
            title: title,
            id: id,
            toggles: Vec::new(),
            index: self.max_idx() + 1,
            check_against: check_against
        };

        for toggle in toggles {
            sub_menu.add_toggle(toggle.0, (check_against & toggle.1) != 0, toggle.1)
        }

        self.sub_menus.push(sub_menu);
    }

    pub fn add_sub_menu_sep(&mut self, title: &'a str, id: &'a str, check_against: usize, strs: Vec<&'a str>, vals: Vec<usize>) {
        let mut sub_menu = SubMenu {
            title: title,
            id: id,
            toggles: Vec::new(),
            index: self.max_idx() + 1,
            check_against: check_against
        };

        for i in 0..strs.len() {
            sub_menu.add_toggle(strs[i], (check_against & vals[i]) != 0, vals[i])
        }

        self.sub_menus.push(sub_menu);
    }
}

macro_rules! add_bitflag_submenu {
    ($menu:ident, $title:literal, $id:ident, $e:ty) => {
        paste::paste!{
            let [<$id _strs>] = <$e>::to_toggle_strs();
            let [<$id _vals>] = <$e>::to_toggle_vals();

            $menu.add_sub_menu_sep(
                $title, 
                stringify!($id), 
                MENU_STRUCT.$id.bits() as usize,
                [<$id _strs>].iter().map(|i| i.as_str()).collect(),
                [<$id _vals>]
            );
        }
    }
}

pub fn set_menu_from_url(s: &str) {
    let base_url_len = "http://localhost/".len();
    let total_len = s.len();

    let ss: String = s.chars().skip(base_url_len).take(total_len - base_url_len).collect();
    
    for toggle_values in ss.split("&") {
        let toggle_value_split = toggle_values.split("?").collect::<Vec<&str>>();
        let toggle = toggle_value_split[0];
        if toggle == "" { continue; }
        
        let toggle_vals = toggle_value_split[1];
        
        let mut bits = 0;
        for toggle_val in toggle_vals.split(",") {
            if toggle_val == "" { continue; }
        
            let mut val = toggle_val.parse::<u32>().unwrap();
            bits = bits | val;
        }


        unsafe {
            MENU_STRUCT.set(toggle, bits);
        }
    }
}

use std::sync::atomic::{AtomicBool, Ordering};
pub static mut TRIGGER_MENU: AtomicBool = AtomicBool::new(false);

pub unsafe fn handle_get_npad_state(
    state: *mut NpadHandheldState,
    controller_id: *const u32,
) {
    let state = *state;

    // X+DRIGHT
    if (state.Buttons & (1 << 2)) != 0 &&
    (state.Buttons & (1 << 14)) != 0
    {
        *TRIGGER_MENU.get_mut() = true;
    }
}

pub unsafe fn loop_input() {
    std::thread::spawn(|| {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));

            let trigg = TRIGGER_MENU.get_mut();
            if !*trigg {
                continue;
            }

            let tpl = Template::new(include_str!("../templates/menu.html")).unwrap();

            let mut overall_menu = Menu {
                sub_menus: Vec::new()
            };

            add_bitflag_submenu!(overall_menu, "Mash Toggles", mash_state, Action);
            add_bitflag_submenu!(overall_menu, "Followup Toggles", follow_up, Action);

            add_bitflag_submenu!(overall_menu, "Ledge Options", ledge_state, LedgeOption);
            add_bitflag_submenu!(overall_menu, "Ledge Delay", ledge_delay, Delay);
            add_bitflag_submenu!(overall_menu, "Tech Options", tech_state, TechFlags);
            add_bitflag_submenu!(overall_menu, "Miss Tech Options", miss_tech_state, MissTechFlags);
            add_bitflag_submenu!(overall_menu, "Defensive Options", defensive_state, Defensive);

            add_bitflag_submenu!(overall_menu, "OoS Offset", oos_offset, Delay);
            add_bitflag_submenu!(overall_menu, "Reaction Time", reaction_time, Delay);

            add_bitflag_submenu!(overall_menu, "Fast Fall", fast_fall, BoolFlag);
            add_bitflag_submenu!(overall_menu, "Fast Fall Delay", fast_fall_delay, Delay);
            add_bitflag_submenu!(overall_menu, "Falling Aerials", falling_aerials, BoolFlag);
            add_bitflag_submenu!(overall_menu, "Full Hop", full_hop, BoolFlag);

            add_bitflag_submenu!(overall_menu, "Shield Tilt", shield_tilt, Direction);

            add_bitflag_submenu!(overall_menu, "DI", di_state, Direction);
            add_bitflag_submenu!(overall_menu, "SDI", sdi_state, Direction);
            add_bitflag_submenu!(overall_menu, "Airdodge Direction", air_dodge_dir, Direction);

            overall_menu.add_sub_menu(
                "Shield Toggles", 
                "shield_state", 
                MENU_STRUCT.shield_state as usize,
                [
                    ("None", Shield::None as usize),
                    ("Hold", Shield::Hold as usize),
                    ("Infinite", Shield::Infinite as usize),
                ].to_vec()
            );

            // add_bitflag_submenu!(overall_menu, "Input Delay", input_delay, Delay);

            let data = &tpl.render(&overall_menu);

            let response = Webpage::new()
                .background(Background::BlurredScreenshot)
                .file("index.html", data)
                .htdocs_dir("contents")
                .boot_display(BootDisplay::BlurredScreenshot)
                .boot_icon(true)
                .open()
                .unwrap();

            let last_url = response.get_last_url().unwrap();

            set_menu_from_url(last_url);
            *trigg = false;
        }
    });
}
