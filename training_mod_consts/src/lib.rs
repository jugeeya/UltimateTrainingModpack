#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate bitflags_serde_shim;

#[macro_use]
extern crate num_derive;

use serde::{Deserialize, Serialize};

pub mod options;
pub use options::*;
pub mod files;
pub use files::*;

#[repr(C)]
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct TrainingModpackMenu {
    pub aerial_delay: Delay,
    pub air_dodge_dir: Direction,
    pub attack_angle: AttackAngle,
    pub buff_state: BuffOption,
    pub character_item: CharacterItem,
    pub clatter_strength: ClatterFrequency,
    pub crouch: OnOff,
    pub di_state: Direction,
    pub falling_aerials: BoolFlag,
    pub fast_fall_delay: Delay,
    pub fast_fall: BoolFlag,
    pub follow_up: Action,
    pub frame_advantage: OnOff,
    pub full_hop: BoolFlag,
    pub hitbox_vis: OnOff,
    pub hud: OnOff,
    pub input_delay: Delay,
    pub ledge_delay: LongDelay,
    pub ledge_state: LedgeOption,
    pub mash_state: Action,
    pub mash_triggers: MashTrigger,
    pub miss_tech_state: MissTechFlags,
    pub oos_offset: Delay,
    pub pummel_delay: MedDelay,
    pub reaction_time: Delay,
    pub save_damage_cpu: SaveDamage,
    pub save_damage_limits_cpu: DamagePercent,
    pub save_damage_player: SaveDamage,
    pub save_damage_limits_player: DamagePercent,
    pub save_state_autoload: OnOff,
    pub save_state_enable: OnOff,
    pub save_state_slot: SaveStateSlot,
    pub randomize_slots: OnOff,
    pub save_state_mirroring: SaveStateMirroring,
    pub sdi_state: Direction,
    pub sdi_strength: SdiFrequency,
    pub shield_state: Shield,
    pub shield_tilt: Direction,
    pub stage_hazards: OnOff,
    pub tech_state: TechFlags,
    pub throw_delay: MedDelay,
    pub throw_state: ThrowOption,
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct MenuJsonStruct {
    pub menu: TrainingModpackMenu,
    pub defaults_menu: TrainingModpackMenu,
    // pub last_focused_submenu: &str
}

// Fighter Ids
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FighterId {
    Player = 0,
    CPU = 1,
}

#[derive(Clone)]
pub enum SubMenuType {
    TOGGLE,
    SLIDER,
}

impl SubMenuType {
    pub fn from_str(s: &str) -> SubMenuType {
        match s {
            "toggle" => SubMenuType::TOGGLE,
            "slider" => SubMenuType::SLIDER,
            _ => panic!("Unexpected SubMenuType!"),
        }
    }
}

pub static DEFAULTS_MENU: TrainingModpackMenu = TrainingModpackMenu {
    aerial_delay: Delay::empty(),
    air_dodge_dir: Direction::empty(),
    attack_angle: AttackAngle::empty(),
    buff_state: BuffOption::empty(),
    character_item: CharacterItem::None,
    clatter_strength: ClatterFrequency::None,
    crouch: OnOff::Off,
    di_state: Direction::empty(),
    falling_aerials: BoolFlag::FALSE,
    fast_fall_delay: Delay::empty(),
    fast_fall: BoolFlag::FALSE,
    follow_up: Action::empty(),
    frame_advantage: OnOff::Off,
    full_hop: BoolFlag::TRUE,
    hitbox_vis: OnOff::On,
    hud: OnOff::On,
    input_delay: Delay::D0,
    ledge_delay: LongDelay::empty(),
    ledge_state: LedgeOption::default(),
    mash_state: Action::empty(),
    mash_triggers: MashTrigger::default(),
    miss_tech_state: MissTechFlags::all(),
    oos_offset: Delay::empty(),
    pummel_delay: MedDelay::empty(),
    reaction_time: Delay::empty(),
    save_damage_cpu: SaveDamage::DEFAULT,
    save_damage_limits_cpu: DamagePercent::default(),
    save_damage_player: SaveDamage::DEFAULT,
    save_damage_limits_player: DamagePercent::default(),
    save_state_autoload: OnOff::Off,
    save_state_enable: OnOff::On,
    save_state_slot: SaveStateSlot::One,
    randomize_slots: OnOff::Off,
    save_state_mirroring: SaveStateMirroring::None,
    sdi_state: Direction::empty(),
    sdi_strength: SdiFrequency::None,
    shield_state: Shield::None,
    shield_tilt: Direction::empty(),
    stage_hazards: OnOff::Off,
    tech_state: TechFlags::all(),
    throw_delay: MedDelay::empty(),
    throw_state: ThrowOption::NONE,
};

pub static mut MENU: TrainingModpackMenu = DEFAULTS_MENU;

#[derive(Clone, Serialize)]
pub struct Slider {
    pub selected_min: u32,
    pub selected_max: u32,
    pub abs_min: u32,
    pub abs_max: u32,
}

#[derive(Clone, Serialize)]
pub struct Toggle<'a> {
    pub toggle_value: u32,
    pub toggle_title: &'a str,
    pub checked: bool,
}

