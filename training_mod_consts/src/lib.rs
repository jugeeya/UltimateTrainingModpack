extern crate byteflags;
extern crate num_derive;

use serde::{Deserialize, Serialize};

pub mod options;
pub use options::*;
pub mod files;
pub use files::*;
pub mod config;
pub use config::*;

use training_mod_tui_2::*;
use paste::paste;
use std::iter::zip;

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

pub static DEFAULTS_MENU: TrainingModpackMenu = TrainingModpackMenu {
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
    save_state_save: ButtonConfig { ZL: 1, DPAD_DOWN: 1, ..ButtonConfig::empty()},
    save_state_load: ButtonConfig { ZL: 1, DPAD_UP: 1, ..ButtonConfig::empty()},
    input_record: ButtonConfig { ZR: 1, DPAD_DOWN: 1, ..ButtonConfig::empty()},
    input_playback: ButtonConfig { ZR: 1, DPAD_UP: 1, ..ButtonConfig::empty()},
    recording_crop: OnOff::ON,
    stale_dodges: OnOff::ON,
    tech_hide: OnOff::OFF,
    update_policy: UpdatePolicy::default(),
};

pub static mut MENU: TrainingModpackMenu = DEFAULTS_MENU;

impl_toggletrait! {
    OnOff,
    "Menu Open Start Press",
    "menu_open_start_press",
    "Menu Open Start Press: Hold start or press minus to open the mod menu. To open the original menu, press start.\nThe default menu open option is always available as Hold DPad Up + Press B.",
    true,
}
impl_toggletrait! {
    ButtonConfig,
    "Save State Save",
    "save_state_save",
    "Save State Save: Hold any one button and press the others to trigger",
    false,
}
impl_toggletrait! {
    ButtonConfig,
    "Save State Load",
    "save_state_load",
    "Save State Load: Hold any one button and press the others to trigger",
    false,
}
impl_toggletrait! {
    ButtonConfig,
    "Input Record",
    "input_record",
    "Input Record: Hold any one button and press the others to trigger",
    false,
}
impl_toggletrait! {
    ButtonConfig,
    "Input Playback",
    "input_playback",
    "Input Playback: Hold any one button and press the others to trigger",
    false,
}
impl_toggletrait! {
    Action,
    "Mash Toggles",
    "mash_state",
    "Mash Toggles: Actions to be performed as soon as possible",
    false,
}
impl_toggletrait! {
    Action,
    "Followup Toggles",
    "follow_up",
    "Followup Toggles: Actions to be performed after a Mash option",
    false,
}
impl_toggletrait! {
    MashTrigger,
    "Mash Triggers",
    "mash_triggers",
    "Mash triggers: Configure what causes the CPU to perform a Mash option",
    false,
}
impl_toggletrait! {
    AttackAngle,
    "Attack Angle",
    "attack_angle",
    "Attack Angle: For attacks that can be angled, such as some forward tilts",
    false,
}
impl_toggletrait! {
    ThrowOption,
    "Throw Options",
    "throw_state",
    "Throw Options: Throw to be performed when a grab is landed",
    false,
}
impl_toggletrait! {
    MedDelay,
    "Throw Delay",
    "throw_delay",
    "Throw Delay: How many frames to delay the throw option",
    false,
}
impl_toggletrait! {
    MedDelay,
    "Pummel Delay",
    "pummel_delay",
    "Pummel Delay: How many frames after a grab to wait before starting to pummel",
    false,
}
impl_toggletrait! {
    BoolFlag,
    "Falling Aerials",
    "falling_aerials",
    "Falling Aerials: Should aerials be performed when rising or when falling",
    false,
}
impl_toggletrait! {
    BoolFlag,
    "Full Hop",
    "full_hop",
    "Full Hop: Should the CPU perform a full hop or a short hop",
    false,
}
impl_toggletrait! {
    Delay,
    "Aerial Delay",
    "aerial_delay",
    "Aerial Delay: How long to delay a Mash aerial attack",
    false,
}
impl_toggletrait! {
    BoolFlag,
    "Fast Fall",
    "fast_fall",
    "Fast Fall: Should the CPU fastfall during a jump",
    false,
}
impl_toggletrait! {
    Delay,
    "Fast Fall Delay",
    "fast_fall_delay",
    "Fast Fall Delay: How many frames the CPU should delay their fastfall",
    false,
}
impl_toggletrait! {
    Delay,
    "OoS Offset",
    "oos_offset",
    "OoS Offset: How many times the CPU shield can be hit before performing a Mash option",
    false,
}
impl_toggletrait! {
    Delay,
    "Reaction Time",
    "reaction_time",
    "Reaction Time: How many frames to delay before performing a mash option",
    false,
}
impl_toggletrait! {
    Action,
    "Ledge Neutral Getup",
    "ledge_neutral_override",
    "Neutral Getup Override: Mash Actions to be performed after a Neutral Getup from ledge",
    false,
}
impl_toggletrait! {
    Action,
    "Ledge Roll",
    "ledge_roll_override",
    "Ledge Roll Override: Mash Actions to be performed after a Roll Getup from ledge",
    false,
}
impl_toggletrait! {
    Action,
    "Ledge Jump",
    "ledge_jump_override",
    "Ledge Jump Override: Mash Actions to be performed after a Jump Getup from ledge",
    false,
}
impl_toggletrait! {
    Action,
    "Ledge Attack",
    "ledge_attack_override",
    "Ledge Attack Override: Mash Actions to be performed after a Getup Attack from ledge",
    false,
}
impl_toggletrait! {
    Action,
    "Tech Action",
    "tech_action_override",
    "Tech Action Override: Mash Actions to be performed after any tech action",
    false,
}
impl_toggletrait! {
    Action,
    "Clatter",
    "clatter_override",
    "Clatter Override: Mash Actions to be performed after leaving a clatter situation (grab, bury, etc)",
    false,
}
impl_toggletrait! {
    Action,
    "Tumble",
    "tumble_override",
    "Tumble Override: Mash Actions to be performed after exiting a tumble state",
    false,
}
impl_toggletrait! {
    Action,
    "Hitstun",
    "hitstun_override",
    "Hitstun Override: Mash Actions to be performed after exiting a hitstun state",
    false,
}
impl_toggletrait! {
    Action,
    "Parry",
    "parry_override",
    "Parry Override: Mash Actions to be performed after a parry",
    false,
}
impl_toggletrait! {
    Action,
    "Shieldstun",
    "shieldstun_override",
    "Shieldstun Override: Mash Actions to be performed after exiting a shieldstun state",
    false,
}
impl_toggletrait! {
    Action,
    "Footstool",
    "footstool_override",
    "Footstool Override: Mash Actions to be performed after exiting a footstool state",
    false,
}
impl_toggletrait! {
    Action,
    "Landing",
    "landing_override",
    "Landing Override: Mash Actions to be performed after landing on the ground",
    false,
}
impl_toggletrait! {
    Action,
    "Ledge Trump",
    "trump_override",
    "Ledge Trump Override: Mash Actions to be performed after leaving a ledgetrump state",
    false,
}
impl_toggletrait! {
    Direction,
    "Airdodge Direction",
    "air_dodge_dir",
    "Airdodge Direction: Direction to angle airdodges",
    false,
}
impl_toggletrait! {
    Direction,
    "DI Direction",
    "di_state",
    "DI Direction: Direction to angle the directional influence during hitlag",
    false,
}
impl_toggletrait! {
    Direction,
    "SDI Direction",
    "sdi_state",
    "SDI Direction: Direction to angle the smash directional influence during hitlag",
    false,
}
impl_toggletrait! {
    SdiFrequency,
    "SDI Strength",
    "sdi_strength",
    "SDI Strength: Relative strength of the smash directional influence inputs",
    true,
}
impl_toggletrait! {
    ClatterFrequency,
    "Clatter Strength",
    "clatter_strength",
    "Clatter Strength: Configure how rapidly the CPU will mash out of grabs, buries, etc.",
    true,
}
impl_toggletrait! {
    LedgeOption,
    "Ledge Options",
    "ledge_state",
    "Ledge Options: Actions to be taken when on the ledge",
    false,
}
impl_toggletrait! {
    LongDelay,
    "Ledge Delay",
    "ledge_delay",
    "Ledge Delay: How many frames to delay the ledge option",
    false,
}
impl_toggletrait! {
    TechFlags,
    "Tech Options",
    "tech_state",
    "Tech Options: Actions to take when slammed into a hard surface",
    false,
}
impl_toggletrait! {
    MissTechFlags,
    "Mistech Options",
    "miss_tech_state",
    "Mistech Options: Actions to take after missing a tech",
    false,
}
impl_toggletrait! {
    Shield,
    "Shield Toggles",
    "shield_state",
    "Shield Toggles: CPU Shield Behavior",
    true,
}
impl_toggletrait! {
    Direction,
    "Shield Tilt",
    "shield_tilt",
    "Shield Tilt: Direction to tilt the shield",
    true,
}

