use std::fs;
use std::path::Path;
use crate::common::*;
use skyline::info::get_program_id;
use smash::lib::lua_const::*;
use skyline_web::{Background, BootDisplay, Webpage};
use ramhorns::{Template, Content};
use strum::IntoEnumIterator;

#[derive(Content)]
struct Slider {
    min: usize,
    max: usize,
    index: usize,
    value: usize
}

#[derive(Content)]
struct Toggle<'a> {
    title: &'a str,
    checked: &'a str,
    index: usize,
    value: usize,
    default: &'a str,
}

#[derive(Content)]
struct OnOffSelector<'a> {
    title: &'a str,
    checked: &'a str,
    default: &'a str,
}

#[derive(Content)]
struct SubMenu<'a> {
    title: &'a str,
    id: &'a str,
    toggles: Vec<Toggle<'a>>,
    sliders: Vec<Slider>,
    onoffselector: Vec<OnOffSelector<'a>>,
    index: usize,
    check_against: usize,
    is_single_option: Option<bool>,
    help_text: &'a str,
}

impl<'a> SubMenu<'a> {
    pub fn max_idx(&self) -> usize {
        self.toggles
            .iter()
            .max_by(|t1,t2| t1.index.cmp(&t2.index))
            .map(|t| t.index)
            .unwrap_or(self.index)
    }

    pub fn add_toggle(&mut self, title: &'a str, checked: bool, value: usize, default: bool) {
        self.toggles.push(Toggle{
            title,
            checked: if checked { "is-appear"} else { "is-hidden" },
            index: self.max_idx() + 1,
            value,
            default: if default { "is-appear"} else { "is-hidden" },
        });
    }

    pub fn add_slider(&mut self, min: usize, max: usize, value: usize) {
        self.sliders.push(Slider{
            min,
            max,
            index: self.max_idx() + 1,
            value
        });
    }

