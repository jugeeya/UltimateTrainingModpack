#![allow(non_snake_case)]
extern crate byteflags;
extern crate num_derive;

use serde::{Deserialize, Serialize};

pub mod options;
pub use options::*;
pub mod files;
pub use files::*;
pub mod config;
pub use config::*;

use training_mod_sync::*;
use training_mod_tui::SubMenuType::*;
pub use training_mod_tui::*;

pub const TOGGLE_MAX: u8 = 5;

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
    pub input_display: InputDisplay,
    pub input_display_status: OnOff,
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
    pub randomize_slots: SaveStateSlot,
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
    pub recording_duration: RecordingDuration,
    pub playback_button_slots: PlaybackSlot,
    pub hitstun_playback: HitstunPlayback,
    pub playback_mash: OnOff,
    pub playback_loop: OnOff,
    pub menu_open_start_press: OnOff,
    pub save_state_save: ButtonConfig,
    pub save_state_load: ButtonConfig,
    pub input_record: ButtonConfig,
    pub input_playback: ButtonConfig,
    pub recording_crop: OnOff,
    pub stale_dodges: OnOff,
    pub tech_hide: OnOff,
    pub update_policy: UpdatePolicy,
    pub lra_reset: OnOff,
    pub selected_locale: Locale,
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct MenuJsonStruct {
    pub menu: TrainingModpackMenu,
    pub defaults_menu: TrainingModpackMenu,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FighterId {
    Player = 0,
    CPU = 1,
}

pub static BASE_MENU: TrainingModpackMenu = TrainingModpackMenu {
    aerial_delay: Delay::empty(),
    air_dodge_dir: Direction::empty(),
    attack_angle: AttackAngle::empty(),
    buff_state: BuffOption::empty(),
    character_item: CharacterItem::NONE,
    clatter_strength: ClatterFrequency::NONE,
    crouch: OnOff::OFF,
    di_state: Direction::empty(),
    falling_aerials: BoolFlag::FALSE,
    fast_fall_delay: Delay::empty(),
    fast_fall: BoolFlag::FALSE,
    follow_up: Action::empty(),
    frame_advantage: OnOff::OFF,
    full_hop: BoolFlag::TRUE,
    hitbox_vis: OnOff::OFF,
    input_display: InputDisplay::SMASH,
    input_display_status: OnOff::OFF,
    hud: OnOff::ON,
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
    save_state_autoload: OnOff::OFF,
    save_state_enable: OnOff::ON,
    save_state_slot: SaveStateSlot::S1,
    randomize_slots: SaveStateSlot::empty(),
    save_state_mirroring: SaveStateMirroring::NONE,
    save_state_playback: PlaybackSlot::empty(),
    sdi_state: Direction::empty(),
    sdi_strength: SdiFrequency::NONE,
    shield_state: Shield::NONE,
    shield_tilt: Direction::empty(),
    stage_hazards: OnOff::OFF,
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
    recording_duration: RecordingDuration::F150,
    record_trigger: RecordTrigger::COMMAND,
    playback_button_slots: PlaybackSlot::S1,
    hitstun_playback: HitstunPlayback::HITSTUN,
    playback_mash: OnOff::ON,
    playback_loop: OnOff::OFF,
    menu_open_start_press: OnOff::ON,
    save_state_save: ButtonConfig {
        ZL: 1,
        DPAD_DOWN: 1,
        ..ButtonConfig::empty()
    },
    save_state_load: ButtonConfig {
        ZL: 1,
        DPAD_UP: 1,
        ..ButtonConfig::empty()
    },
    input_record: ButtonConfig {
        ZR: 1,
        DPAD_DOWN: 1,
        ..ButtonConfig::empty()
    },
    input_playback: ButtonConfig {
        ZR: 1,
        DPAD_UP: 1,
        ..ButtonConfig::empty()
    },
    recording_crop: OnOff::ON,
    stale_dodges: OnOff::ON,
    tech_hide: OnOff::OFF,
    update_policy: UpdatePolicy::default(),
    lra_reset: OnOff::ON,
    selected_locale: Locale::default(),
};

pub static DEFAULTS_MENU: RwLock<TrainingModpackMenu> = RwLock::new(BASE_MENU);
pub static MENU: RwLock<TrainingModpackMenu> = RwLock::new(BASE_MENU);

pub unsafe fn create_app<'a>() -> App<'a> {
    let mut overall_menu = App::new();

    // Mash Tab
    let mut mash_tab_submenus: Vec<SubMenu> = Vec::new();
    mash_tab_submenus.push(Action::to_submenu(
        "menus.mash_settings.mash_toggles.title",
        "mash_state",
        "menus.mash_settings.mash_toggles.description",
        ToggleMultiple,
    ));
    mash_tab_submenus.push(Action::to_submenu(
        "menus.mash_settings.follow_up.title",
        "follow_up",
        "menus.mash_settings.follow_up.description",
        ToggleMultiple,
    ));
    mash_tab_submenus.push(MashTrigger::to_submenu(
        "menus.mash_settings.mash_triggers.title",
        "mash_triggers",
        "menus.mash_settings.mash_triggers.description",
        ToggleSingle,
    ));
    mash_tab_submenus.push(AttackAngle::to_submenu(
        "menus.mash_settings.attack_angle.title",
        "attack_angle",
        "menus.mash_settings.attack_angle.description",
        ToggleMultiple,
    ));
    mash_tab_submenus.push(ThrowOption::to_submenu(
        "menus.mash_settings.throw_options.title",
        "throw_state",
        "menus.mash_settings.throw_options.description",
        ToggleMultiple,
    ));
    mash_tab_submenus.push(MedDelay::to_submenu(
        "menus.mash_settings.throw_delay.title",
        "throw_delay",
        "menus.mash_settings.throw_delay.description",
        ToggleMultiple,
    ));
    mash_tab_submenus.push(MedDelay::to_submenu(
        "menus.mash_settings.pummel_delay.title",
        "pummel_delay",
        "menus.mash_settings.pummel_delay.description",
        ToggleMultiple,
    ));
    mash_tab_submenus.push(BoolFlag::to_submenu(
        "menus.mash_settings.falling_aerials.title",
        "falling_aerials",
        "menus.mash_settings.falling_aerials.description",
        ToggleMultiple,
    ));
    mash_tab_submenus.push(BoolFlag::to_submenu(
        "menus.mash_settings.full_hop.title",
        "full_hop",
        "menus.mash_settings.full_hop.description",
        ToggleMultiple,
    ));
    mash_tab_submenus.push(Delay::to_submenu(
        "menus.mash_settings.aerial_delay.title",
        "aerial_delay",
        "menus.mash_settings.aerial_delay.description",
        ToggleMultiple,
    ));
    mash_tab_submenus.push(BoolFlag::to_submenu(
        "menus.mash_settings.fast_fall.title",
        "fast_fall",
        "menus.mash_settings.fast_fall.description",
        ToggleMultiple,
    ));
    mash_tab_submenus.push(Delay::to_submenu(
        "menus.mash_settings.fast_fall_delay.title",
        "fast_fall_delay",
        "menus.mash_settings.fast_fall_delay.description",
        ToggleMultiple,
    ));
    mash_tab_submenus.push(Delay::to_submenu(
        "menus.mash_settings.oos_offset.title",
        "oos_offset",
        "menus.mash_settings.oos_offset.description",
        ToggleMultiple,
    ));
    mash_tab_submenus.push(Delay::to_submenu(
        "menus.mash_settings.reaction_time.title",
        "reaction_time",
        "menus.mash_settings.reaction_time.description",
        ToggleMultiple,
    ));
    let mash_tab = Tab {
        id: "mash",
        title: "menus.mash_settings.title",
        submenus: StatefulTable::with_items(NX_SUBMENU_ROWS, NX_SUBMENU_COLUMNS, mash_tab_submenus),
    };
    overall_menu.tabs.push(mash_tab);

    // Mash Override Tab
    let mut override_tab_submenus: Vec<SubMenu> = Vec::new();
    override_tab_submenus.push(Action::to_submenu(
        "Ledge Neutral Getup",
        "ledge_neutral_override",
        "Mash Actions to be performed after a Neutral Getup from ledge",
        ToggleMultiple,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "Ledge Roll",
        "ledge_roll_override",
        "Mash Actions to be performed after a Roll Getup from ledge",
        ToggleMultiple,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "Ledge Jump",
        "ledge_jump_override",
        "Mash Actions to be performed after a Jump Get up from ledge",
        ToggleMultiple,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "Ledge Attack",
        "ledge_attack_override",
        "Mash Actions to be performed after a Getup Attack from ledge",
        ToggleMultiple,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "Tech Action",
        "tech_action_override",
        "Mash Actions to be performed after any tech action",
        ToggleMultiple,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "Clatter",
        "clatter_override",
        "Mash Actions to be performed after leaving a clatter situation (grab, bury, etc)",
        ToggleMultiple,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "Tumble",
        "tumble_override",
        "Mash Actions to be performed after exiting a tumble state",
        ToggleMultiple,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "Hitstun",
        "hitstun_override",
        "Mash Actions to be performed after exiting a hitstun state",
        ToggleMultiple,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "Parry",
        "parry_override",
        "Mash Actions to be performed after a parry",
        ToggleMultiple,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "Shieldstun",
        "shieldstun_override",
        "Mash Actions to be performed after exiting a shieldstun state",
        ToggleMultiple,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "Footstool",
        "footstool_override",
        "Mash Actions to be performed after exiting a footstool state",
        ToggleMultiple,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "Landing",
        "landing_override",
        "Mash Actions to be performed after landing on the ground",
        ToggleMultiple,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "Ledge Trump",
        "trump_override",
        "Mash Actions to be performed after leaving a ledgetrump state",
        ToggleMultiple,
    ));
    let override_tab = Tab {
        id: "override",
        title: "Override Settings",
        submenus: StatefulTable::with_items(
            NX_SUBMENU_ROWS,
            NX_SUBMENU_COLUMNS,
            override_tab_submenus,
        ),
    };
    overall_menu.tabs.push(override_tab);

    // Defensive Tab
    let mut defensive_tab_submenus: Vec<SubMenu> = Vec::new();
    defensive_tab_submenus.push(Direction::to_submenu(
        "Airdodge Direction",
        "air_dodge_dir",
        "Direction to angle airdodges",
        ToggleMultiple,
    ));
    defensive_tab_submenus.push(Direction::to_submenu(
        "DI Direction",
        "di_state",
        "Direction to angle the directional influence during hitlag",
        ToggleMultiple,
    ));
    defensive_tab_submenus.push(Direction::to_submenu(
        "SDI Direction",
        "sdi_state",
        "Direction to angle the smash directional influence during hitlag",
        ToggleMultiple,
    ));
    defensive_tab_submenus.push(SdiFrequency::to_submenu(
        "SDI Strength",
        "sdi_strength",
        "Relative strength of the smash directional influence inputs",
        ToggleMultiple,
    ));
    defensive_tab_submenus.push(ClatterFrequency::to_submenu(
        "Clatter Strength",
        "clatter_strength",
        "Configure how rapidly the CPU will mash out of grabs, buries, etc.",
        ToggleMultiple,
    ));
    defensive_tab_submenus.push(LedgeOption::to_submenu(
        "Ledge Options",
        "ledge_state",
        "Actions to be taken when on the ledge",
        ToggleMultiple,
    ));
    defensive_tab_submenus.push(LongDelay::to_submenu(
        "Ledge Delay",
        "ledge_delay",
        "How many frames to delay the ledge option",
        ToggleMultiple,
    ));
    defensive_tab_submenus.push(TechFlags::to_submenu(
        "Tech Options",
        "tech_state",
        "Actions to take when slammed into a hard surface",
        ToggleMultiple,
    ));
    defensive_tab_submenus.push(MissTechFlags::to_submenu(
        "Mistech Options",
        "miss_tech_state",
        "Actions to take after missing a tech",
        ToggleMultiple,
    ));
    defensive_tab_submenus.push(Shield::to_submenu(
        "Shield Toggles",
        "shield_state",
        "CPU Shield Behavior",
        ToggleSingle,
    ));
    defensive_tab_submenus.push(Direction::to_submenu(
        "Shield Tilt",
        "shield_tilt",
        "Direction to tilt the shield",
        ToggleSingle,
    ));
    defensive_tab_submenus.push(OnOff::to_submenu(
        "Crouch",
        "crouch",
        "Have the CPU crouch when on the ground",
        ToggleSingle,
    ));
    defensive_tab_submenus.push(OnOff::to_submenu("Dodge Staling", "stale_dodges", "Controls whether the CPU's dodges will worsen with repetitive use\n(Note: This can setting can cause combo behavior not possible in the original game)", ToggleSingle));
    defensive_tab_submenus.push(OnOff::to_submenu("Hide Tech Animations", "tech_hide", "Hides tech animations and effects after 7 frames to help with reacting to tech animation startup", ToggleSingle));
    let defensive_tab = Tab {
        id: "defensive",
        title: "Defensive Settings",
        submenus: StatefulTable::with_items(
            NX_SUBMENU_ROWS,
            NX_SUBMENU_COLUMNS,
            defensive_tab_submenus,
        ),
    };
    overall_menu.tabs.push(defensive_tab);

    // Input Recording Tab
    let mut input_recording_tab_submenus: Vec<SubMenu> = Vec::new();
    input_recording_tab_submenus.push(RecordSlot::to_submenu(
        "Recording Slot",
        "recording_slot",
        "Choose which slot to record into",
        ToggleSingle,
    ));
    input_recording_tab_submenus.push(RecordTrigger::to_submenu(
        "Recording Trigger",
        "record_trigger",
        "Whether to begin recording via button combination or upon loading a Save State",
        ToggleSingle,
    ));
    input_recording_tab_submenus.push(RecordingDuration::to_submenu(
        "Recording Duration",
        "recording_duration",
        "How long an input recording should last in frames",
        ToggleSingle,
    ));
    input_recording_tab_submenus.push(OnOff::to_submenu(
        "Recording Crop",
        "recording_crop",
        "Remove neutral input frames at the end of your recording",
        ToggleSingle,
    ));
    input_recording_tab_submenus.push(PlaybackSlot::to_submenu(
        "Playback Button Slots",
        "playback_button_slots",
        "Choose which slots to playback input recording upon pressing button combination",
        ToggleMultiple,
    ));
    input_recording_tab_submenus.push(HitstunPlayback::to_submenu(
        "Playback Hitstun Timing",
        "hitstun_playback",
        "When to begin playing back inputs when a hitstun mash trigger occurs",
        ToggleSingle,
    ));
    input_recording_tab_submenus.push(PlaybackSlot::to_submenu(
        "Save State Playback",
        "save_state_playback",
        "Choose which slots to playback input recording upon loading a save state",
        ToggleMultiple,
    ));
    input_recording_tab_submenus.push(OnOff::to_submenu(
        "Playback Mash Interrupt",
        "playback_mash",
        "End input playback when a mash trigger occurs",
        ToggleSingle,
    ));
    input_recording_tab_submenus.push(OnOff::to_submenu(
        "Playback Loop",
        "playback_loop",
        "Repeat triggered input playbacks indefinitely",
        ToggleSingle,
    ));
    let input_tab = Tab {
        id: "input",
        title: "Input Recording",
        submenus: StatefulTable::with_items(
            NX_SUBMENU_ROWS,
            NX_SUBMENU_COLUMNS,
            input_recording_tab_submenus,
        ),
    };
    overall_menu.tabs.push(input_tab);

    // Button Tab
    let mut button_tab_submenus: Vec<SubMenu> = Vec::new();
    button_tab_submenus.push(OnOff::to_submenu("Menu Open Start Press", "menu_open_start_press", "Hold start or press minus to open the mod menu. To open the original menu, press start.\nThe default menu open option is always available as Hold DPad Up + Press B.", ToggleSingle));
    button_tab_submenus.push(ButtonConfig::to_submenu(
        "Save State Save",
        "save_state_save",
        "Hold any one button and press the others to trigger",
        ToggleSingle,
    ));
    button_tab_submenus.push(ButtonConfig::to_submenu(
        "Save State Load",
        "save_state_load",
        "Hold any one button and press the others to trigger",
        ToggleSingle,
    ));
    button_tab_submenus.push(ButtonConfig::to_submenu(
        "Input Record",
        "input_record",
        "Hold any one button and press the others to trigger",
        ToggleSingle,
    ));
    button_tab_submenus.push(ButtonConfig::to_submenu(
        "Input Playback",
        "input_playback",
        "Hold any one button and press the others to trigger",
        ToggleSingle,
    ));
    let button_tab = Tab {
        id: "button",
        title: "Button Config",
        submenus: StatefulTable::with_items(
            NX_SUBMENU_ROWS,
            NX_SUBMENU_COLUMNS,
            button_tab_submenus,
        ),
    };
    overall_menu.tabs.push(button_tab);

    // Save State Tab
    let mut save_state_tab_submenus: Vec<SubMenu> = Vec::new();
    save_state_tab_submenus.push(SaveStateMirroring::to_submenu(
        "Mirroring",
        "save_state_mirroring",
        "Flips save states in the left-right direction across the stage center",
        ToggleSingle,
    ));
    save_state_tab_submenus.push(OnOff::to_submenu(
        "Auto Save States",
        "save_state_autoload",
        "Load save state when any fighter dies",
        ToggleSingle,
    ));
    save_state_tab_submenus.push(SaveDamage::to_submenu(
        "Save Dmg (CPU)",
        "save_damage_cpu",
        "Should save states retain CPU damage",
        ToggleSingle,
    ));
    save_state_tab_submenus.push(DamagePercent::to_submenu(
        "Dmg Range (CPU)",
        "save_damage_limits_cpu",
        "Limits on Random Damage to apply to the CPU when loading a save state",
        Slider,
    ));
    save_state_tab_submenus.push(SaveDamage::to_submenu(
        "Save Dmg (Player)",
        "save_damage_player",
        "Should save states retain player damage",
        ToggleSingle,
    ));
    save_state_tab_submenus.push(DamagePercent::to_submenu(
        "Dmg Range (Player)",
        "save_damage_limits_player",
        "Limits on random damage to apply to the player when loading a save state",
        Slider,
    ));
    save_state_tab_submenus.push(OnOff::to_submenu(
        "Enable Save States",
        "save_state_enable",
        "Enable save states! Save a state with Shield+Down Taunt, load it with Shield+Up Taunt.",
        ToggleSingle,
    ));
    save_state_tab_submenus.push(SaveStateSlot::to_submenu(
        "Save State Slot",
        "save_state_slot",
        "Save and load states from different slots.",
        ToggleSingle,
    ));
    save_state_tab_submenus.push(SaveStateSlot::to_submenu(
        "Randomize Slots",
        "randomize_slots",
        "Slots to randomize when loading save state.",
        ToggleMultiple,
    ));
    save_state_tab_submenus.push(CharacterItem::to_submenu(
        "Character Item",
        "character_item",
        "The item to give to the player's fighter when loading a save state",
        ToggleSingle,
    ));
    save_state_tab_submenus.push(BuffOption::to_submenu(
        "Buff Options",
        "buff_state",
        "Buff(s) to be applied to the respective fighters when loading a save state",
        ToggleSingle,
    ));
    let save_state_tab = Tab {
        id: "save_state",
        title: "Save States",
        submenus: StatefulTable::with_items(
            NX_SUBMENU_ROWS,
            NX_SUBMENU_COLUMNS,
            save_state_tab_submenus,
        ),
    };
    overall_menu.tabs.push(save_state_tab);

    // Miscellaneous Tab
    let mut misc_tab_submenus: Vec<SubMenu> = Vec::new();
    misc_tab_submenus.push(OnOff::to_submenu("Frame Advantage", "frame_advantage", "Display the time difference between when the player is actionable and the CPU is actionable\nNote that the CPU must not be mashing any options.", ToggleSingle));
    misc_tab_submenus.push(OnOff::to_submenu(
        "Hitbox Visualization",
        "hitbox_vis",
        "Display a visual representation for active hitboxes (hides other visual effects)",
        ToggleSingle,
    ));
    misc_tab_submenus.push(InputDisplay::to_submenu(
        "Input Display",
        "input_display",
        "Log inputs in a queue on the left of the screen",
        ToggleSingle,
    ));
    misc_tab_submenus.push(OnOff::to_submenu(
        "Input Display Status",
        "input_display_status",
        "Group input logs by status in which they occurred",
        ToggleSingle,
    ));
    misc_tab_submenus.push(Delay::to_submenu(
        "Input Delay",
        "input_delay",
        "Frames to delay player inputs by",
        ToggleSingle,
    ));
    misc_tab_submenus.push(OnOff::to_submenu(
        "Stage Hazards",
        "stage_hazards",
        "Turn stage hazards on/off",
        ToggleSingle,
    ));
    misc_tab_submenus.push(OnOff::to_submenu(
        "HUD",
        "hud",
        "Show/hide elements of the UI",
        ToggleSingle,
    ));
    misc_tab_submenus.push(UpdatePolicy::to_submenu(
        "Auto-Update",
        "update_policy",
        "What type of Training Modpack updates to automatically apply. (Console Only!)",
        ToggleSingle,
    ));
    misc_tab_submenus.push(OnOff::to_submenu(
        "L+R+A Reset",
        "lra_reset",
        "Reset Training Room when pressing L+R+A",
        ToggleSingle,
    ));
    misc_tab_submenus.push(Locale::to_submenu(
        "menus.misc_settings.language.title",
        "selected_locale",
        "menus.misc_settings.language.help_text",
        ToggleSingle,
    ));
    let misc_tab = Tab {
        id: "misc",
        title: "menus.misc_settings.tab_name",
        submenus: StatefulTable::with_items(NX_SUBMENU_ROWS, NX_SUBMENU_COLUMNS, misc_tab_submenus),
    };
    overall_menu.tabs.push(misc_tab);

    // Ensure that a tab is always selected
    if overall_menu.tabs.get_selected().is_none() {
        overall_menu.tabs.state.select(Some(0));
    }

    overall_menu
}
