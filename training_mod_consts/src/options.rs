use core::f64::consts::PI;
use serde::{Deserialize, Serialize};
#[cfg(feature = "smash")]
use smash::lib::lua_const::*;
use byteflags::*;

#[macro_export]
macro_rules! impl_toggletrait {
    (
        $e:ty,
        $title:literal,
        $id:literal,
        $help_text:literal,
        $single:literal,
    ) => {
        impl $e {
            paste! {
                fn [<to_submenu_ $id>](&self) -> SubMenu {
                    let submenu_type = if $single { SubMenuType::ToggleSingle } else { SubMenuType::ToggleMultiple };
                    let max: u8 = if $single { 1 } else { 8 };
                    let toggles_vec: Vec<Toggle> = zip(<$e>::ALL_NAMES, self.to_vec())
                        .map(|(title, value)| Toggle { title, value, max })
                        .collect();
                    SubMenu {
                        title: $title,
                        id: $id,
                        help_text: $help_text,
                        submenu_type: submenu_type,
                        toggles: StatefulTable::with_items(NX_SUBMENU_ROWS, NX_SUBMENU_COLUMNS, toggles_vec),
                        slider: None
                    }
                }
            }
            
        }
    }
}

#[macro_export]
macro_rules! impl_slidertrait {
    (
        $e:ty,
        $title:literal,
        $id:literal,
        $help_text:literal,
    ) => {
        impl $e {
            paste! {
                fn [<to_submenu_ $id>](&self) -> SubMenu {
                    let slider = StatefulSlider {
                        lower: self.0,
                        upper: self.1,
                        ..StatefulSlider::new()
                    };
                    SubMenu {
                        title: $title,
                        id: $id,
                        help_text: $help_text,
                        submenu_type: SubMenuType::Slider,
                        toggles: StatefulTable::with_items(NX_SUBMENU_ROWS, NX_SUBMENU_COLUMNS, Vec::new()),
                        slider: Some(slider)
                    }
                }
            }
            
        }
    }
}

pub fn get_random_int(_max: i32) -> i32 {
    #[cfg(feature = "smash")]
    unsafe {
        smash::app::sv_math::rand(smash::hash40("fighter"), _max)
    }

    #[cfg(not(feature = "smash"))]
    0
}

/// Generate a random float between _min and _max.
/// Note that (_min <= _max) is not enforced.
pub fn get_random_float(_min: f32, _max: f32) -> f32 {
    #[cfg(feature = "smash")]
    unsafe {
        _min + smash::app::sv_math::randf(smash::hash40("fighter"), _max - _min)
    }

    #[cfg(not(feature = "smash"))]
    _min
}

pub fn random_option<T>(arg: &[T]) -> &T {
    &arg[get_random_int(arg.len() as i32) as usize]
}

// DI
/*
 0, 0.785398, 1.570796, 2.356194, -3.14159, -2.356194,  -1.570796, -0.785398
 0, pi/4,     pi/2,     3pi/4,    pi,       5pi/4,      3pi/2,     7pi/4
*/

// DI / Left stick
byteflags! {
    pub struct Direction {
        OUT = "Out",
        UP_OUT = "Up Out",
        UP = "Up",
        UP_IN = "Up In",
        IN = "In",
        DOWN_IN = "Down In",
        DOWN = "Down",
        DOWN_OUT = "Down Out",
        NEUTRAL = "Neutral",
        LEFT = "Left",
        RIGHT = "Right",
    }
}

impl Direction {
    pub fn into_angle(self) -> Option<f64> {
        let index = self.into_index();

        if index == 0.0 {
            None
        } else {
            Some((index - 1.0) * PI / 4.0)
        }
    }
    fn into_index(self) -> f64 {
        match self {
            Direction::OUT => 1.0,
            Direction::UP_OUT => 2.0,
            Direction::UP => 3.0,
            Direction::UP_IN => 4.0,
            Direction::IN => 5.0,
            Direction::DOWN_IN => 6.0,
            Direction::DOWN => 7.0,
            Direction::DOWN_OUT => 8.0,
            Direction::NEUTRAL => 0.0,
            Direction::LEFT => 5.0,
            Direction::RIGHT => 1.0,
            _ => 0.0,
        }
    }
}

