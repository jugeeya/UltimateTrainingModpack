use crate::common::*;
use crate::events::{Event, EVENT_QUEUE};
use crate::training::frame_counter;
use ramhorns::{Content, Template};
use skyline::info::get_program_id;
use skyline_web::{Background, BootDisplay, Webpage};
use smash::lib::lua_const::*;
use std::fs;
use std::path::Path;

static mut FRAME_COUNTER_INDEX: usize = 0;
const MENU_LOCKOUT_FRAMES: u32 = 15;

pub fn init() {
    unsafe {
        FRAME_COUNTER_INDEX = frame_counter::register_counter();
        write_menu();
    }
}

#[derive(Content)]
struct Toggle<'a> {
    toggle_value: usize,
    toggle_title: &'a str,
}

#[derive(Content)]
struct SubMenu<'a> {
    submenu_title: &'a str,
    submenu_id: &'a str,
    help_text: &'a str,
    is_single_option: bool,
    toggles: Vec<Toggle<'a>>,
}

impl<'a> SubMenu<'a> {
    pub fn add_toggle(
        &mut self,
        toggle_value: usize,
        toggle_title: &'a str,
    ) {
        self.toggles.push(
            Toggle {
                toggle_value: toggle_value,
                toggle_title: toggle_title,
            }
        );
    }
    pub fn new_with_toggles<T:ToggleTrait>(
        submenu_title: &'a str,
        submenu_id: &'a str,
        help_text: &'a str,
        is_single_option: bool,
    ) -> SubMenu<'a> {
            let mut instance = SubMenu {
                submenu_title: submenu_title,
                submenu_id: submenu_id,
                help_text: help_text,
                is_single_option: is_single_option,
                toggles: Vec::new(),
            };
    
            let values = T::to_toggle_vals();
            let titles = T::to_toggle_strs();
            for i in 0..values.len() {
                instance.add_toggle(
                    values[i],
                    titles[i],
                );
            }
            instance
    }
}

#[derive(Content)]
struct Tab<'a> {
    tab_id: &'a str,
    tab_title: &'a str,
    tab_submenus: Vec<SubMenu<'a>>,
}

impl<'a> Tab<'a> {
    pub fn add_submenu_with_toggles<T:ToggleTrait>(
        &mut self,
        submenu_title: &'a str,
        submenu_id: &'a str,
        help_text: &'a str,
        is_single_option: bool,
    ) {
        self.tab_submenus.push(
            SubMenu::new_with_toggles::<T>(
                submenu_title,
                submenu_id,
                help_text,
                is_single_option,
            )
        );
    }
}

#[derive(Content)]
struct Menu<'a> {
    tabs: Vec<Tab<'a>>,
}

pub fn get_menu_from_url(mut menu: TrainingModpackMenu, s: &str, defaults: bool) -> TrainingModpackMenu {
    let base_url_len = "http://localhost/?".len();
    let total_len = s.len();

    let ss: String = s
        .chars()
        .skip(base_url_len)
        .take(total_len - base_url_len)
        .collect();

    for toggle_values in ss.split('&') {
        let toggle_value_split = toggle_values.split('=').collect::<Vec<&str>>();
        let mut toggle = toggle_value_split[0];
        if toggle.is_empty() | (
            // Default menu settings begin with the prefix "__"
            // So if skip toggles without the prefix if defaults is true
            // And skip toggles with the prefix if defaults is false
            defaults ^ toggle.starts_with("__")
        ) { continue }
        toggle = toggle.strip_prefix("__").unwrap_or(toggle);

        let bits: u32 = toggle_value_split[1].parse().unwrap_or(0);
        menu.set(toggle, bits);
    }
    menu
}