#[derive(Clone, Serialize)]
pub struct SubMenu<'a> {
    pub submenu_title: &'a str,
    pub submenu_id: &'a str,
    pub help_text: &'a str,
    pub is_single_option: bool,
    pub toggles: Vec<Toggle<'a>>,
    pub slider: Option<Slider>,
    pub _type: &'a str,
}

impl<'a> SubMenu<'a> {
    pub fn add_toggle(&mut self, toggle_value: u32, toggle_title: &'a str, checked: bool) {
        self.toggles.push(Toggle {
            toggle_value,
            toggle_title,
            checked,
        });
    }
    pub fn new_with_toggles<T: ToggleTrait>(
        submenu_title: &'a str,
        submenu_id: &'a str,
        help_text: &'a str,
        is_single_option: bool,
        initial_value: &u32,
    ) -> SubMenu<'a> {
        let mut instance = SubMenu {
            submenu_title: submenu_title,
            submenu_id: submenu_id,
            help_text: help_text,
            is_single_option: is_single_option,
            toggles: Vec::new(),
            slider: None,
            _type: "toggle",
        };

        let values = T::to_toggle_vals();
        let titles = T::to_toggle_strs();
        for i in 0..values.len() {
            let checked: bool =
                (values[i] & initial_value) > 0 || (!values[i] == 0 && initial_value == &0);
            instance.add_toggle(values[i], titles[i], checked);
        }
        // Select the first option if there's nothing selected atm but it's a single option submenu
        if is_single_option && instance.toggles.iter().all(|t| !t.checked) {
            instance.toggles[0].checked = true;
        }
        instance
    }
    pub fn new_with_slider<S: SliderTrait>(
        submenu_title: &'a str,
        submenu_id: &'a str,
        help_text: &'a str,
        initial_lower_value: &u32,
        initial_upper_value: &u32,
    ) -> SubMenu<'a> {
        let min_max = S::get_limits();
        SubMenu {
            submenu_title: submenu_title,
            submenu_id: submenu_id,
            help_text: help_text,
            is_single_option: false,
            toggles: Vec::new(),
            slider: Some(Slider {
                selected_min: *initial_lower_value,
                selected_max: *initial_upper_value,
                abs_min: min_max.0,
                abs_max: min_max.1,
            }),
            _type: "slider",
        }
    }
}

#[derive(Serialize, Clone)]
pub struct Tab<'a> {
    pub tab_id: &'a str,
    pub tab_title: &'a str,
    pub tab_submenus: Vec<SubMenu<'a>>,
}