// Ledge Option
byteflags! {
    pub struct LedgeOption
    {
        NEUTRAL = "Neutral Getup",
        ROLL = "Roll",
        JUMP = "Jump",
        ATTACK = "Getup Attack",
        WAIT = "Wait",
        PLAYBACK_1 = "Playback Slot 1",
        PLAYBACK_2 = "Playback Slot 2",
        PLAYBACK_3 = "Playback Slot 3",
        PLAYBACK_4 = "Playback Slot 4",
        PLAYBACK_5 = "Playback Slot 5",
    }
}

impl LedgeOption {
    pub fn into_status(self) -> Option<i32> {
        #[cfg(feature = "smash")]
        {
            Some(match self {
                LedgeOption::NEUTRAL => *FIGHTER_STATUS_KIND_CLIFF_CLIMB,
                LedgeOption::ROLL => *FIGHTER_STATUS_KIND_CLIFF_ESCAPE,
                LedgeOption::JUMP => *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
                LedgeOption::ATTACK => *FIGHTER_STATUS_KIND_CLIFF_ATTACK,
                LedgeOption::WAIT => *FIGHTER_STATUS_KIND_CLIFF_WAIT,
                LedgeOption::PLAYBACK_1
                | LedgeOption::PLAYBACK_2
                | LedgeOption::PLAYBACK_3
                | LedgeOption::PLAYBACK_4
                | LedgeOption::PLAYBACK_5 => *FIGHTER_STATUS_KIND_NONE,
                _ => return None,
            })
        }

        #[cfg(not(feature = "smash"))]
        None
    }

    pub fn is_playback(self) -> bool {
        match self {
            LedgeOption::PLAYBACK_1
            | LedgeOption::PLAYBACK_2
            | LedgeOption::PLAYBACK_3
            | LedgeOption::PLAYBACK_4
            | LedgeOption::PLAYBACK_5 => true,
            _ => false,
        }
    }

    pub fn playback_slot(self) -> Option<usize> {
        Some(match self {
            LedgeOption::PLAYBACK_1 => 0,
            LedgeOption::PLAYBACK_2 => 1,
            LedgeOption::PLAYBACK_3 => 2,
            LedgeOption::PLAYBACK_4 => 3,
            LedgeOption::PLAYBACK_5 => 4,
            _ => return None,
        })
    }

    pub const fn default() -> LedgeOption {
        // Neutral,Roll,Jump,Attack (everything except wait)
        LedgeOption {
            NEUTRAL: 1,
            ROLL: 1,
            JUMP: 1,
            ATTACK: 1,
            ..LedgeOption::empty()
        }
    }
}

// Tech options
byteflags! {
    pub struct TechFlags {
        NO_TECH = "No Tech",
        ROLL_F = "Roll Forwards",
        ROLL_B = "Roll Backwards",
        IN_PLACE = "Tech In Place",
    }
}

// Missed Tech Options
byteflags! {
    pub struct MissTechFlags {
        GETUP = "Neutral Getup",
        ATTACK = "Getup Attack",
        ROLL_F = "Roll Forwards",
        ROLL_B = "Roll Backwards",
    }
}

byteflags! {
    pub struct Shield {
        NONE = "None",
        INFINITE = "Infinite",
        HOLD = "Hold",
        CONSTANT = "Constant",
    }
}

byteflags! {
    pub struct SaveStateMirroring {
        NONE = "None",
        ALTERNATE = "Alternate",
        RANDOM = "Random",
    }
}

byteflags! {
    pub struct OnOff {
        ON = "On",
        OFF = "Off",
    }
}

impl OnOff {
    pub fn from_val(val: u32) -> Option<Self> {
        match val {
            1 => Some(OnOff::ON),
            0 => Some(OnOff::OFF),
            _ => None,
        }
    }

    pub fn as_bool(self) -> bool {
        match self {
            OnOff::OFF => false,
            OnOff::ON => true,
            _ => panic!("Bad option for OnOff::as_bool"),
        }
    }
}