pub unsafe fn menu_condition(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> bool {
    // Only check for button combination if the counter is 0 (not locked out)
    match frame_counter::get_frame_count(FRAME_COUNTER_INDEX) {
        0 => {
            ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
                && ControlModule::check_button_on_trriger(
                    module_accessor,
                    *CONTROL_PAD_BUTTON_APPEAL_HI,
                )
        }
        1..MENU_LOCKOUT_FRAMES => false,
        _ => {
            // Waited longer than the lockout time, reset the counter so the menu can be opened again
            frame_counter::full_reset(FRAME_COUNTER_INDEX);
            false
        }
    }
}

pub unsafe fn write_menu() {
    let tpl = Template::new(include_str!("../templates/menu.html")).unwrap();

    let mut overall_menu = Menu {
        tabs: Vec::new(),
    };

    let mut mash_tab = Tab {
        tab_id: "mash",
        tab_title: "Mash Settings",
        tab_submenus: Vec::new(),
    };
    mash_tab.add_submenu_with_toggles::<Action>(
        "Mash Toggles",
        "mash_state",
        "Mash Toggles: Actions to be performed as soon as possible",
        false,
    );
    mash_tab.add_submenu_with_toggles::<Action>(
        "Followup Toggles",
        "follow_up",
        "Followup Toggles: Actions to be performed after the Mash option",
        false,
    );
    mash_tab.add_submenu_with_toggles::<AttackAngle>(
        "Attack Angle",
        "attack_angle",
        "Attack Angle: For attacks that can be angled, such as some forward tilts",
        false,
    );
    mash_tab.add_submenu_with_toggles::<ThrowOption>(
        "Throw Options",
        "throw_state",
        "Throw Options: Throw to be performed when a grab is landed",
        false,
    );
    mash_tab.add_submenu_with_toggles::<MedDelay>(
        "Throw Delay",
        "throw_delay",
        "Throw Delay: How many frames to delay the throw option",
        false,
    );
    mash_tab.add_submenu_with_toggles::<MedDelay>(
        "Pummel Delay",
        "pummel_delay",
        "Pummel Delay: How many frames after a grab to wait before starting to pummel",
        false,
    );
    mash_tab.add_submenu_with_toggles::<BoolFlag>(
        "Falling Aerials",
        "falling_aerials",
        "Falling Aerials: Should aerials be performed when rising or when falling",
        false, // TODO: Should this be a single option submenu?
    );
    mash_tab.add_submenu_with_toggles::<BoolFlag>(
        "Full Hop",
        "full_hop",
        "Full Hop: Should the CPU perform a full hop or a short hop",
        false,
    );
    mash_tab.add_submenu_with_toggles::<Delay>(
        "Aerial Delay",
        "aerial_delay",
        "Aerial Delay: How long to delay a Mash aerial attack",
        false,
    );
    mash_tab.add_submenu_with_toggles::<BoolFlag>(
        "Fast Fall",
        "fast_fall",
        "Fast Fall: Should the CPU fastfall during a jump",
        false,
    );
    mash_tab.add_submenu_with_toggles::<Delay>(
        "Fast Fall Delay",
        "fast_fall_delay",
        "Fast Fall Delay: How many frames the CPU should delay their fastfall",
        false,
    );
    mash_tab.add_submenu_with_toggles::<Delay>(
        "OoS Offset",
        "oos_offset",
        "OoS Offset: How many times the CPU shield can be hit before performing a Mash option",
        false,
    );
    mash_tab.add_submenu_with_toggles::<Delay>(
        "Reaction Time",
        "reaction_time",
        "Reaction Time: How many frames to delay before performing a mash option",
        false,
    );
    mash_tab.add_submenu_with_toggles::<OnOff>(
        "Mash in Neutral",
        "mash_in_neutral",
        "Mash In Neutral: Should Mash options be performed repeatedly or only when the CPU is hit",
        true,
    );
    overall_menu.tabs.push(mash_tab);


    let mut defensive_tab = Tab {
        tab_id: "defensive",
        tab_title: "Defensive Settings",
        tab_submenus: Vec::new(),
    };
    defensive_tab.add_submenu_with_toggles::<Direction>(
        "Airdodge Direction",
        "air_dodge_dir",
        "Airdodge Direction: Direction to angle airdodges",
        false,
    );
    defensive_tab.add_submenu_with_toggles::<Direction>(
        "DI Direction",
        "di_state",
        "DI Direction: Direction to angle the directional influence during hitlag",
        false,
    );
    defensive_tab.add_submenu_with_toggles::<Direction>(
        "SDI Direction",
        "sdi_state",
        "SDI Direction: Direction to angle the smash directional influence during hitlag",
        false,
    );
    defensive_tab.add_submenu_with_toggles::<SdiStrength>(
        "SDI Strength",
        "sdi_strength",
        "SDI Strength: Relative strength of the smash directional influence inputs",
        true,
    );
    defensive_tab.add_submenu_with_toggles::<LedgeOption>(
        "Ledge Options",
        "ledge_state",
        "Ledge Options: Actions to be taken when on the ledge",
        false,
    );
    defensive_tab.add_submenu_with_toggles::<LongDelay>(
        "Ledge Delay",
        "ledge_delay",
        "Ledge Delay: How many frames to delay the ledge option",
        false,
    );
    defensive_tab.add_submenu_with_toggles::<TechFlags>(
        "Tech Options",
        "tech_state",
        "Tech Options: Actions to take when slammed into a hard surface",
        false,
    );
    defensive_tab.add_submenu_with_toggles::<MissTechFlags>(
        "Mistech Options",
        "miss_tech_state",
        "Mistech Options: Actions to take after missing a tech",
        false,
    );
    defensive_tab.add_submenu_with_toggles::<Shield>(
        "Shield Toggles",
        "shield_state",
        "Shield Toggles: CPU Shield Behavior",
        true,
    );
    defensive_tab.add_submenu_with_toggles::<Direction>(
        "Shield Tilt",
        "shield_tilt",
        "Shield Tilt: Direction to tilt the shield",
        false, // TODO: Should this be true?
    );
    defensive_tab.add_submenu_with_toggles::<Defensive>(
        "Defensive Toggles",
        "defensive_state",
        "Defensive Options: Actions to take after a ledge option, tech option, or mistech option",
        false,
    );
    defensive_tab.add_submenu_with_toggles::<BuffOption>(
        "Buff Options",
        "buff_state",
        "Buff Options: Buff(s) to be applied to respective character when loading save states",
        false,
    );
    overall_menu.tabs.push(defensive_tab);

    let mut misc_tab = Tab {
        tab_id: "misc",
        tab_title: "Misc Settings",
        tab_submenus: Vec::new(),
    };
    misc_tab.add_submenu_with_toggles::<SaveStateMirroring>(
        "Mirroring",
        "save_state_mirroring",
        "Mirroring: Flips save states in the left-right direction across the stage center",
        true,
    );
    misc_tab.add_submenu_with_toggles::<OnOff>(
        "Save Damage",
        "save_damage",
        "Save Damage: Should save states retain player/CPU damage",
        true,
    );
    misc_tab.add_submenu_with_toggles::<OnOff>(
        "Enable Save States",
        "save_state_enable",
        "Save States: Enable save states! Save a state with Grab+Down Taunt, load it with Grab+Up Taunt.",
        true,
    );
    misc_tab.add_submenu_with_toggles::<OnOff>(
        "Frame Advantage",
        "frame_advantage",
        "Frame Advantage: Display the time difference between when the player is actionable and the CPU is actionable",
        true,
    );
    misc_tab.add_submenu_with_toggles::<OnOff>(
        "Hitbox Visualization",
        "hitbox_vis",
        "Hitbox Visualization: Should hitboxes be displayed, hiding other visual effects",
        true,
    );
    misc_tab.add_submenu_with_toggles::<Delay>(
        "Input Delay",
        "input_delay",
        "Input Delay: Frames to delay player inputs by",
        true,
    );
    misc_tab.add_submenu_with_toggles::<OnOff>(
        "Stage Hazards",
        "stage_hazards",
        "Stage Hazards: Should stage hazards be present",
        true
    );
    overall_menu.tabs.push(misc_tab);

    let data = tpl.render(&overall_menu);

    // Now that we have the html, write it to file
    // From skyline-web
    let program_id = get_program_id();
    let htdocs_dir = "training_modpack";
    let path = Path::new("sd:/atmosphere/contents")
        .join(&format!("{:016X}", program_id))
        .join(&format!("manual_html/html-document/{}.htdocs/", htdocs_dir))
        .join("training_menu.html");
    fs::write(path, data).unwrap();
}

const MENU_CONF_PATH: &str = "sd:/TrainingModpack/training_modpack_menu.conf";

pub fn spawn_menu() {
    unsafe {
        frame_counter::reset_frame_count(FRAME_COUNTER_INDEX);
        frame_counter::start_counting(FRAME_COUNTER_INDEX);
    }

    let fname = "training_menu.html";
    let params = unsafe { MENU.to_url_params(false) };
    let default_params = unsafe { DEFAULT_MENU.to_url_params(true) };
    let page_response = Webpage::new()
        .background(Background::BlurredScreenshot)
        .htdocs_dir("training_modpack")
        .boot_display(BootDisplay::BlurredScreenshot)
        .boot_icon(true)
        .start_page(&format!("{}?{}&{}", fname, params, default_params))
        .open()
        .unwrap();

    let last_url = page_response.get_last_url().unwrap();
    unsafe {
        MENU = get_menu_from_url(MENU, last_url, false);
        DEFAULT_MENU = get_menu_from_url(DEFAULT_MENU, last_url, true);
    }
    std::fs::write(MENU_CONF_PATH, last_url).expect("Failed to write menu conf file");
    unsafe {
        EVENT_QUEUE.push(Event::menu_open(last_url.to_string()));
    }
}