    pub fn add_onoffselector(&mut self, title: &'a str, checked: bool, default: bool) {
        // TODO: Is there a more elegant way to do this?
        // The HTML only supports a single onoffselector but the SubMenu stores it as a Vec
        self.onoffselector.push(OnOffSelector{
            title,
            checked: if checked { "is-appear"} else { "is-hidden" },
            default: if default { "is-appear"} else { "is-hidden" },
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

    pub fn add_sub_menu(&mut self, title: &'a str, id: &'a str, check_against: usize, toggles: Vec<(&'a str, usize)>, sliders: Vec<(usize,usize,usize)>, defaults: usize, help_text: &'a str) {
        let mut sub_menu = SubMenu {
            title,
            id,
            toggles: Vec::new(),
            sliders: Vec::new(),
            onoffselector: Vec::new(),
            index: self.max_idx() + 1,
            check_against,
            is_single_option: Some(true),
            help_text,
        };

        for toggle in toggles {
            sub_menu.add_toggle(toggle.0, (check_against & toggle.1) != 0, toggle.1, (defaults & toggle.1) != 0)
        }

        for slider in sliders {
            sub_menu.add_slider(slider.0, slider.1, slider.2);
        }

        self.sub_menus.push(sub_menu);
    }

    pub fn add_sub_menu_sep(&mut self, title: &'a str, id: &'a str, check_against: usize, strs: Vec<&'a str>, vals: Vec<usize>, defaults: usize, help_text: &'a str) {
        let mut sub_menu = SubMenu {
            title,
            id,
            toggles: Vec::new(),
            sliders: Vec::new(),
            onoffselector: Vec::new(),
            index: self.max_idx() + 1,
            check_against,
            is_single_option: None,
            help_text,
        };

        for i in 0..strs.len() {
            sub_menu.add_toggle(strs[i], (check_against & vals[i]) != 0, vals[i], (defaults & vals[i]) != 0)
        }

        // TODO: add sliders?

        self.sub_menus.push(sub_menu);
    }

    pub fn add_sub_menu_onoff(&mut self, title: &'a str, id: &'a str, check_against: usize, checked: bool, default: usize, help_text: &'a str) {
        let mut sub_menu = SubMenu {
            title,
            id,
            toggles: Vec::new(),
            sliders: Vec::new(),
            onoffselector: Vec::new(),
            index: self.max_idx() + 1,
            check_against,
            is_single_option: None,
            help_text,
        };

        sub_menu.add_onoffselector(title, checked, (default & OnOff::On as usize) != 0);
        self.sub_menus.push(sub_menu);
    }
}

macro_rules! add_bitflag_submenu {
    ($menu:ident, $title:literal, $id:ident, $e:ty, $help_text:literal) => {
        paste::paste!{
            let [<$id _strs>] = <$e>::to_toggle_strs();
            let [<$id _vals>] = <$e>::to_toggle_vals();

            $menu.add_sub_menu_sep(
                $title, 
                stringify!($id), 
                MENU.$id.bits() as usize,
                [<$id _strs>].iter().map(|i| i.as_str()).collect(),
                [<$id _vals>],
                DEFAULT_MENU.$id.bits() as usize,
                stringify!($help_text),
            );
        }
    }
}

macro_rules! add_single_option_submenu {
    ($menu:ident, $title:literal, $id:ident, $e:ty, $help_text:literal) => {
        paste::paste!{
            let mut [<$id _toggles>] = Vec::new();
            for val in [<$e>]::iter() {
                [<$id _toggles>].push((val.into_string(), val as usize));
            }

            $menu.add_sub_menu(
                $title, 
                stringify!($id), 
                MENU.$id as usize,
                [<$id _toggles>].iter().map(|(x, y)| (x.as_str(), *y)).collect::<Vec<(&str, usize)>>(),
                [].to_vec(),
                DEFAULT_MENU.$id as usize,
                stringify!($help_text),
            );
        }
    }
}

macro_rules! add_onoff_submenu {
    ($menu:ident, $title:literal, $id:ident, $help_text:literal) => {
        paste::paste!{
            $menu.add_sub_menu_onoff(
                $title, 
                stringify!($id), 
                MENU.$id as usize,
                (MENU.$id as usize & OnOff::On as usize) != 0,
                DEFAULT_MENU.$id as usize,
                stringify!($help_text),
            );
        }
    }
}

pub fn set_menu_from_url(s: &str) {
    let base_url_len = "http://localhost/?".len();
    let total_len = s.len();

    let ss: String = s.chars().skip(base_url_len).take(total_len - base_url_len).collect();
    
    for toggle_values in ss.split('&') {
        let toggle_value_split = toggle_values.split('=').collect::<Vec<&str>>();
        let toggle = toggle_value_split[0];
        if toggle.is_empty() { continue; }
        
        let toggle_vals = toggle_value_split[1];
        
        let mut bits = 0;
        for toggle_val in toggle_vals.split(',') {
            if toggle_val.is_empty() { continue; }
        
            let val = toggle_val.parse::<u32>().unwrap();
            bits |= val;
        }


        unsafe {
            MENU.set(toggle, bits);
        }
    }
}

pub unsafe fn menu_condition(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> bool {
    ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) &&
    ControlModule::check_button_on_trriger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
}

pub unsafe fn write_menu() {
    let tpl = Template::new(include_str!("../templates/menu.html")).unwrap();

    let mut overall_menu = Menu {
        sub_menus: Vec::new()
    };

    // Toggle/bitflag menus
    add_bitflag_submenu!(overall_menu, "Mash Toggles", mash_state, Action, "Mash Toggles: Actions to be performed as soon as possible");
    add_bitflag_submenu!(overall_menu, "Followup Toggles", follow_up, Action, "Followup Toggles: Actions to be performed after the Mash option");
    add_bitflag_submenu!(overall_menu, "Attack Angle", attack_angle, AttackAngle, "Attack Angle: For attacks that can be angled, such as some forward tilts");

    add_bitflag_submenu!(overall_menu, "Ledge Options", ledge_state, LedgeOption, "Ledge Options: Actions to be taken when on the ledge");
    add_bitflag_submenu!(overall_menu, "Ledge Delay", ledge_delay, LongDelay, "Ledge Delay: How many frames to delay the ledge option");
    add_bitflag_submenu!(overall_menu, "Tech Options", tech_state, TechFlags, "Tech Options: Actions to take when slammed into a hard surface");
    add_bitflag_submenu!(overall_menu, "Miss Tech Options", miss_tech_state, MissTechFlags, "Miss Tech Options: Actions to take after missing a tech");
    add_bitflag_submenu!(overall_menu, "Defensive Options", defensive_state, Defensive, "Defensive Options: Actions to take after a ledge option, tech option, or miss tech option");

    add_bitflag_submenu!(overall_menu, "Aerial Delay", aerial_delay, Delay, "Aerial Delay: How long to delay a Mash aerial attack");
    add_bitflag_submenu!(overall_menu, "OoS Offset", oos_offset, Delay, "OoS Offset: How many times the CPU shield can be hit before performing a Mash option");
    add_bitflag_submenu!(overall_menu, "Reaction Time", reaction_time, Delay, "Reaction Time: How many frames to delay before performing an option out of shield");

    add_bitflag_submenu!(overall_menu, "Fast Fall", fast_fall, BoolFlag, "Fast Fall: Should the CPU fastfall during a jump");
    add_bitflag_submenu!(overall_menu, "Fast Fall Delay", fast_fall_delay, Delay, "Fast Fall Delay: How many frames the CPU should delay their fastfall");
    add_bitflag_submenu!(overall_menu, "Falling Aerials", falling_aerials, BoolFlag, "Falling Aerials: Should aerials be performed when rising or when falling");
    add_bitflag_submenu!(overall_menu, "Full Hop", full_hop, BoolFlag, "Full Hop: Should the CPU perform a full hop or a short hop");

    add_bitflag_submenu!(overall_menu, "Shield Tilt", shield_tilt, Direction, "Shield Tilt: Direction to tilt the shield");
    add_bitflag_submenu!(overall_menu, "DI Direction", di_state, Direction, "DI Direction: Direction to angle the directional influence during hitlag");
    add_bitflag_submenu!(overall_menu, "SDI Direction", sdi_state, Direction, "SDI Direction: Direction to angle the smash directional influence during hitlag");
    add_bitflag_submenu!(overall_menu, "Airdodge Direction", air_dodge_dir, Direction, "Airdodge Direction: Direction to angle airdodges");

    add_single_option_submenu!(overall_menu, "SDI Strength", sdi_strength, SdiStrength, "SDI Strength: Relative strength of the smash directional influence inputs");
    add_single_option_submenu!(overall_menu, "Shield Toggles", shield_state, Shield, "Shield Toggles: CPU Shield Behavior");
    add_single_option_submenu!(overall_menu, "Mirroring", save_state_mirroring, SaveStateMirroring, "Mirroring: Flips save states in the left-right direction across the stage center");


    // Slider menus
    overall_menu.add_sub_menu(
        "Input Delay", 
        "input_delay", 
        // unnecessary for slider?
        MENU.input_delay as usize,
        [("0", 0),("1",1),("2",2),("3",3),("4",4),("5",5),("6",6),("7",7),("8",8),("9",9),("10",10)].to_vec(),
        [].to_vec(), //(0, 10, MENU.input_delay as usize)
        DEFAULT_MENU.input_delay as usize,
        stringify!("Input Delay: Frames to delay player inputs by"),
    );

    add_onoff_submenu!(overall_menu, "Save Damage", save_damage, "Save Damage: Should save states retain player/CPU damage");
    add_bitflag_submenu!(overall_menu, "Visualization", visualization, VisualizationFlags, "Visualization: Should hitboxes and/or hurtboxes be displayed.");
    add_onoff_submenu!(overall_menu, "Stage Hazards", stage_hazards, "Stage Hazards: Should stage hazards be present");
    add_onoff_submenu!(overall_menu, "Frame Advantage", frame_advantage, "Frame Advantage: Display the time difference between when the player is actionable and the CPU is actionable");
    add_onoff_submenu!(overall_menu, "Mash In Neutral", mash_in_neutral, "Mash In Neutral: Should Mash options be performed repeatedly or only when the CPU is hit");

    let data = tpl.render(&overall_menu);

    // Now that we have the html, write it to file
    // From skyline-web
    let program_id = get_program_id();
    let htdocs_dir = "contents";
    let path = Path::new("sd:/atmosphere/contents")
        .join(&format!("{:016X}", program_id))
        .join(&format!("manual_html/html-document/{}.htdocs/", htdocs_dir))
        .join("index.html");
    fs::write(path, data).unwrap();
}

pub unsafe fn spawn_menu() {
    let fname = "index.html";
    let params = MENU.to_url_params();
    let page_response = Webpage::new()
        .background(Background::BlurredScreenshot)
        .htdocs_dir("contents")
        .boot_display(BootDisplay::BlurredScreenshot)
        .boot_icon(true)
        .start_page(&format!("{}{}", fname, params))
        .open()
        .unwrap();

     let last_url = page_response
        .get_last_url()
        .unwrap();

    set_menu_from_url(last_url);

    let menu_conf_path = "sd:/TrainingModpack/training_modpack_menu.conf";
    std::fs::write(menu_conf_path, last_url)
        .expect("Failed to write menu conf file");
}