byteflags! {
    pub struct Action {
        AIR_DODGE = "Air Dodge",
        JUMP = "Jump",
        SHIELD = "Shield",
        SPOT_DODGE = "Spot Dodge",
        ROLL_F = "Roll Forwards",
        ROLL_B = "Roll Backwards",
        NAIR = "Neutral Air",
        FAIR = "Forward Air",
        BAIR = "Back Air",
        UAIR = "Up Air",
        DAIR = "Down Air",
        NEUTRAL_B = "Neutral Special",
        SIDE_B = "Side Special",
        UP_B = "Up Special",
        DOWN_B = "Down Special",
        F_SMASH = "Forward Smash",
        U_SMASH = "Up Smash",
        D_SMASH = "Down Smash",
        JAB = "Jab",
        F_TILT = "Forward Tilt",
        U_TILT  = "Up Tilt",
        D_TILT  = "Down Tilt",
        GRAB = "Grab",
        DASH = "Dash",
        DASH_ATTACK = "Dash Attack",
        PLAYBACK_1 = "Playback Slot 1",
        PLAYBACK_2 = "Playback Slot 2",
        PLAYBACK_3 = "Playback Slot 3",
        PLAYBACK_4 = "Playback Slot 4",
        PLAYBACK_5 = "Playback Slot 5",
    }
}

impl Action {
    pub fn into_attack_air_kind(self) -> Option<i32> {
        #[cfg(feature = "smash")]
        {
            Some(match self {
                Action::NAIR => *FIGHTER_COMMAND_ATTACK_AIR_KIND_N,
                Action::FAIR => *FIGHTER_COMMAND_ATTACK_AIR_KIND_F,
                Action::BAIR => *FIGHTER_COMMAND_ATTACK_AIR_KIND_B,
                Action::DAIR => *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW,
                Action::UAIR => *FIGHTER_COMMAND_ATTACK_AIR_KIND_HI,
                _ => return None,
            })
        }

        #[cfg(not(feature = "smash"))]
        None
    }

    pub fn is_playback(self) -> bool {
        match self {
            Action::PLAYBACK_1
            | Action::PLAYBACK_2
            | Action::PLAYBACK_3
            | Action::PLAYBACK_4
            | Action::PLAYBACK_5 => true,
            _ => false,
        }
    }

    pub fn playback_slot(self) -> usize {
        match self {
            Action::PLAYBACK_1 => 0,
            Action::PLAYBACK_2 => 1,
            Action::PLAYBACK_3 => 2,
            Action::PLAYBACK_4 => 3,
            Action::PLAYBACK_5 => 4,
            _ => panic!("Invalid Action playback slot: {}", self.to_string()),
        }
    }
}
byteflags! {
    pub struct AttackAngle {
        NEUTRAL = "Neutral",
        UP = "Up",
        DOWN = "Down",
    }
}

byteflags! {
    pub struct Delay {
        D0 = "0",
        D1 = "1",
        D2 = "2",
        D3 = "3",
        D4 = "4",
        D5 = "5",
        D6 = "6",
        D7 = "7",
        D8 = "8",
        D9 = "9",
        D10 = "10",
        D11 = "11",
        D12 = "12",
        D13 = "13",
        D14 = "14",
        D15 = "15",
        D16 = "16",
        D17 = "17",
        D18 = "18",
        D19 = "19",
        D20 = "20",
        D21 = "21",
        D22 = "22",
        D23 = "23",
        D24 = "24",
        D25 = "25",
        D26 = "26",
        D27 = "27",
        D28 = "28",
        D29 = "29",
        D30 = "30",
    }
}

impl Delay {
    pub fn into_delay(&self) -> u32 {
        match *self {
            Delay::D0 => 0,
            Delay::D1 => 1,
            Delay::D2 => 2,
            Delay::D3 => 3,
            Delay::D4 => 4,
            Delay::D5 => 5,
            Delay::D6 => 6,
            Delay::D7 => 7,
            Delay::D8 => 8,
            Delay::D9 => 9,
            Delay::D10 => 10,
            Delay::D11 => 11,
            Delay::D12 => 12,
            Delay::D13 => 13,
            Delay::D14 => 14,
            Delay::D15 => 15,
            Delay::D16 => 16,
            Delay::D17 => 17,
            Delay::D18 => 18,
            Delay::D19 => 19,
            Delay::D20 => 20,
            Delay::D21 => 21,
            Delay::D22 => 22,
            Delay::D23 => 23,
            Delay::D24 => 24,
            Delay::D25 => 25,
            Delay::D26 => 26,
            Delay::D27 => 27,
            Delay::D28 => 28,
            Delay::D29 => 29,
            Delay::D30 => 30,
            _ => panic!("Invalid option for Delay::into_delay()"),
        }
    }
}