impl<'a> Tab<'a> {
    pub fn add_submenu_with_toggles<T: ToggleTrait>(
        &mut self,
        submenu_title: &'a str,
        submenu_id: &'a str,
        help_text: &'a str,
        is_single_option: bool,
        initial_value: &u32,
    ) {
        self.tab_submenus.push(SubMenu::new_with_toggles::<T>(
            submenu_title,
            submenu_id,
            help_text,
            is_single_option,
            initial_value,
        ));
    }

    pub fn add_submenu_with_slider<S: SliderTrait>(
        &mut self,
        submenu_title: &'a str,
        submenu_id: &'a str,
        help_text: &'a str,
        initial_lower_value: &u32,
        initial_upper_value: &u32,
    ) {
        self.tab_submenus.push(SubMenu::new_with_slider::<S>(
            submenu_title,
            submenu_id,
            help_text,
            initial_lower_value,
            initial_upper_value,
        ))
    }
}

#[derive(Serialize, Clone)]
pub struct UiMenu<'a> {
    pub tabs: Vec<Tab<'a>>,
}

pub unsafe fn ui_menu(menu: TrainingModpackMenu) -> UiMenu<'static> {
    let mut overall_menu = UiMenu { tabs: Vec::new() };

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
        &(menu.mash_state.bits()),
    );
    mash_tab.add_submenu_with_toggles::<Action>(
        "Followup Toggles",
        "follow_up",
        "Followup Toggles: Actions to be performed after a Mash option",
        false,
        &(menu.follow_up.bits()),
    );
    mash_tab.add_submenu_with_toggles::<MashTrigger>(
        "Mash Triggers",
        "mash_triggers",
        "Mash triggers: Configure what causes the CPU to perform a Mash option",
        false,
        &(menu.mash_triggers.bits()),
    );
    mash_tab.add_submenu_with_toggles::<AttackAngle>(
        "Attack Angle",
        "attack_angle",
        "Attack Angle: For attacks that can be angled, such as some forward tilts",
        false,
        &(menu.attack_angle.bits()),
    );
    mash_tab.add_submenu_with_toggles::<ThrowOption>(
        "Throw Options",
        "throw_state",
        "Throw Options: Throw to be performed when a grab is landed",
        false,
        &(menu.throw_state.bits()),
    );
    mash_tab.add_submenu_with_toggles::<MedDelay>(
        "Throw Delay",
        "throw_delay",
        "Throw Delay: How many frames to delay the throw option",
        false,
        &(menu.throw_delay.bits()),
    );
    mash_tab.add_submenu_with_toggles::<MedDelay>(
        "Pummel Delay",
        "pummel_delay",
        "Pummel Delay: How many frames after a grab to wait before starting to pummel",
        false,
        &(menu.pummel_delay.bits()),
    );
    mash_tab.add_submenu_with_toggles::<BoolFlag>(
        "Falling Aerials",
        "falling_aerials",
        "Falling Aerials: Should aerials be performed when rising or when falling",
        false,
        &(menu.falling_aerials.bits()),
    );
    mash_tab.add_submenu_with_toggles::<BoolFlag>(
        "Full Hop",
        "full_hop",
        "Full Hop: Should the CPU perform a full hop or a short hop",
        false,
        &(menu.full_hop.bits()),
    );
    mash_tab.add_submenu_with_toggles::<Delay>(
        "Aerial Delay",
        "aerial_delay",
        "Aerial Delay: How long to delay a Mash aerial attack",
        false,
        &(menu.aerial_delay.bits()),
    );
    mash_tab.add_submenu_with_toggles::<BoolFlag>(
        "Fast Fall",
        "fast_fall",
        "Fast Fall: Should the CPU fastfall during a jump",
        false,
        &(menu.fast_fall.bits()),
    );
    mash_tab.add_submenu_with_toggles::<Delay>(
        "Fast Fall Delay",
        "fast_fall_delay",
        "Fast Fall Delay: How many frames the CPU should delay their fastfall",
        false,
        &(menu.fast_fall_delay.bits()),
    );
    mash_tab.add_submenu_with_toggles::<Delay>(
        "OoS Offset",
        "oos_offset",
        "OoS Offset: How many times the CPU shield can be hit before performing a Mash option",
        false,
        &(menu.oos_offset.bits()),
    );
    mash_tab.add_submenu_with_toggles::<Delay>(
        "Reaction Time",
        "reaction_time",
        "Reaction Time: How many frames to delay before performing a mash option",
        false,
        &(menu.reaction_time.bits()),
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
        &(menu.air_dodge_dir.bits()),
    );
    defensive_tab.add_submenu_with_toggles::<Direction>(
        "DI Direction",
        "di_state",
        "DI Direction: Direction to angle the directional influence during hitlag",
        false,
        &(menu.di_state.bits()),
    );
    defensive_tab.add_submenu_with_toggles::<Direction>(
        "SDI Direction",
        "sdi_state",
        "SDI Direction: Direction to angle the smash directional influence during hitlag",
        false,
        &(menu.sdi_state.bits()),
    );
    defensive_tab.add_submenu_with_toggles::<SdiFrequency>(
        "SDI Strength",
        "sdi_strength",
        "SDI Strength: Relative strength of the smash directional influence inputs",
        true,
        &(menu.sdi_strength as u32),
    );
    defensive_tab.add_submenu_with_toggles::<ClatterFrequency>(
        "Clatter Strength",
        "clatter_strength",
        "Clatter Strength: Configure how rapidly the CPU will mash out of grabs, buries, etc.",
        true,
        &(menu.clatter_strength as u32),
    );
    defensive_tab.add_submenu_with_toggles::<LedgeOption>(
        "Ledge Options",
        "ledge_state",
        "Ledge Options: Actions to be taken when on the ledge",
        false,
        &(menu.ledge_state.bits()),
    );
    defensive_tab.add_submenu_with_toggles::<LongDelay>(
        "Ledge Delay",
        "ledge_delay",
        "Ledge Delay: How many frames to delay the ledge option",
        false,
        &(menu.ledge_delay.bits()),
    );
    defensive_tab.add_submenu_with_toggles::<TechFlags>(
        "Tech Options",
        "tech_state",
        "Tech Options: Actions to take when slammed into a hard surface",
        false,
        &(menu.tech_state.bits()),
    );
    defensive_tab.add_submenu_with_toggles::<MissTechFlags>(
        "Mistech Options",
        "miss_tech_state",
        "Mistech Options: Actions to take after missing a tech",
        false,
        &(menu.miss_tech_state.bits()),
    );
    defensive_tab.add_submenu_with_toggles::<Shield>(
        "Shield Toggles",
        "shield_state",
        "Shield Toggles: CPU Shield Behavior",
        true,
        &(menu.shield_state as u32),
    );
    defensive_tab.add_submenu_with_toggles::<Direction>(
        "Shield Tilt",
        "shield_tilt",
        "Shield Tilt: Direction to tilt the shield",
        false, // TODO: Should this be true?
        &(menu.shield_tilt.bits()),
    );

    defensive_tab.add_submenu_with_toggles::<OnOff>(
        "Crouch",
        "crouch",
        "Crouch: Have the CPU crouch when on the ground",
        true,
        &(menu.crouch as u32),
    );
    overall_menu.tabs.push(defensive_tab);

    let mut save_state_tab = Tab {
        tab_id: "save_state",
        tab_title: "Save States",
        tab_submenus: Vec::new(),
    };
    save_state_tab.add_submenu_with_toggles::<SaveStateMirroring>(
        "Mirroring",
        "save_state_mirroring",
        "Mirroring: Flips save states in the left-right direction across the stage center",
        true,
        &(menu.save_state_mirroring as u32),
    );
    save_state_tab.add_submenu_with_toggles::<OnOff>(
        "Auto Save States",
        "save_state_autoload",
        "Auto Save States: Load save state when any fighter dies",
        true,
        &(menu.save_state_autoload as u32),
    );
    save_state_tab.add_submenu_with_toggles::<SaveDamage>(
        "Save Dmg (CPU)",
        "save_damage_cpu",
        "Save Damage: Should save states retain CPU damage",
        true,
        &(menu.save_damage_cpu.bits()),
    );
    save_state_tab.add_submenu_with_slider::<DamagePercent>(
        "Dmg Range (CPU)",
        "save_damage_limits_cpu",
        "Limits on random damage to apply to the CPU when loading a save state",
        &(menu.save_damage_limits_cpu.0 as u32),
        &(menu.save_damage_limits_cpu.1 as u32),
    );
    save_state_tab.add_submenu_with_toggles::<SaveDamage>(
        "Save Dmg (Player)",
        "save_damage_player",
        "Save Damage: Should save states retain player damage",
        true,
        &(menu.save_damage_player.bits() as u32),
    );
    save_state_tab.add_submenu_with_slider::<DamagePercent>(
        "Dmg Range (Player)",
        "save_damage_limits_player",
        "Limits on random damage to apply to the player when loading a save state",
        &(menu.save_damage_limits_player.0 as u32),
        &(menu.save_damage_limits_player.1 as u32),
    );
    save_state_tab.add_submenu_with_toggles::<OnOff>(
        "Enable Save States",
        "save_state_enable",
        "Save States: Enable save states! Save a state with Shield+Down Taunt, load it with Shield+Up Taunt.",
        true,
        &(menu.save_state_enable as u32),
    );
    save_state_tab.add_submenu_with_toggles::<SaveStateSlot>(
        "Save State Slot",
        "save_state_slot",
        "Save State Slot: Save and load states from different slots.",
        true,
        &(menu.save_state_slot as u32),
    );
    save_state_tab.add_submenu_with_toggles::<OnOff>(
        "Randomize Slots",
        "randomize_slots",
        "Randomize Slots: Randomize slot when loading save state.",
        true,
        &(menu.randomize_slots as u32),
    );
    save_state_tab.add_submenu_with_toggles::<CharacterItem>(
        "Character Item",
        "character_item",
        "Character Item: The item to give to the player's fighter when loading a save state",
        true,
        &(menu.character_item as u32),
    );
    save_state_tab.add_submenu_with_toggles::<BuffOption>(
        "Buff Options",
        "buff_state",
        "Buff Options: Buff(s) to be applied to the respective fighters when loading a save state",
        false,
        &(menu.buff_state.bits()),
    );
    overall_menu.tabs.push(save_state_tab);

    let mut misc_tab = Tab {
        tab_id: "misc",
        tab_title: "Misc Settings",
        tab_submenus: Vec::new(),
    };
    misc_tab.add_submenu_with_toggles::<OnOff>(
        "Frame Advantage",
        "frame_advantage",
        "Frame Advantage: Display the time difference between when the player is actionable and the CPU is actionable",
        true,
        &(menu.frame_advantage as u32),
    );
    misc_tab.add_submenu_with_toggles::<OnOff>(
        "Hitbox Visualization",
        "hitbox_vis",
        "Hitbox Visualization: Display a visual representation for active hitboxes (hides other visual effects)",
        true,
        &(menu.hitbox_vis as u32),
    );
    misc_tab.add_submenu_with_toggles::<Delay>(
        "Input Delay",
        "input_delay",
        "Input Delay: Frames to delay player inputs by",
        true,
        &(menu.input_delay.bits()),
    );
    misc_tab.add_submenu_with_toggles::<OnOff>(
        "Stage Hazards",
        "stage_hazards",
        "Stage Hazards: Turn stage hazards on/off",
        true,
        &(menu.stage_hazards as u32),
    );
    misc_tab.add_submenu_with_toggles::<OnOff>(
        "HUD",
        "hud",
        "HUD: Show/hide elements of the UI",
        true,
        &(menu.hud as u32),
    );
    overall_menu.tabs.push(misc_tab);

    overall_menu
}
