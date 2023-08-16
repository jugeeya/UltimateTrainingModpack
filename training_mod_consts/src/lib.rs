#![feature(iter_intersperse)]
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
    pub save_state_playback: PlaybackSlot,
    pub sdi_state: Direction,
    pub sdi_strength: SdiFrequency,
    pub shield_state: Shield,
    pub shield_tilt: Direction,
    pub stage_hazards: OnOff,
    pub tech_state: TechFlags,
    pub throw_delay: MedDelay,
    pub throw_state: ThrowOption,
    pub ledge_neutral_override: Action,
    pub ledge_roll_override: Action,
    pub ledge_jump_override: Action,
    pub ledge_attack_override: Action,
    pub tech_action_override: Action,
    pub clatter_override: Action,
    pub tumble_override: Action,
    pub hitstun_override: Action,
    pub parry_override: Action,
    pub shieldstun_override: Action,
    pub footstool_override: Action,
    pub landing_override: Action,
    pub trump_override: Action,
    pub recording_slot: RecordSlot,
    pub record_trigger: RecordTrigger,
    pub recording_frames: RecordingFrames,
    pub playback_button_combination: PlaybackSlot,
    pub hitstun_playback: HitstunPlayback,
    pub playback_mash: OnOff,
    pub playback_loop: OnOff,
    pub menu_open: ButtonConfig,
    pub save_state_save: ButtonConfig,
    pub save_state_load: ButtonConfig,
    pub input_record: ButtonConfig,
    pub input_playback: ButtonConfig,
    pub recording_crop: OnOff,
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
    pub fn from_string(s: &String) -> SubMenuType {
        match s.as_str() {
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
    save_state_playback: PlaybackSlot::empty(),
    sdi_state: Direction::empty(),
    sdi_strength: SdiFrequency::None,
    shield_state: Shield::None,
    shield_tilt: Direction::empty(),
    stage_hazards: OnOff::Off,
    tech_state: TechFlags::all(),
    throw_delay: MedDelay::empty(),
    throw_state: ThrowOption::NONE,
    ledge_neutral_override: Action::empty(),
    ledge_roll_override: Action::empty(),
    ledge_jump_override: Action::empty(),
    ledge_attack_override: Action::empty(),
    tech_action_override: Action::empty(),
    clatter_override: Action::empty(),
    tumble_override: Action::empty(),
    hitstun_override: Action::empty(),
    parry_override: Action::empty(),
    shieldstun_override: Action::empty(),
    footstool_override: Action::empty(),
    landing_override: Action::empty(),
    trump_override: Action::empty(),
    recording_slot: RecordSlot::S1,
    recording_frames: RecordingFrames::F150,
    record_trigger: RecordTrigger::COMMAND,
    playback_button_combination: PlaybackSlot::S1,
    hitstun_playback: HitstunPlayback::Hitstun,
    playback_mash: OnOff::On,
    playback_loop: OnOff::Off,
    menu_open: ButtonConfig::B.union(ButtonConfig::DPAD_UP),
    save_state_save: ButtonConfig::ZL.union(ButtonConfig::DPAD_DOWN),
    save_state_load: ButtonConfig::ZL.union(ButtonConfig::DPAD_UP),
    input_record: ButtonConfig::ZR.union(ButtonConfig::DPAD_DOWN),
    input_playback: ButtonConfig::ZR.union(ButtonConfig::DPAD_UP),
    recording_crop: OnOff::On,
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
pub struct Toggle {
    pub toggle_value: u32,
    pub toggle_title: String,
    pub checked: bool,
}

#[derive(Clone, Serialize)]
pub struct SubMenu {
    pub submenu_title: String,
    pub submenu_id: String,
    pub help_text: String,
    pub is_single_option: bool,
    pub toggles: Vec<Toggle>,
    pub slider: Option<Slider>,
    pub _type: String,
}

impl SubMenu {
    pub fn add_toggle(&mut self, toggle_value: u32, toggle_title: String, checked: bool) {
        self.toggles.push(Toggle {
            toggle_value,
            toggle_title,
            checked,
        });
    }
    pub fn new_with_toggles<T: ToggleTrait>(
        submenu_title: String,
        submenu_id: String,
        help_text: String,
        is_single_option: bool,
        initial_value: &u32,
    ) -> SubMenu {
        let mut instance = SubMenu {
            submenu_title: submenu_title,
            submenu_id: submenu_id,
            help_text: help_text,
            is_single_option: is_single_option,
            toggles: Vec::new(),
            slider: None,
            _type: "toggle".to_string(),
        };

        let values = T::to_toggle_vals();
        let titles = T::to_toggle_strings();
        for i in 0..values.len() {
            let checked: bool =
                (values[i] & initial_value) > 0 || (!values[i] == 0 && initial_value == &0);
            instance.add_toggle(values[i], titles[i].clone(), checked);
        }
        // Select the first option if there's nothing selected atm but it's a single option submenu
        if is_single_option && instance.toggles.iter().all(|t| !t.checked) {
            instance.toggles[0].checked = true;
        }
        instance
    }
    pub fn new_with_slider<S: SliderTrait>(
        submenu_title: String,
        submenu_id: String,
        help_text: String,
        initial_lower_value: &u32,
        initial_upper_value: &u32,
    ) -> SubMenu {
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
            _type: "slider".to_string(),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct Tab {
    pub tab_id: String,
    pub tab_title: String,
    pub tab_submenus: Vec<SubMenu>,
}

impl Tab {
    pub fn add_submenu_with_toggles<T: ToggleTrait>(
        &mut self,
        submenu_title: String,
        submenu_id: String,
        help_text: String,
        is_single_option: bool,
        initial_value: &u32,
    ) {
        self.tab_submenus.push(SubMenu::new_with_toggles::<T>(
            submenu_title.to_string(),
            submenu_id.to_string(),
            help_text.to_string(),
            is_single_option,
            initial_value,
        ));
    }

    pub fn add_submenu_with_slider<S: SliderTrait>(
        &mut self,
        submenu_title: String,
        submenu_id: String,
        help_text: String,
        initial_lower_value: &u32,
        initial_upper_value: &u32,
    ) {
        self.tab_submenus.push(SubMenu::new_with_slider::<S>(
            submenu_title.to_string(),
            submenu_id.to_string(),
            help_text.to_string(),
            initial_lower_value,
            initial_upper_value,
        ))
    }
}

#[derive(Serialize, Clone)]
pub struct UiMenu {
    pub tabs: Vec<Tab>,
}

pub unsafe fn ui_menu(menu: TrainingModpackMenu) -> UiMenu {
    let mut overall_menu = UiMenu { tabs: Vec::new() };

    let mut mash_tab = Tab {
        tab_id: "mash".to_string(),
        tab_title: "Mash Settings".to_string(),
        tab_submenus: Vec::new(),
    };
    mash_tab.add_submenu_with_toggles::<Action>(
        "Mash Toggles".to_string(),
        "mash_state".to_string(),
        "Mash Toggles: Actions to be performed as soon as possible".to_string(),
        false,
        &(menu.mash_state.bits()),
    );
    mash_tab.add_submenu_with_toggles::<Action>(
        "Followup Toggles".to_string(),
        "follow_up".to_string(),
        "Followup Toggles: Actions to be performed after a Mash option".to_string(),
        false,
        &(menu.follow_up.bits()),
    );
    mash_tab.add_submenu_with_toggles::<MashTrigger>(
        "Mash Triggers".to_string(),
        "mash_triggers".to_string(),
        "Mash triggers: Configure what causes the CPU to perform a Mash option".to_string(),
        false,
        &(menu.mash_triggers.bits()),
    );
    mash_tab.add_submenu_with_toggles::<AttackAngle>(
        "Attack Angle".to_string(),
        "attack_angle".to_string(),
        "Attack Angle: For attacks that can be angled, such as some forward tilts".to_string(),
        false,
        &(menu.attack_angle.bits()),
    );
    mash_tab.add_submenu_with_toggles::<ThrowOption>(
        "Throw Options".to_string(),
        "throw_state".to_string(),
        "Throw Options: Throw to be performed when a grab is landed".to_string(),
        false,
        &(menu.throw_state.bits()),
    );
    mash_tab.add_submenu_with_toggles::<MedDelay>(
        "Throw Delay".to_string(),
        "throw_delay".to_string(),
        "Throw Delay: How many frames to delay the throw option".to_string(),
        false,
        &(menu.throw_delay.bits()),
    );
    mash_tab.add_submenu_with_toggles::<MedDelay>(
        "Pummel Delay".to_string(),
        "pummel_delay".to_string(),
        "Pummel Delay: How many frames after a grab to wait before starting to pummel".to_string(),
        false,
        &(menu.pummel_delay.bits()),
    );
    mash_tab.add_submenu_with_toggles::<BoolFlag>(
        "Falling Aerials".to_string(),
        "falling_aerials".to_string(),
        "Falling Aerials: Should aerials be performed when rising or when falling".to_string(),
        false,
        &(menu.falling_aerials.bits()),
    );
    mash_tab.add_submenu_with_toggles::<BoolFlag>(
        "Full Hop".to_string(),
        "full_hop".to_string(),
        "Full Hop: Should the CPU perform a full hop or a short hop".to_string(),
        false,
        &(menu.full_hop.bits()),
    );
    mash_tab.add_submenu_with_toggles::<Delay>(
        "Aerial Delay".to_string(),
        "aerial_delay".to_string(),
        "Aerial Delay: How long to delay a Mash aerial attack".to_string(),
        false,
        &(menu.aerial_delay.bits()),
    );
    mash_tab.add_submenu_with_toggles::<BoolFlag>(
        "Fast Fall".to_string(),
        "fast_fall".to_string(),
        "Fast Fall: Should the CPU fastfall during a jump".to_string(),
        false,
        &(menu.fast_fall.bits()),
    );
    mash_tab.add_submenu_with_toggles::<Delay>(
        "Fast Fall Delay".to_string(),
        "fast_fall_delay".to_string(),
        "Fast Fall Delay: How many frames the CPU should delay their fastfall".to_string(),
        false,
        &(menu.fast_fall_delay.bits()),
    );
    mash_tab.add_submenu_with_toggles::<Delay>(
        "OoS Offset".to_string(),
        "oos_offset".to_string(),
        "OoS Offset: How many times the CPU shield can be hit before performing a Mash option"
            .to_string(),
        false,
        &(menu.oos_offset.bits()),
    );
    mash_tab.add_submenu_with_toggles::<Delay>(
        "Reaction Time".to_string(),
        "reaction_time".to_string(),
        "Reaction Time: How many frames to delay before performing a mash option".to_string(),
        false,
        &(menu.reaction_time.bits()),
    );
    overall_menu.tabs.push(mash_tab);

    let mut override_tab = Tab {
        tab_id: "override".to_string(),
        tab_title: "Override Settings".to_string(),
        tab_submenus: Vec::new(),
    };
    override_tab.add_submenu_with_toggles::<Action>(
        "Ledge Neutral Getup".to_string(),
        "ledge_neutral_override".to_string(),
        "Neutral Getup Override: Mash Actions to be performed after a Neutral Getup from ledge"
            .to_string(),
        false,
        &(menu.ledge_neutral_override.bits()),
    );
    override_tab.add_submenu_with_toggles::<Action>(
        "Ledge Roll".to_string(),
        "ledge_roll_override".to_string(),
        "Ledge Roll Override: Mash Actions to be performed after a Roll Getup from ledge"
            .to_string(),
        false,
        &(menu.ledge_roll_override.bits()),
    );
    override_tab.add_submenu_with_toggles::<Action>(
        "Ledge Jump".to_string(),
        "ledge_jump_override".to_string(),
        "Ledge Jump Override: Mash Actions to be performed after a Jump Getup from ledge"
            .to_string(),
        false,
        &(menu.ledge_jump_override.bits()),
    );
    override_tab.add_submenu_with_toggles::<Action>(
        "Ledge Attack".to_string(),
        "ledge_attack_override".to_string(),
        "Ledge Attack Override: Mash Actions to be performed after a Getup Attack from ledge"
            .to_string(),
        false,
        &(menu.ledge_attack_override.bits()),
    );
    override_tab.add_submenu_with_toggles::<Action>(
        "Tech Action".to_string(),
        "tech_action_override".to_string(),
        "Tech Action Override: Mash Actions to be performed after any tech action".to_string(),
        false,
        &(menu.tech_action_override.bits()),
    );
    override_tab.add_submenu_with_toggles::<Action>(
        "Clatter".to_string(),
        "clatter_override".to_string(),
        "Clatter Override: Mash Actions to be performed after leaving a clatter situation (grab.to_string(), bury, etc)".to_string(),
        false,
        &(menu.clatter_override.bits()),
    );
    override_tab.add_submenu_with_toggles::<Action>(
        "Tumble".to_string(),
        "tumble_override".to_string(),
        "Tumble Override: Mash Actions to be performed after exiting a tumble state".to_string(),
        false,
        &(menu.tumble_override.bits()),
    );
    override_tab.add_submenu_with_toggles::<Action>(
        "Hitstun".to_string(),
        "hitstun_override".to_string(),
        "Hitstun Override: Mash Actions to be performed after exiting a hitstun state".to_string(),
        false,
        &(menu.hitstun_override.bits()),
    );
    override_tab.add_submenu_with_toggles::<Action>(
        "Parry".to_string(),
        "parry_override".to_string(),
        "Parry Override: Mash Actions to be performed after a parry".to_string(),
        false,
        &(menu.parry_override.bits()),
    );
    override_tab.add_submenu_with_toggles::<Action>(
        "Shieldstun".to_string(),
        "shieldstun_override".to_string(),
        "Shieldstun Override: Mash Actions to be performed after exiting a shieldstun state"
            .to_string(),
        false,
        &(menu.shieldstun_override.bits()),
    );
    override_tab.add_submenu_with_toggles::<Action>(
        "Footstool".to_string(),
        "footstool_override".to_string(),
        "Footstool Override: Mash Actions to be performed after exiting a footstool state"
            .to_string(),
        false,
        &(menu.footstool_override.bits()),
    );
    override_tab.add_submenu_with_toggles::<Action>(
        "Landing".to_string(),
        "landing_override".to_string(),
        "Landing Override: Mash Actions to be performed after landing on the ground".to_string(),
        false,
        &(menu.landing_override.bits()),
    );
    override_tab.add_submenu_with_toggles::<Action>(
        "Ledge Trump".to_string(),
        "trump_override".to_string(),
        "Ledge Trump Override: Mash Actions to be performed after leaving a ledgetrump state"
            .to_string(),
        false,
        &(menu.trump_override.bits()),
    );
    overall_menu.tabs.push(override_tab);

    let mut defensive_tab = Tab {
        tab_id: "defensive".to_string(),
        tab_title: "Defensive Settings".to_string(),
        tab_submenus: Vec::new(),
    };
    defensive_tab.add_submenu_with_toggles::<Direction>(
        "Airdodge Direction".to_string(),
        "air_dodge_dir".to_string(),
        "Airdodge Direction: Direction to angle airdodges".to_string(),
        false,
        &(menu.air_dodge_dir.bits()),
    );
    defensive_tab.add_submenu_with_toggles::<Direction>(
        "DI Direction".to_string(),
        "di_state".to_string(),
        "DI Direction: Direction to angle the directional influence during hitlag".to_string(),
        false,
        &(menu.di_state.bits()),
    );
    defensive_tab.add_submenu_with_toggles::<Direction>(
        "SDI Direction".to_string(),
        "sdi_state".to_string(),
        "SDI Direction: Direction to angle the smash directional influence during hitlag"
            .to_string(),
        false,
        &(menu.sdi_state.bits()),
    );
    defensive_tab.add_submenu_with_toggles::<SdiFrequency>(
        "SDI Strength".to_string(),
        "sdi_strength".to_string(),
        "SDI Strength: Relative strength of the smash directional influence inputs".to_string(),
        true,
        &(menu.sdi_strength as u32),
    );
    defensive_tab.add_submenu_with_toggles::<ClatterFrequency>(
        "Clatter Strength".to_string(),
        "clatter_strength".to_string(),
        "Clatter Strength: Configure how rapidly the CPU will mash out of grabs, buries, etc."
            .to_string(),
        true,
        &(menu.clatter_strength as u32),
    );
    defensive_tab.add_submenu_with_toggles::<LedgeOption>(
        "Ledge Options".to_string(),
        "ledge_state".to_string(),
        "Ledge Options: Actions to be taken when on the ledge".to_string(),
        false,
        &(menu.ledge_state.bits()),
    );
    defensive_tab.add_submenu_with_toggles::<LongDelay>(
        "Ledge Delay".to_string(),
        "ledge_delay".to_string(),
        "Ledge Delay: How many frames to delay the ledge option".to_string(),
        false,
        &(menu.ledge_delay.bits()),
    );
    defensive_tab.add_submenu_with_toggles::<TechFlags>(
        "Tech Options".to_string(),
        "tech_state".to_string(),
        "Tech Options: Actions to take when slammed into a hard surface".to_string(),
        false,
        &(menu.tech_state.bits()),
    );
    defensive_tab.add_submenu_with_toggles::<MissTechFlags>(
        "Mistech Options".to_string(),
        "miss_tech_state".to_string(),
        "Mistech Options: Actions to take after missing a tech".to_string(),
        false,
        &(menu.miss_tech_state.bits()),
    );
    defensive_tab.add_submenu_with_toggles::<Shield>(
        "Shield Toggles".to_string(),
        "shield_state".to_string(),
        "Shield Toggles: CPU Shield Behavior".to_string(),
        true,
        &(menu.shield_state as u32),
    );
    defensive_tab.add_submenu_with_toggles::<Direction>(
        "Shield Tilt".to_string(),
        "shield_tilt".to_string(),
        "Shield Tilt: Direction to tilt the shield".to_string(),
        false, // TODO: Should this be true?
        &(menu.shield_tilt.bits()),
    );

    defensive_tab.add_submenu_with_toggles::<OnOff>(
        "Crouch".to_string(),
        "crouch".to_string(),
        "Crouch: Have the CPU crouch when on the ground".to_string(),
        true,
        &(menu.crouch as u32),
    );
    overall_menu.tabs.push(defensive_tab);

    let mut save_state_tab = Tab {
        tab_id: "save_state".to_string(),
        tab_title: "Save States".to_string(),
        tab_submenus: Vec::new(),
    };
    save_state_tab.add_submenu_with_toggles::<SaveStateMirroring>(
        "Mirroring".to_string(),
        "save_state_mirroring".to_string(),
        "Mirroring: Flips save states in the left-right direction across the stage center"
            .to_string(),
        true,
        &(menu.save_state_mirroring as u32),
    );
    save_state_tab.add_submenu_with_toggles::<OnOff>(
        "Auto Save States".to_string(),
        "save_state_autoload".to_string(),
        "Auto Save States: Load save state when any fighter dies".to_string(),
        true,
        &(menu.save_state_autoload as u32),
    );
    save_state_tab.add_submenu_with_toggles::<SaveDamage>(
        "Save Dmg (CPU)".to_string(),
        "save_damage_cpu".to_string(),
        "Save Damage: Should save states retain CPU damage".to_string(),
        true,
        &(menu.save_damage_cpu.bits()),
    );
    save_state_tab.add_submenu_with_slider::<DamagePercent>(
        "Dmg Range (CPU)".to_string(),
        "save_damage_limits_cpu".to_string(),
        "Limits on random damage to apply to the CPU when loading a save state".to_string(),
        &(menu.save_damage_limits_cpu.0 as u32),
        &(menu.save_damage_limits_cpu.1 as u32),
    );
    save_state_tab.add_submenu_with_toggles::<SaveDamage>(
        "Save Dmg (Player)".to_string(),
        "save_damage_player".to_string(),
        "Save Damage: Should save states retain player damage".to_string(),
        true,
        &(menu.save_damage_player.bits() as u32),
    );
    save_state_tab.add_submenu_with_slider::<DamagePercent>(
        "Dmg Range (Player)".to_string(),
        "save_damage_limits_player".to_string(),
        "Limits on random damage to apply to the player when loading a save state".to_string(),
        &(menu.save_damage_limits_player.0 as u32),
        &(menu.save_damage_limits_player.1 as u32),
    );
    save_state_tab.add_submenu_with_toggles::<OnOff>(
        "Enable Save States".to_string(),
        "save_state_enable".to_string(),
        "Save States: Enable save states! Save a state with Shield+Down Taunt, load it with Shield+Up Taunt.".to_string(),
        true,
        &(menu.save_state_enable as u32),
    );
    save_state_tab.add_submenu_with_toggles::<SaveStateSlot>(
        "Save State Slot".to_string(),
        "save_state_slot".to_string(),
        "Save State Slot: Save and load states from different slots.".to_string(),
        true,
        &(menu.save_state_slot as u32),
    );
    save_state_tab.add_submenu_with_toggles::<OnOff>(
        "Randomize Slots".to_string(),
        "randomize_slots".to_string(),
        "Randomize Slots: Randomize slot when loading save state.".to_string(),
        true,
        &(menu.randomize_slots as u32),
    );
    save_state_tab.add_submenu_with_toggles::<CharacterItem>(
        "Character Item".to_string(),
        "character_item".to_string(),
        "Character Item: The item to give to the player's fighter when loading a save state"
            .to_string(),
        true,
        &(menu.character_item as u32),
    );
    save_state_tab.add_submenu_with_toggles::<BuffOption>(
        "Buff Options".to_string(),
        "buff_state".to_string(),
        "Buff Options: Buff(s) to be applied to the respective fighters when loading a save state"
            .to_string(),
        false,
        &(menu.buff_state.bits()),
    );
    save_state_tab.add_submenu_with_toggles::<PlaybackSlot>(
        "Save State Playback".to_string(),
        "save_state_playback".to_string(),
        "Save State Playback: Choose which slots to playback input recording upon loading a save state".to_string(),
        false,
        &(menu.save_state_playback.bits() as u32),
    );
    overall_menu.tabs.push(save_state_tab);

    let mut misc_tab = Tab {
        tab_id: "misc".to_string(),
        tab_title: "Misc Settings".to_string(),
        tab_submenus: Vec::new(),
    };
    misc_tab.add_submenu_with_toggles::<OnOff>(
        "Frame Advantage".to_string(),
        "frame_advantage".to_string(),
        "Frame Advantage: Display the time difference between when the player is actionable and the CPU is actionable".to_string(),
        true,
        &(menu.frame_advantage as u32),
    );
    misc_tab.add_submenu_with_toggles::<OnOff>(
        "Hitbox Visualization".to_string(),
        "hitbox_vis".to_string(),
        "Hitbox Visualization: Display a visual representation for active hitboxes (hides other visual effects)".to_string(),
        true,
        &(menu.hitbox_vis as u32),
    );
    misc_tab.add_submenu_with_toggles::<Delay>(
        "Input Delay".to_string(),
        "input_delay".to_string(),
        "Input Delay: Frames to delay player inputs by".to_string(),
        true,
        &(menu.input_delay.bits()),
    );
    misc_tab.add_submenu_with_toggles::<OnOff>(
        "Stage Hazards".to_string(),
        "stage_hazards".to_string(),
        "Stage Hazards: Turn stage hazards on/off".to_string(),
        true,
        &(menu.stage_hazards as u32),
    );
    misc_tab.add_submenu_with_toggles::<OnOff>(
        "HUD".to_string(),
        "hud".to_string(),
        "HUD: Show/hide elements of the UI".to_string(),
        true,
        &(menu.hud as u32),
    );
    overall_menu.tabs.push(misc_tab);

    let mut input_tab = Tab {
        tab_id: "input".to_string(),
        tab_title: "Input Recording".to_string(),
        tab_submenus: Vec::new(),
    };
    input_tab.add_submenu_with_toggles::<RecordSlot>(
        "Recording Slot".to_string(),
        "recording_slot".to_string(),
        "Recording Slot: Choose which slot to record into".to_string(),
        true,
        &(menu.recording_slot as u32),
    );
    input_tab.add_submenu_with_toggles::<RecordTrigger>(
        "Recording Trigger".to_string(),
        "record_trigger".to_string(),
        format!("Recording Trigger: Whether to begin recording via button combination (Default: {} or upon loading a Save State", menu.input_record.combination_string()),
        false,
        &(menu.record_trigger.bits() as u32),
    );
    input_tab.add_submenu_with_toggles::<RecordingFrames>(
        "Recording Frames".to_string(),
        "recording_frames".to_string(),
        "Recording Frames: Number of frames to record for in the current slot".to_string(),
        true,
        &(menu.recording_frames as u32),
    );
    input_tab.add_submenu_with_toggles::<PlaybackSlot>(
        "Playback Button Combination".to_string(),
        "playback_button_combination".to_string(),
        format!("Playback Button Combination: Choose which slots to playback input recording upon pressing button combination (Default: {})", menu.input_playback.combination_string()),
        false,
        &(menu.playback_button_combination.bits() as u32),
    );
    input_tab.add_submenu_with_toggles::<HitstunPlayback>(
        "Playback Hitstun Timing".to_string(),
        "hitstun_playback".to_string(),
        "Playback Hitstun Timing: When to begin playing back inputs when a hitstun mash trigger occurs".to_string(),
        true,
        &(menu.hitstun_playback as u32),
    );
    input_tab.add_submenu_with_toggles::<OnOff>(
        "Playback Mash Interrupt".to_string(),
        "playback_mash".to_string(),
        "Playback Mash Interrupt: End input playback when a mash trigger occurs".to_string(),
        true,
        &(menu.playback_mash as u32),
    );
    input_tab.add_submenu_with_toggles::<OnOff>(
        "Playback Loop".to_string(),
        "playback_loop".to_string(),
        "Playback Loop: Repeat triggered input playbacks indefinitely".to_string(),
        true,
        &(menu.playback_loop as u32),
    );
    input_tab.add_submenu_with_toggles::<OnOff>(
        "Recording Crop".to_string(),
        "recording_crop".to_string(),
        "Recording Crop: Remove neutral input frames at the end of your recording".to_string(),
        true,
        &(menu.recording_crop as u32),
    );
    overall_menu.tabs.push(input_tab);

    let mut button_tab = Tab {
        tab_id: "button".to_string(),
        tab_title: "Button Config".to_string(),
        tab_submenus: Vec::new(),
    };
    button_tab.add_submenu_with_toggles::<ButtonConfig>(
        "Menu Open".to_string(),
        "menu_open".to_string(),
        "Menu Open: Hold: Hold any one button and press the others to trigger".to_string(),
        false,
        &(menu.menu_open.bits() as u32),
    );
    button_tab.add_submenu_with_toggles::<ButtonConfig>(
        "Save State Save".to_string(),
        "save_state_save".to_string(),
        "Save State Save: Hold any one button and press the others to trigger".to_string(),
        false,
        &(menu.save_state_save.bits() as u32),
    );

    button_tab.add_submenu_with_toggles::<ButtonConfig>(
        "Save State Load".to_string(),
        "save_state_load".to_string(),
        "Save State Load: Hold any one button and press the others to trigger".to_string(),
        false,
        &(menu.save_state_load.bits() as u32),
    );
    button_tab.add_submenu_with_toggles::<ButtonConfig>(
        "Input Record".to_string(),
        "input_record".to_string(),
        "Input Record: Hold any one button and press the others to trigger".to_string(),
        false,
        &(menu.input_record.bits() as u32),
    );
    button_tab.add_submenu_with_toggles::<ButtonConfig>(
        "Input Playback".to_string(),
        "input_playback".to_string(),
        "Input Playback: Hold any one button and press the others to trigger".to_string(),
        false,
        &(menu.input_playback.bits() as u32),
    );
    overall_menu.tabs.push(button_tab);

    overall_menu
}