byteflags! {
    pub struct MedDelay {
        D0 = "0",
        D5 = "5",
        D10 = "10",
        D15 = "15",
        D20 = "20",
        D25 = "25",
        D30 = "30",
        D35 = "35",
        D40 = "40",
        D45 = "45",
        D50 = "50",
        D55 = "55",
        D60 = "60",
        D65 = "65",
        D70 = "70",
        D75 = "75",
        D80 = "80",
        D85 = "85",
        D90 = "90",
        D95 = "95",
        D100 = "100",
        D105 = "105",
        D110 = "110",
        D115 = "115",
        D120 = "120",
        D125 = "125",
        D130 = "130",
        D135 = "135",
        D140 = "140",
        D145 = "145",
        D150 = "150",
    }
}

impl MedDelay {
    pub fn into_meddelay(&self) -> u32 {
        match *self {
            MedDelay::D0 => 0,
            MedDelay::D5 => 5,
            MedDelay::D10 => 10,
            MedDelay::D15 => 15,
            MedDelay::D20 => 20,
            MedDelay::D25 => 25,
            MedDelay::D30 => 30,
            MedDelay::D35 => 35,
            MedDelay::D40 => 40,
            MedDelay::D45 => 45,
            MedDelay::D50 => 50,
            MedDelay::D55 => 55,
            MedDelay::D60 => 60,
            MedDelay::D65 => 65,
            MedDelay::D70 => 70,
            MedDelay::D75 => 75,
            MedDelay::D80 => 80,
            MedDelay::D85 => 85,
            MedDelay::D90 => 90,
            MedDelay::D95 => 95,
            MedDelay::D100 => 100,
            MedDelay::D105 => 105,
            MedDelay::D110 => 110,
            MedDelay::D115 => 115,
            MedDelay::D120 => 120,
            MedDelay::D125 => 125,
            MedDelay::D130 => 130,
            MedDelay::D135 => 135,
            MedDelay::D140 => 140,
            MedDelay::D145 => 145,
            MedDelay::D150 => 150,
            _ => panic!("Invalid option for MedDelay::into_MedDelay()"),
        }
    }
}

byteflags! {
    pub struct LongDelay {
        D0 = "0",
        D10 = "10",
        D20 = "20",
        D30 = "30",
        D40 = "40",
        D50 = "50",
        D60 = "60",
        D70 = "70",
        D80 = "80",
        D90 = "90",
        D100 = "100",
        D110 = "110",
        D120 = "120",
        D130 = "130",
        D140 = "140",
        D150 = "150",
        D160 = "160",
        D170 = "170",
        D180 = "180",
        D190 = "190",
        D200 = "200",
        D210 = "210",
        D220 = "220",
        D230 = "230",
        D240 = "240",
        D250 = "250",
        D260 = "260",
        D270 = "270",
        D280 = "280",
        D290 = "290",
        D300 = "300",
    }
}

impl LongDelay {
    pub fn into_longdelay(&self) -> u32 {
        match *self {
            LongDelay::D0 => 0,
            LongDelay::D10 => 10,
            LongDelay::D20 => 20,
            LongDelay::D30 => 30,
            LongDelay::D40 => 40,
            LongDelay::D50 => 50,
            LongDelay::D60 => 60,
            LongDelay::D70 => 70,
            LongDelay::D80 => 80,
            LongDelay::D90 => 90,
            LongDelay::D100 => 100,
            LongDelay::D110 => 110,
            LongDelay::D120 => 120,
            LongDelay::D130 => 130,
            LongDelay::D140 => 140,
            LongDelay::D150 => 150,
            LongDelay::D160 => 160,
            LongDelay::D170 => 170,
            LongDelay::D180 => 180,
            LongDelay::D190 => 190,
            LongDelay::D200 => 200,
            LongDelay::D210 => 210,
            LongDelay::D220 => 220,
            LongDelay::D230 => 230,
            LongDelay::D240 => 240,
            LongDelay::D250 => 250,
            LongDelay::D260 => 260,
            LongDelay::D270 => 270,
            LongDelay::D280 => 280,
            LongDelay::D290 => 290,
            LongDelay::D300 => 300,
            _ => panic!("Invalid option for LongDelay::into_LongDelay()"),
        }
    }
}

