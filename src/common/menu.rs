
use std::fs;
use std::path::Path;
use crate::common::*;
use skyline::info::get_program_id;
use skyline::nn::hid::NpadHandheldState;
use smash::lib::lua_const::*;
use skyline_web::{Background, BootDisplay, Webpage};
use ramhorns::{Template, Content};

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
struct OnOffSelector<'a> {
    title: &'a str,
    checked: &'a str
}

impl <'a>OnOffSelector<'a> {
    pub fn new(title: &'a str, checked: bool) -> OnOffSelector<'a> {
        OnOffSelector {
            title: title,
            checked: if checked { "is-appear "} else { "is-hidden" }
        }
    }
}

#[derive(Content)]
struct SubMenu<'a> {
    title: &'a str,
    id: &'a str,
    toggles: Vec<Toggle<'a>>,
    sliders: Vec<Slider>,
    onoffselector: Vec<OnOffSelector<'a>>,
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

    pub fn add_slider(&mut self, min: usize, max: usize, value: usize) {
        self.sliders.push(Slider{
            min,
            max,
            index: self.max_idx() + 1,
            value
        });
    }

    pub fn add_onoffselector(&mut self, title: &'a str, checked: bool) {
        // TODO: Is there a more elegant way to do this?
        // The HTML only supports a single onoffselector but the SubMenu stores it as a Vec
        self.onoffselector.push(OnOffSelector{
            title: title,
            checked: if checked { "is-appear "} else { "is-hidden" }
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

    pub fn add_sub_menu(&mut self, title: &'a str, id: &'a str, check_against: usize, toggles: Vec<(&'a str, usize)>, sliders: Vec<(usize,usize,usize)>) {
        let mut sub_menu = SubMenu {
            title: title,
            id: id,
            toggles: Vec::new(),
            sliders: Vec::new(),
            onoffselector: Vec::new(),
            index: self.max_idx() + 1,
            check_against: check_against
        };

        for toggle in toggles {
            sub_menu.add_toggle(toggle.0, (check_against & toggle.1) != 0, toggle.1)
        }

        for slider in sliders {
            sub_menu.add_slider(slider.0, slider.1, slider.2);
        }

        self.sub_menus.push(sub_menu);
    }

    pub fn add_sub_menu_sep(&mut self, title: &'a str, id: &'a str, check_against: usize, strs: Vec<&'a str>, vals: Vec<usize>) {
        let mut sub_menu = SubMenu {
            title: title,
            id: id,
            toggles: Vec::new(),
            sliders: Vec::new(),
            onoffselector: Vec::new(),
            index: self.max_idx() + 1,
            check_against: check_against
        };

        for i in 0..strs.len() {
            sub_menu.add_toggle(strs[i], (check_against & vals[i]) != 0, vals[i])
        }

        // TODO: add sliders?

        self.sub_menus.push(sub_menu);
    }

    pub fn add_sub_menu_onoff(&mut self, title: &'a str, id: &'a str, check_against: usize, checked: bool) {
        let mut sub_menu = SubMenu {
            title: title,
            id: id,
            toggles: Vec::new(),
            sliders: Vec::new(),
            onoffselector: Vec::new(),
            index: self.max_idx() + 1,
            check_against: check_against
        };

        sub_menu.add_onoffselector(title, checked);
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
                MENU.$id.bits() as usize,
                [<$id _strs>].iter().map(|i| i.as_str()).collect(),
                [<$id _vals>]
            );
        }
    }
}

pub fn set_menu_from_url(s: &str) {
    let base_url_len = "http://localhost/?".len();
    let total_len = s.len();

    let ss: String = s.chars().skip(base_url_len).take(total_len - base_url_len).collect();
    
    for toggle_values in ss.split("&") {
        let toggle_value_split = toggle_values.split("=").collect::<Vec<&str>>();
        let toggle = toggle_value_split[0];
        if toggle == "" { continue; }
        
        let toggle_vals = toggle_value_split[1];
        
        let mut bits = 0;
        for toggle_val in toggle_vals.split(",") {
            if toggle_val == "" { continue; }
        
            let val = toggle_val.parse::<u32>().unwrap();
            bits = bits | val;
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
    add_bitflag_submenu!(overall_menu, "Mash Toggles", mash_state, Action);
    add_bitflag_submenu!(overall_menu, "Followup Toggles", follow_up, Action);
    add_bitflag_submenu!(overall_menu, "Attack Angle", attack_angle, AttackAngle);

    add_bitflag_submenu!(overall_menu, "Ledge Options", ledge_state, LedgeOption);
    add_bitflag_submenu!(overall_menu, "Ledge Delay", ledge_delay, LongDelay);
    add_bitflag_submenu!(overall_menu, "Tech Options", tech_state, TechFlags);
    add_bitflag_submenu!(overall_menu, "Miss Tech Options", miss_tech_state, MissTechFlags);
    add_bitflag_submenu!(overall_menu, "Defensive Options", defensive_state, Defensive);

    add_bitflag_submenu!(overall_menu, "Aerial Delay", aerial_delay, Delay);
    add_bitflag_submenu!(overall_menu, "OoS Offset", oos_offset, Delay);
    add_bitflag_submenu!(overall_menu, "Reaction Time", reaction_time, Delay);

    add_bitflag_submenu!(overall_menu, "Fast Fall", fast_fall, BoolFlag);
    add_bitflag_submenu!(overall_menu, "Fast Fall Delay", fast_fall_delay, Delay);
    add_bitflag_submenu!(overall_menu, "Falling Aerials", falling_aerials, BoolFlag);
    add_bitflag_submenu!(overall_menu, "Full Hop", full_hop, BoolFlag);

    add_bitflag_submenu!(overall_menu, "Shield Tilt", shield_tilt, Direction);
    add_bitflag_submenu!(overall_menu, "DI Direction", di_state, Direction);
    add_bitflag_submenu!(overall_menu, "SDI Direction", sdi_state, Direction);
    add_bitflag_submenu!(overall_menu, "Airdodge Direction", air_dodge_dir, Direction);

    overall_menu.add_sub_menu(
        "SDI Strength",
        "sdi_strength",
        MENU.sdi_strength as usize,
        [
            ("Normal", SdiStrength::Normal as usize),
            ("Medium", SdiStrength::Medium as usize),
            ("High", SdiStrength::High as usize),
        ].to_vec(),
        [].to_vec()
    );

    overall_menu.add_sub_menu(
        "Shield Toggles",
        "shield_state",
        MENU.shield_state as usize,
        [
            ("None", Shield::None as usize),
            ("Hold", Shield::Hold as usize),
            ("Infinite", Shield::Infinite as usize),
        ].to_vec(),
        [].to_vec()
    );

    overall_menu.add_sub_menu(
        "Mirroring",
        "save_state_mirroring",
        MENU.save_state_mirroring as usize,
        [
            ("None", SaveStateMirroring::None as usize),
            ("Alternate", SaveStateMirroring::Alternate as usize),
            ("Random", SaveStateMirroring::Random as usize),
        ].to_vec(),
        [].to_vec()
    );

    // Slider menus
    overall_menu.add_sub_menu(
        "Input Delay", 
        "input_delay", 
        // unnecessary for slider?
        MENU.input_delay as usize,
        [].to_vec(),
        [
            (0, 10, MENU.input_delay as usize)
        ].to_vec()
    );


    // OnOff flags
    overall_menu.add_sub_menu_onoff(
        "Save Damage",
        "save_damage",
        MENU.save_damage as usize,
        (MENU.save_damage as usize & OnOff::On as usize) != 0
    );
    overall_menu.add_sub_menu_onoff(
        "Hitbox Visualization",
        "hitbox_vis",
        MENU.hitbox_vis as usize,
        (MENU.hitbox_vis as usize & OnOff::On as usize) != 0
    );

    overall_menu.add_sub_menu_onoff(
        "Stage Hazards",
        "stage_hazards",
        MENU.stage_hazards as usize,
        (MENU.stage_hazards as usize & OnOff::On as usize) != 0
    );
    overall_menu.add_sub_menu_onoff(
        "Frame Advantage",
        "frame_advantage",
        MENU.frame_advantage as usize,
        (MENU.frame_advantage as usize & OnOff::On as usize) != 0
    );
    overall_menu.add_sub_menu_onoff(
        "Mash In Neutral",
        "mash_in_neutral",
        MENU.mash_in_neutral as usize,
        (MENU.mash_in_neutral as usize & OnOff::On as usize) != 0
    );

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

    let response = Webpage::new()
        .background(Background::BlurredScreenshot)
        .htdocs_dir("contents")
        .boot_display(BootDisplay::BlurredScreenshot)
        .boot_icon(true)
        .start_page(&format!("{}{}", fname, params))
        .open()
        .unwrap();

    let last_url = response.get_last_url().unwrap();

    set_menu_from_url(last_url);
}