impl_toggletrait! {
    OnOff,
    "Crouch",
    "crouch",
    "Crouch: Have the CPU crouch when on the ground",
    true,
}
impl_toggletrait! {
    OnOff,
    "Dodge Staling",
    "stale_dodges",
    "Dodge Staling: Controls whether the CPU's dodges will worsen with repetitive use\n(Note: This can setting can cause combo behavior not possible in the original game)",
    true,
}
impl_toggletrait! {
    OnOff,
    "Hide Tech Animations",
    "tech_hide",
    "Hide Tech Animations: Hides tech animations and effects after 7 frames to help with reacting to tech animation startup",
    true,
}
impl_toggletrait! {
    SaveStateMirroring,
    "Mirroring",
    "save_state_mirroring",
    "Mirroring: Flips save states in the left-right direction across the stage center",
    true,
}
impl_toggletrait! {
    OnOff,
    "Auto Save States",
    "save_state_autoload",
    "Auto Save States: Load save state when any fighter dies",
    true,
}
impl_toggletrait! {
    SaveDamage,
    "Save Dmg (CPU)",
    "save_damage_cpu",
    "Save Damage: Should save states retain CPU damage",
    true,
}
impl_slidertrait! {
    DamagePercent,
    "Dmg Range (CPU)",
    "save_damage_limits_cpu",
    "Limits on random damage to apply to the CPU when loading a save state",
}
impl_toggletrait! {
    SaveDamage,
    "Save Dmg (Player)",
    "save_damage_player",
    "Save Damage: Should save states retain player damage",
    true,
}
impl_slidertrait! {
    DamagePercent,
    "Dmg Range (Player)",
    "save_damage_limits_player",
    "Limits on random damage to apply to the player when loading a save state",
}
impl_toggletrait! {
    OnOff,
    "Enable Save States",
    "save_state_enable",
    "Save States: Enable save states! Save a state with Shield+Down Taunt, load it with Shield+Up Taunt.",
    true,
}
impl_toggletrait! {
    SaveStateSlot,
    "Save State Slot",
    "save_state_slot",
    "Save State Slot: Save and load states from different slots.",
    true,
}
impl_toggletrait! {
    SaveStateSlot,
    "Randomize Slots",
    "randomize_slots",
    "Randomize Slots: Slots to randomize when loading save state.",
    false,
}
impl_toggletrait! {
    CharacterItem,
    "Character Item",
    "character_item",
    "Character Item: The item to give to the player's fighter when loading a save state",
    true,
}
impl_toggletrait! {
    BuffOption,
    "Buff Options",
    "buff_state",
    "Buff Options: Buff(s) to be applied to the respective fighters when loading a save state",
    false,
}
impl_toggletrait! {
    PlaybackSlot,
    "Save State Playback",
    "save_state_playback",
    "Save State Playback: Choose which slots to playback input recording upon loading a save state",
    false,
}
impl_toggletrait! {
    OnOff,
    "Frame Advantage",
    "frame_advantage",
    "Frame Advantage: Display the time difference between when the player is actionable and the CPU is actionable",
    true,
}
impl_toggletrait! {
    OnOff,
    "Hitbox Visualization",
    "hitbox_vis",
    "Hitbox Visualization: Display a visual representation for active hitboxes (hides other visual effects)",
    true,
}
impl_toggletrait! {
    InputDisplay,
    "Input Display",
    "input_display",
    "Input Display: Log inputs in a queue on the left of the screen",
    true,
}
impl_toggletrait! {
    OnOff,
    "Input Display Status",
    "input_display_status",
    "Input Display Status: Group input logs by status in which they occurred",
    true,
}
impl_toggletrait! {
    Delay,
    "Input Delay",
    "input_delay",
    "Input Delay: Frames to delay player inputs by",
    true,
}
impl_toggletrait! {
    OnOff,
    "Stage Hazards",
    "stage_hazards",
    "Stage Hazards: Turn stage hazards on/off",
    true,
}
impl_toggletrait! {
    OnOff,
    "HUD",
    "hud",
    "HUD: Show/hide elements of the UI",
    true,
}
impl_toggletrait! {
    UpdatePolicy,
    "Auto-Update",
    "update_policy",
    "Auto-Update: What type of Training Modpack updates to automatically apply. (Console Only!)",
    true,
}
impl_toggletrait! {
    RecordSlot,
    "Recording Slot",
    "recording_slot",
    "Recording Slot: Choose which slot to record into",
    true,
}
impl_toggletrait! {
    RecordTrigger,
    "Recording Trigger",
    "record_trigger",
    "Recording Trigger: Whether to begin recording via button combination or upon loading a Save State",
    false,
}
impl_toggletrait! {
    RecordingDuration,
    "Recording Duration",
    "recording_duration",
    "Recording Duration: How long an input recording should last in frames",
    true,
}
impl_toggletrait! {
    OnOff,
    "Recording Crop",
    "recording_crop",
    "Recording Crop: Remove neutral input frames at the end of your recording",
    true,
}
impl_toggletrait! {
    PlaybackSlot,
    "Playback Button Slots",
    "playback_button_slots",
    "Playback Button Slots: Choose which slots to playback input recording upon pressing button combination",
    false,
}
impl_toggletrait! {
    HitstunPlayback,
    "Playback Hitstun Timing",
    "hitstun_playback",
    "Playback Hitstun Timing: When to begin playing back inputs when a hitstun mash trigger occurs",
    true,
}
impl_toggletrait! {
    OnOff,
    "Playback Mash Interrupt",
    "playback_mash",
    "Playback Mash Interrupt: End input playback when a mash trigger occurs",
    true,
}
impl_toggletrait! {
    OnOff,
    "Playback Loop",
    "playback_loop",
    "Playback Loop: Repeat triggered input playbacks indefinitely",
    true,
}