byteflags! {
    pub struct BuffOption
    {
        ACCELERATLE = "Acceleratle",
        OOMPH = "Oomph",
        PSYCHE = "Psyche Up",
        BOUNCE = "Bounce",
        ARSENE = "Arsene",
        BREATHING = "Deep Breathing",
        LIMIT = "Limit",
        KO = "KO Punch",
        WING = "1-Winged Angel",
        MONAD_JUMP = "Jump",
        MONAD_SPEED = "Speed",
        MONAD_SHIELD = "Shield",
        MONAD_BUSTER = "Buster",
        MONAD_SMASH = "Smash",
        POWER_DRAGON = "Power Dragon",
        WAFT_MINI = "Mini Waft",
        WAFT_HALF = "Half Waft",
        WAFT_FULL = "Full Waft",
    }
}

impl BuffOption {
    pub fn into_int(self) -> Option<i32> {
        #[cfg(feature = "smash")]
        {
            Some(match self {
                BuffOption::ACCELERATLE => *FIGHTER_BRAVE_SPECIAL_LW_COMMAND11_SPEED_UP,
                BuffOption::OOMPH => *FIGHTER_BRAVE_SPECIAL_LW_COMMAND12_ATTACK_UP,
                BuffOption::PSYCHE => *FIGHTER_BRAVE_SPECIAL_LW_COMMAND21_CHARGE,
                BuffOption::BOUNCE => *FIGHTER_BRAVE_SPECIAL_LW_COMMAND13_REFLECT,
                BuffOption::BREATHING => 1,
                BuffOption::ARSENE => 1,
                BuffOption::LIMIT => 1,
                BuffOption::KO => 1,
                BuffOption::WING => 1,
                BuffOption::MONAD_JUMP => *FIGHTER_SHULK_MONAD_TYPE_JUMP,
                BuffOption::MONAD_SPEED => *FIGHTER_SHULK_MONAD_TYPE_SPEED,
                BuffOption::MONAD_SHIELD => *FIGHTER_SHULK_MONAD_TYPE_SHIELD,
                BuffOption::MONAD_BUSTER => *FIGHTER_SHULK_MONAD_TYPE_BUSTER,
                BuffOption::MONAD_SMASH => *FIGHTER_SHULK_MONAD_TYPE_SMASH,
                BuffOption::POWER_DRAGON => 1,
                BuffOption::WAFT_MINI => *FIGHTER_WARIO_GASS_LEVEL_M,
                BuffOption::WAFT_HALF => *FIGHTER_WARIO_GASS_LEVEL_L,
                BuffOption::WAFT_FULL => *FIGHTER_WARIO_GASS_LEVEL_FLY,
                _ => return None,
            })
        }

        #[cfg(not(feature = "smash"))]
        None
    }

    pub fn hero_buffs(self) -> BuffOption {
        // Return a struct with only Hero's selected buffs
        let hero_buffs_byteflags = BuffOption::ACCELERATLE
            .union(BuffOption::OOMPH)
            .union(BuffOption::BOUNCE)
            .union(BuffOption::PSYCHE);
        self.left_intersection(hero_buffs_byteflags)
    }

    pub fn shulk_buffs(self) -> BuffOption {
        // Return a struct with only Shulk's selected arts
        let shulk_buffs_byteflags = BuffOption::MONAD_JUMP
            .union(BuffOption::MONAD_SPEED)
            .union(BuffOption::MONAD_SHIELD)
            .union(BuffOption::MONAD_BUSTER)
            .union(BuffOption::MONAD_SMASH);
        self.left_intersection(shulk_buffs_byteflags)
    }

    pub fn wario_buffs(self) -> BuffOption {
        let wario_buffs_byteflags = BuffOption::WAFT_MINI
            .union(BuffOption::WAFT_HALF)
            .union(BuffOption::WAFT_FULL);
        self.left_intersection(wario_buffs_byteflags)
    }
}

byteflags! {
    pub struct ThrowOption
    {
        NONE = "None",
        FORWARD = "Forward Throw",
        BACKWARD = "Backward Throw",
        UP = "Up Throw",
        DOWN = "Down Throw",
    }
}

impl ThrowOption {
    pub fn into_cmd(self) -> Option<i32> {
        #[cfg(feature = "smash")]
        {
            Some(match self {
                ThrowOption::NONE => 0,
                ThrowOption::FORWARD => *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_F,
                ThrowOption::BACKWARD => *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_B,
                ThrowOption::UP => *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_HI,
                ThrowOption::DOWN => *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_LW,
                _ => return None,
            })
        }

        #[cfg(not(feature = "smash"))]
        None
    }
}

// TODO!() Is this redundant with OnOff?
byteflags! {
    pub struct BoolFlag {
        TRUE = "True",
        FALSE = "False",
    }
}

impl BoolFlag {
    pub fn into_bool(self) -> bool {
        matches!(self, BoolFlag::TRUE)
    }
}

byteflags! {
    pub struct SdiFrequency {
        NONE = "None",
        NORMAL = "Normal",
        MEDIUM = "Medium",
        HIGH = "High",
    }
}

impl SdiFrequency {
    pub fn into_u32(self) -> u32 {
        match self {
            SdiFrequency::NONE => u32::MAX,
            SdiFrequency::NORMAL => 8,
            SdiFrequency::MEDIUM => 6,
            SdiFrequency::HIGH => 4,
            _ => panic!("Invalid option for SdiFrequency::into_u32()"),
        }
    }
}

byteflags! {
    pub struct ClatterFrequency {
        NONE = "None",
        NORMAL = "Normal",
        MEDIUM = "Medium",
        HIGH = "High",
    }
}

impl ClatterFrequency {
    pub fn into_u32(self) -> u32 {
        match self {
            ClatterFrequency::NONE => u32::MAX,
            ClatterFrequency::NORMAL => 8,
            ClatterFrequency::MEDIUM => 5,
            ClatterFrequency::HIGH => 2,
            _ => panic!("Invalid option for ClatterFrequency::into_u32()"),
        }
    }
}

byteflags! {
    pub struct CharacterItem {
        NONE = "None",
        PLAYER_VARIATION_1 = "Player 1st Var.",
        PLAYER_VARIATION_2 = "Player 2nd Var.",
        PLAYER_VARIATION_3 = "Player 3rd Var.",
        PLAYER_VARIATION_4 = "Player 4th Var.",
        PLAYER_VARIATION_5 = "Player 5th Var.",
        PLAYER_VARIATION_6 = "Player 6th Var.",
        PLAYER_VARIATION_7 = "Player 7th Var.",
        PLAYER_VARIATION_8 = "Player 8th Var.",
        CPUV_ARIATION_1 = "CPU 1st Var.",
        CPUV_ARIATION_2 = "CPU 2nd Var.",
        CPUV_ARIATION_3 = "CPU 3rd Var.",
        CPUV_ARIATION_4 = "CPU 4th Var.",
        CPUV_ARIATION_5 = "CPU 5th Var.",
        CPUV_ARIATION_6 = "CPU 6th Var.",
        CPUV_ARIATION_7 = "CPU 7th Var.",
        CPUV_ARIATION_8 = "CPU 8th Var.",
    }
}


byteflags! {
    pub struct MashTrigger {
        HIT = "Hitstun",
        SHIELDSTUN = "Shieldstun",
        PARRY = "Parry",
        TUMBLE = "Tumble",
        LANDING = "Landing",
        TRUMP = "Ledge Trump",
        FOOTSTOOL = "Footstool",
        CLATTER = "Clatter",
        LEDGE = "Ledge Option",
        TECH = "Tech Option",
        MISTECH = "Mistech Option",
        GROUNDED = "Grounded",
        AIRBORNE = "Airborne",
        DISTANCE_CLOSE = "Distance: Close",
        DISTANCE_MID = "Distance: Mid",
        DISTANCE_FAR = "Distance: Far",
        ALWAYS = "Always",
    }
}