pub unsafe fn ui_menu<'a>(menu: TrainingModpackMenu) -> App<'a> {
    
    let mut overall_menu = App::new();

    // Mash Tab
    let mut mash_tab_submenus: Vec<SubMenu> = Vec::new();
    mash_tab_submenus.push(menu.mash_state.to_submenu_mash_state());
    mash_tab_submenus.push(menu.follow_up.to_submenu_follow_up());
    mash_tab_submenus.push(menu.mash_triggers.to_submenu_mash_triggers());
    mash_tab_submenus.push(menu.attack_angle.to_submenu_attack_angle());
    mash_tab_submenus.push(menu.throw_state.to_submenu_throw_state());
    mash_tab_submenus.push(menu.throw_delay.to_submenu_throw_delay());
    mash_tab_submenus.push(menu.pummel_delay.to_submenu_pummel_delay());
    mash_tab_submenus.push(menu.falling_aerials.to_submenu_falling_aerials());
    mash_tab_submenus.push(menu.full_hop.to_submenu_full_hop());
    mash_tab_submenus.push(menu.aerial_delay.to_submenu_aerial_delay());
    mash_tab_submenus.push(menu.fast_fall.to_submenu_fast_fall());
    mash_tab_submenus.push(menu.fast_fall_delay.to_submenu_fast_fall_delay());
    mash_tab_submenus.push(menu.oos_offset.to_submenu_oos_offset());
    mash_tab_submenus.push(menu.reaction_time.to_submenu_reaction_time());
    let mash_tab = Tab {
        id: "mash",
        title: "Mash Settings",
        submenus: StatefulTable::with_items(NX_SUBMENU_ROWS, NX_SUBMENU_COLUMNS, mash_tab_submenus),
    };
    overall_menu.tabs.push(mash_tab);

    // Mash Override Tab
    let mut override_tab_submenus: Vec<SubMenu> = Vec::new();
    override_tab_submenus.push(menu.ledge_neutral_override.to_submenu_ledge_neutral_override());
    override_tab_submenus.push(menu.ledge_roll_override.to_submenu_ledge_roll_override());
    override_tab_submenus.push(menu.ledge_jump_override.to_submenu_ledge_jump_override());
    override_tab_submenus.push(menu.ledge_attack_override.to_submenu_ledge_attack_override());
    override_tab_submenus.push(menu.tech_action_override.to_submenu_tech_action_override());
    override_tab_submenus.push(menu.clatter_override.to_submenu_clatter_override());
    override_tab_submenus.push(menu.tumble_override.to_submenu_tumble_override());
    override_tab_submenus.push(menu.hitstun_override.to_submenu_hitstun_override());
    override_tab_submenus.push(menu.parry_override.to_submenu_parry_override());
    override_tab_submenus.push(menu.shieldstun_override.to_submenu_shieldstun_override());
    override_tab_submenus.push(menu.footstool_override.to_submenu_footstool_override());
    override_tab_submenus.push(menu.landing_override.to_submenu_landing_override());
    override_tab_submenus.push(menu.trump_override.to_submenu_trump_override());
    let override_tab = Tab {
        id: "override",
        title: "Override Settings",
        submenus: StatefulTable::with_items(NX_SUBMENU_ROWS, NX_SUBMENU_COLUMNS, override_tab_submenus),
    };
    overall_menu.tabs.push(override_tab);

    // Defensive Tab
    let mut defensive_tab_submenus: Vec<SubMenu> = Vec::new();
    defensive_tab_submenus.push(menu.air_dodge_dir.to_submenu_air_dodge_dir());
    defensive_tab_submenus.push(menu.di_state.to_submenu_di_state());
    defensive_tab_submenus.push(menu.sdi_state.to_submenu_sdi_state());
    defensive_tab_submenus.push(menu.sdi_strength.to_submenu_sdi_strength());
    defensive_tab_submenus.push(menu.clatter_strength.to_submenu_clatter_strength());
    defensive_tab_submenus.push(menu.ledge_state.to_submenu_ledge_state());
    defensive_tab_submenus.push(menu.ledge_delay.to_submenu_ledge_delay());
    defensive_tab_submenus.push(menu.tech_state.to_submenu_tech_state());
    defensive_tab_submenus.push(menu.miss_tech_state.to_submenu_miss_tech_state());
    defensive_tab_submenus.push(menu.shield_state.to_submenu_shield_state());
    defensive_tab_submenus.push(menu.shield_tilt.to_submenu_shield_tilt());
    defensive_tab_submenus.push(menu.crouch.to_submenu_crouch());
    defensive_tab_submenus.push(menu.stale_dodges.to_submenu_stale_dodges());
    defensive_tab_submenus.push(menu.tech_hide.to_submenu_tech_hide());
    let defensive_tab = Tab {
        id: "defensive",
        title: "Defensive Settings",
        submenus: StatefulTable::with_items(NX_SUBMENU_ROWS, NX_SUBMENU_COLUMNS, defensive_tab_submenus),
    };
    overall_menu.tabs.push(defensive_tab);

    // Input Recording Tab
    let mut input_recording_tab_submenus: Vec<SubMenu> = Vec::new();
    input_recording_tab_submenus.push(menu.recording_slot.to_submenu_recording_slot());
    input_recording_tab_submenus.push(menu.record_trigger.to_submenu_record_trigger());
    input_recording_tab_submenus.push(menu.recording_duration.to_submenu_recording_duration());
    input_recording_tab_submenus.push(menu.recording_crop.to_submenu_recording_crop());
    input_recording_tab_submenus.push(menu.playback_button_slots.to_submenu_playback_button_slots());
    input_recording_tab_submenus.push(menu.hitstun_playback.to_submenu_hitstun_playback());
    input_recording_tab_submenus.push(menu.playback_mash.to_submenu_playback_mash());
    input_recording_tab_submenus.push(menu.playback_loop.to_submenu_playback_loop());
    let input_tab = Tab {
        id: "input",
        title: "Input Recording",
        submenus: StatefulTable::with_items(NX_SUBMENU_ROWS, NX_SUBMENU_COLUMNS, input_recording_tab_submenus),
    };
    overall_menu.tabs.push(input_tab);

    // Button Tab
    let mut button_tab_submenus: Vec<SubMenu> = Vec::new();
    button_tab_submenus.push(menu.menu_open_start_press.to_submenu_menu_open_start_press());
    button_tab_submenus.push(menu.save_state_save.to_submenu_save_state_save());
    button_tab_submenus.push(menu.save_state_load.to_submenu_save_state_load());
    button_tab_submenus.push(menu.input_record.to_submenu_input_record());
    button_tab_submenus.push(menu.input_playback.to_submenu_input_playback());
    let button_tab = Tab {
        id: "button",
        title: "Button Config",
        submenus: StatefulTable::with_items(NX_SUBMENU_ROWS, NX_SUBMENU_COLUMNS, button_tab_submenus),
    };
    overall_menu.tabs.push(button_tab);

    // Save State Tab
    let mut save_state_tab_submenus: Vec<SubMenu> = Vec::new();
    save_state_tab_submenus.push(menu.save_state_mirroring.to_submenu_save_state_mirroring());
    save_state_tab_submenus.push(menu.save_state_autoload.to_submenu_save_state_autoload());
    save_state_tab_submenus.push(menu.save_damage_cpu.to_submenu_save_damage_cpu());
    save_state_tab_submenus.push(menu.save_damage_limits_cpu.to_submenu_save_damage_limits_cpu());
    save_state_tab_submenus.push(menu.save_damage_player.to_submenu_save_damage_player());
    save_state_tab_submenus.push(menu.save_damage_limits_player.to_submenu_save_damage_limits_player());
    save_state_tab_submenus.push(menu.save_state_enable.to_submenu_save_state_enable());
    save_state_tab_submenus.push(menu.save_state_slot.to_submenu_save_state_slot());
    save_state_tab_submenus.push(menu.randomize_slots.to_submenu_randomize_slots());
    save_state_tab_submenus.push(menu.character_item.to_submenu_character_item());
    save_state_tab_submenus.push(menu.buff_state.to_submenu_buff_state());
    save_state_tab_submenus.push(menu.save_state_playback.to_submenu_save_state_playback());
    let save_state_tab = Tab {
        id: "save_state",
        title: "Save States",
        submenus: StatefulTable::with_items(NX_SUBMENU_ROWS, NX_SUBMENU_COLUMNS, save_state_tab_submenus),
    };
    overall_menu.tabs.push(save_state_tab);

    // Miscellaneous Tab
    let mut misc_tab_submenus: Vec<SubMenu> = Vec::new();
    misc_tab_submenus.push(menu.frame_advantage.to_submenu_frame_advantage());
    misc_tab_submenus.push(menu.hitbox_vis.to_submenu_hitbox_vis());
    misc_tab_submenus.push(menu.input_display.to_submenu_input_display());
    misc_tab_submenus.push(menu.input_display_status.to_submenu_input_display_status());
    misc_tab_submenus.push(menu.input_delay.to_submenu_input_delay());
    misc_tab_submenus.push(menu.stage_hazards.to_submenu_stage_hazards());
    misc_tab_submenus.push(menu.hud.to_submenu_hud());
    misc_tab_submenus.push(menu.update_policy.to_submenu_update_policy());
    let misc_tab = Tab {
        id: "misc",
        title: "Misc Settings",
        submenus: StatefulTable::with_items(NX_SUBMENU_ROWS, NX_SUBMENU_COLUMNS, misc_tab_submenus),
    };
    overall_menu.tabs.push(misc_tab);

    // Ensure that a tab is always selected
    if overall_menu.tabs.get_selected().is_none() {
        overall_menu.tabs.state.select(Some(0));
    }

    overall_menu
}