impl MashTrigger {
    pub const fn default() -> MashTrigger {
        // Hit, block, clatter
        MashTrigger {
            HIT: 1,
            TUMBLE: 1,
            SHIELDSTUN: 1,
            CLATTER: 1,
            ..MashTrigger::empty()
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct DamagePercent(pub u32, pub u32);

impl DamagePercent {
    fn get_limits() -> (u32, u32) {
        (0, 150)
    }

    fn lower(&self) -> u32 { self.0 }
    fn upper(&self) -> u32 { self.1 }
}

impl DamagePercent {
    pub const fn default() -> DamagePercent {
        DamagePercent(0, 150)
    }
}

byteflags! {
    pub struct SaveDamage
    {
        DEFAULT = "Default",
        SAVED = "Save State",
        RANDOM = "Random Value",
    }
}

byteflags! {
    pub struct SaveStateSlot
    {
        S1 = "Slot 1",
        S2 = "Slot 2",
        S3 = "Slot 3",
        S4 = "Slot 4",
        S5 = "Slot 5",
    }
}

byteflags! {
    pub struct RecordSlot {
        S1 = "Slot 1",
        S2 = "Slot 2",
        S3 = "Slot 3",
        S4 = "Slot 4",
        S5 = "Slot 5",
    }
}

byteflags! {
    pub struct PlaybackSlot {
        S1 = "Slot 1",
        S2 = "Slot 2",
        S3 = "Slot 3",
        S4 = "Slot 4",
        S5 = "Slot 5",
    }
}

// If doing input recording out of hitstun, when does playback begin after?
byteflags! {
    pub struct HitstunPlayback {
        HITSTUN = "As Hitstun Ends",
        HITSTOP = "As Hitstop Ends",
        INSTANT = "As Hitstop Begins",
    }
}

byteflags! {
    pub struct RecordTrigger
    {
        COMMAND = "Button Combination",
        SAVESTATE = "Save State Load",
    }
}

byteflags! {
    pub struct RecordingDuration {
        F60 = "60",
        F90 = "90",
        F120 = "120",
        F150 = "150",
        F180 = "180",
        F210 = "210",
        F240 = "240",
        F270 = "270",
        F300 = "300",
        F330 = "330",
        F360 = "360",
        F390 = "390",
        F420 = "420",
        F450 = "450",
        F480 = "480",
        F510 = "510",
        F540 = "540",
        F570 = "570",
        F600 = "600",
    }
}

impl RecordingDuration {
    pub fn into_frames(&self) -> usize {
        match *self {
            RecordingDuration::F60 => 60,
            RecordingDuration::F90 => 90,
            RecordingDuration::F120 => 120,
            RecordingDuration::F150 => 150,
            RecordingDuration::F180 => 180,
            RecordingDuration::F210 => 210,
            RecordingDuration::F240 => 240,
            RecordingDuration::F270 => 270,
            RecordingDuration::F300 => 300,
            RecordingDuration::F330 => 330,
            RecordingDuration::F360 => 360,
            RecordingDuration::F390 => 390,
            RecordingDuration::F420 => 420,
            RecordingDuration::F450 => 450,
            RecordingDuration::F480 => 480,
            RecordingDuration::F510 => 510,
            RecordingDuration::F540 => 540,
            RecordingDuration::F570 => 570,
            RecordingDuration::F600 => 600,
            _ => panic!("Invalid option for RecordingDuration::into_frames()"),
        }
    }
}

byteflags! {
    pub struct  ButtonConfig {
        A = "A",
        B = "B",
        X = "X",
        Y = "Y",
        L = "Pro L",
        R = "Pro R; GCC Z",
        ZL = "Pro ZL; GCC L",
        ZR = "Pro ZR; GCC R",
        DPAD_UP = "DPad Up",
        DPAD_DOWN = "DPad Down",
        DPAD_LEFT = "DPad Left",
        DPAD_RIGHT = "DPad Right",
        PLUS = "Plus",
        MINUS = "Minus",
        LSTICK = "Left Stick Press",
        RSTICK = "Right Stick Press",
    }
}

byteflags! {
    pub struct UpdatePolicy {
        STABLE = "Stable",
        BETA = "Beta",
        DISABLED = "Disabled",
    }
}

impl UpdatePolicy {
    pub const fn default() -> UpdatePolicy {
        UpdatePolicy::STABLE
    }
}

byteflags! {
    pub struct InputDisplay {
        NONE = "None",
        SMASH = "Smash Inputs",
        RAW = "Raw Inputs",
    }
}
