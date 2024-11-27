use crate::TOGGLE_MAX;
use byteflags::*;
use core::f64::consts::PI;

#[cfg(feature = "smash")]
use smash::lib::lua_const::*;
use training_mod_tui::{
    StatefulSlider, StatefulTable, SubMenu, SubMenuType, Toggle, NX_SUBMENU_COLUMNS,
    NX_SUBMENU_ROWS,
};

pub trait SubMenuTrait {
    fn to_submenu<'a>(
        title: &'a str,
        id: &'a str,
        help_text: &'a str,
        submenu_type: SubMenuType,
    ) -> SubMenu<'a>;
}

#[macro_export]
macro_rules! impl_submenutrait {
    ($e:ty) => {
        impl SubMenuTrait for $e {
            fn to_submenu<'a>(
                title: &'a str,
                id: &'a str,
                help_text: &'a str,
                submenu_type: SubMenuType,
            ) -> SubMenu<'a> {
                match submenu_type {
                    SubMenuType::ToggleSingle => {
                        let value = 0;
                        let max = 1;
                        let toggles_vec: Vec<Toggle> = Self::ALL_NAMES
                            .iter()
                            .map(|title| Toggle { title, value, max })
                            .collect();
                        SubMenu {
                            title: title,
                            id: id,
                            help_text: help_text,
                            submenu_type: submenu_type,
                            toggles: StatefulTable::with_items(
                                NX_SUBMENU_ROWS,
                                NX_SUBMENU_COLUMNS,
                                toggles_vec,
                            ),
                            slider: None,
                        }
                    }
                    SubMenuType::ToggleMultiple => {
                        let value = 0;
                        let max = TOGGLE_MAX;
                        let toggles_vec: Vec<Toggle> = Self::ALL_NAMES
                            .iter()
                            .map(|title| Toggle { title, value, max })
                            .collect();
                        SubMenu {
                            title: title,
                            id: id,
                            help_text: help_text,
                            submenu_type: submenu_type,
                            toggles: StatefulTable::with_items(
                                NX_SUBMENU_ROWS,
                                NX_SUBMENU_COLUMNS,
                                toggles_vec,
                            ),
                            slider: None,
                        }
                    }
                    SubMenuType::Slider => {
                        let slider = StatefulSlider {
                            lower: 0,
                            upper: 150,
                            ..StatefulSlider::new()
                        };
                        SubMenu {
                            title: title,
                            id: id,
                            help_text: help_text,
                            submenu_type: submenu_type,
                            toggles: StatefulTable::with_items(
                                NX_SUBMENU_ROWS,
                                NX_SUBMENU_COLUMNS,
                                Vec::new(),
                            ),
                            slider: Some(slider),
                        }
                    }
                }
            }
        }
    };
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
        pub OUT = "Out",
        pub UP_OUT = "Up Out",
        pub UP = "Up",
        pub UP_IN = "Up In",
        pub IN = "In",
        pub DOWN_IN = "Down In",
        pub DOWN = "Down",
        pub DOWN_OUT = "Down Out",
        pub NEUTRAL = "Neutral",
        pub LEFT = "Left",
        pub RIGHT = "Right",
    }
}

impl_submenutrait!(Direction);

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
        if self == Direction::empty() {
            return 0.0;
        };
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
            _ => panic!("Invalid value in Direction::into_index: {}", self),
        }
    }
}

// Ledge Option
byteflags! {
    pub struct LedgeOption
    {
        pub NEUTRAL = "Neutral Getup",
        pub ROLL = "Roll",
        pub JUMP = "Jump",
        pub ATTACK = "Getup Attack",
        pub WAIT = "Wait",
        pub PLAYBACK_1 = "Playback Slot 1",
        pub PLAYBACK_2 = "Playback Slot 2",
        pub PLAYBACK_3 = "Playback Slot 3",
        pub PLAYBACK_4 = "Playback Slot 4",
        pub PLAYBACK_5 = "Playback Slot 5",
    }
}

impl_submenutrait!(LedgeOption);

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
        pub NO_TECH = "No Tech",
        pub ROLL_F = "Roll Forwards",
        pub ROLL_B = "Roll Backwards",
        pub IN_PLACE = "Tech In Place",
    }
}

impl_submenutrait!(TechFlags);

// Missed Tech Options
byteflags! {
    pub struct MissTechFlags {
        pub GETUP = "Neutral Getup",
        pub ATTACK = "Getup Attack",
        pub ROLL_F = "Roll Forwards",
        pub ROLL_B = "Roll Backwards",
    }
}

impl_submenutrait!(MissTechFlags);

byteflags! {
    pub struct Shield {
        pub NONE = "None",
        pub INFINITE = "Infinite",
        pub HOLD = "Hold",
        pub CONSTANT = "Constant",
    }
}

impl_submenutrait!(Shield);

byteflags! {
    pub struct SaveStateMirroring {
        pub NONE = "None",
        pub ALTERNATE = "Alternate",
        pub RANDOM = "Random",
    }
}

impl_submenutrait!(SaveStateMirroring);

byteflags! {
    pub struct OnOff {
        pub ON = "On",
        pub OFF = "Off",
    }
}

impl_submenutrait!(OnOff);

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
            _ => panic!("Invalid value in OnOff::as_bool: {}", self),
        }
    }
}

byteflags! {
    pub struct Action {
        pub AIR_DODGE = "Air Dodge",
        pub JUMP = "Jump",
        pub SHIELD = "Shield",
        pub SPOT_DODGE = "Spot Dodge",
        pub ROLL_F = "Roll Forwards",
        pub ROLL_B = "Roll Backwards",
        pub NAIR = "Neutral Air",
        pub FAIR = "Forward Air",
        pub BAIR = "Back Air",
        pub UAIR = "Up Air",
        pub DAIR = "Down Air",
        pub NEUTRAL_B = "Neutral Special",
        pub SIDE_B = "Side Special",
        pub UP_B = "Up Special",
        pub DOWN_B = "Down Special",
        pub F_SMASH = "Forward Smash",
        pub U_SMASH = "Up Smash",
        pub D_SMASH = "Down Smash",
        pub JAB = "Jab",
        pub F_TILT = "Forward Tilt",
        pub U_TILT  = "Up Tilt",
        pub D_TILT  = "Down Tilt",
        pub GRAB = "Grab",
        pub DASH = "Dash",
        pub DASH_ATTACK = "Dash Attack",
        pub PLAYBACK_1 = "Playback Slot 1",
        pub PLAYBACK_2 = "Playback Slot 2",
        pub PLAYBACK_3 = "Playback Slot 3",
        pub PLAYBACK_4 = "Playback Slot 4",
        pub PLAYBACK_5 = "Playback Slot 5",
    }
}

impl_submenutrait!(Action);

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
        pub NEUTRAL = "Neutral",
        pub UP = "Up",
        pub DOWN = "Down",
    }
}

impl_submenutrait!(AttackAngle);

byteflags! {
    pub struct Delay {
        pub D0 = "0",
        pub D1 = "1",
        pub D2 = "2",
        pub D3 = "3",
        pub D4 = "4",
        pub D5 = "5",
        pub D6 = "6",
        pub D7 = "7",
        pub D8 = "8",
        pub D9 = "9",
        pub D10 = "10",
        pub D11 = "11",
        pub D12 = "12",
        pub D13 = "13",
        pub D14 = "14",
        pub D15 = "15",
        pub D16 = "16",
        pub D17 = "17",
        pub D18 = "18",
        pub D19 = "19",
        pub D20 = "20",
        pub D21 = "21",
        pub D22 = "22",
        pub D23 = "23",
        pub D24 = "24",
        pub D25 = "25",
        pub D26 = "26",
        pub D27 = "27",
        pub D28 = "28",
        pub D29 = "29",
        pub D30 = "30",
    }
}

impl_submenutrait!(Delay);

impl Delay {
    pub fn into_delay(&self) -> u32 {
        if *self == Delay::empty() {
            return 0;
        };
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
            _ => panic!("Invalid value in Delay::into_delay: {}", self),
        }
    }
}

byteflags! {
    pub struct MedDelay {
        pub D0 = "0",
        pub D5 = "5",
        pub D10 = "10",
        pub D15 = "15",
        pub D20 = "20",
        pub D25 = "25",
        pub D30 = "30",
        pub D35 = "35",
        pub D40 = "40",
        pub D45 = "45",
        pub D50 = "50",
        pub D55 = "55",
        pub D60 = "60",
        pub D65 = "65",
        pub D70 = "70",
        pub D75 = "75",
        pub D80 = "80",
        pub D85 = "85",
        pub D90 = "90",
        pub D95 = "95",
        pub D100 = "100",
        pub D105 = "105",
        pub D110 = "110",
        pub D115 = "115",
        pub D120 = "120",
        pub D125 = "125",
        pub D130 = "130",
        pub D135 = "135",
        pub D140 = "140",
        pub D145 = "145",
        pub D150 = "150",
    }
}

impl_submenutrait!(MedDelay);

impl MedDelay {
    pub fn into_meddelay(&self) -> u32 {
        if *self == MedDelay::empty() {
            return 0;
        };
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
            _ => panic!("Invalid value in MedDelay::into_meddelay: {}", self),
        }
    }
}

byteflags! {
    pub struct LongDelay {
        pub D0 = "0",
        pub D10 = "10",
        pub D20 = "20",
        pub D30 = "30",
        pub D40 = "40",
        pub D50 = "50",
        pub D60 = "60",
        pub D70 = "70",
        pub D80 = "80",
        pub D90 = "90",
        pub D100 = "100",
        pub D110 = "110",
        pub D120 = "120",
        pub D130 = "130",
        pub D140 = "140",
        pub D150 = "150",
        pub D160 = "160",
        pub D170 = "170",
        pub D180 = "180",
        pub D190 = "190",
        pub D200 = "200",
        pub D210 = "210",
        pub D220 = "220",
        pub D230 = "230",
        pub D240 = "240",
        pub D250 = "250",
        pub D260 = "260",
        pub D270 = "270",
        pub D280 = "280",
        pub D290 = "290",
        pub D300 = "300",
    }
}

impl_submenutrait!(LongDelay);

impl LongDelay {
    pub fn into_longdelay(&self) -> u32 {
        if *self == LongDelay::empty() {
            return 0;
        };
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
            _ => panic!("Invalid value in LongDelay::into_longdelay: {}", self),
        }
    }
}

byteflags! {
    pub struct BuffOption
    {
        pub ACCELERATLE = "Acceleratle",
        pub OOMPH = "Oomph",
        pub PSYCHE = "Psyche Up",
        pub BOUNCE = "Bounce",
        pub ARSENE = "Arsene",
        pub BREATHING = "Deep Breathing",
        pub LIMIT = "Limit",
        pub KO = "KO Punch",
        pub WING = "1-Winged Angel",
        pub MONAD_JUMP = "Jump",
        pub MONAD_SPEED = "Speed",
        pub MONAD_SHIELD = "Shield",
        pub MONAD_BUSTER = "Buster",
        pub MONAD_SMASH = "Smash",
        pub POWER_DRAGON = "Power Dragon",
        pub WAFT_MINI = "Mini Waft",
        pub WAFT_HALF = "Half Waft",
        pub WAFT_FULL = "Full Waft",
    }
}

impl_submenutrait!(BuffOption);

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

impl_submenutrait!(ThrowOption);

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
        pub TRUE = "True",
        pub FALSE = "False",
    }
}

impl_submenutrait!(BoolFlag);

impl BoolFlag {
    pub fn into_bool(self) -> bool {
        matches!(self, BoolFlag::TRUE)
    }
}

byteflags! {
    pub struct SdiFrequency {
        pub NONE = "None",
        pub NORMAL = "Normal",
        pub MEDIUM = "Medium",
        pub HIGH = "High",
    }
}

impl_submenutrait!(SdiFrequency);

impl SdiFrequency {
    pub fn into_u32(self) -> u32 {
        match self {
            SdiFrequency::NONE => u32::MAX,
            SdiFrequency::NORMAL => 8,
            SdiFrequency::MEDIUM => 6,
            SdiFrequency::HIGH => 4,
            _ => panic!("Invalid value in SdiFrequency::into_u32: {}", self),
        }
    }
}

byteflags! {
    pub struct ClatterFrequency {
        pub NONE = "None",
        pub NORMAL = "Normal",
        pub MEDIUM = "Medium",
        pub HIGH = "High",
    }
}

impl_submenutrait!(ClatterFrequency);

impl ClatterFrequency {
    pub fn into_u32(self) -> u32 {
        match self {
            ClatterFrequency::NONE => u32::MAX,
            ClatterFrequency::NORMAL => 8,
            ClatterFrequency::MEDIUM => 5,
            ClatterFrequency::HIGH => 2,
            _ => panic!("Invalid value in ClatterFrequency::into_u32: {}", self),
        }
    }
}

byteflags! {
    pub struct CharacterItem {
        pub NONE = "None",
        pub PLAYER_VARIATION_1 = "Player 1st Var.",
        pub PLAYER_VARIATION_2 = "Player 2nd Var.",
        pub PLAYER_VARIATION_3 = "Player 3rd Var.",
        pub PLAYER_VARIATION_4 = "Player 4th Var.",
        pub PLAYER_VARIATION_5 = "Player 5th Var.",
        pub PLAYER_VARIATION_6 = "Player 6th Var.",
        pub PLAYER_VARIATION_7 = "Player 7th Var.",
        pub PLAYER_VARIATION_8 = "Player 8th Var.",
        pub CPU_VARIATION_1 = "CPU 1st Var.",
        pub CPU_VARIATION_2 = "CPU 2nd Var.",
        pub CPU_VARIATION_3 = "CPU 3rd Var.",
        pub CPU_VARIATION_4 = "CPU 4th Var.",
        pub CPU_VARIATION_5 = "CPU 5th Var.",
        pub CPU_VARIATION_6 = "CPU 6th Var.",
        pub CPU_VARIATION_7 = "CPU 7th Var.",
        pub CPU_VARIATION_8 = "CPU 8th Var.",
    }
}

impl_submenutrait!(CharacterItem);

impl CharacterItem {
    pub fn as_idx(&self) -> usize {
        match *self {
            CharacterItem::NONE => 0,
            CharacterItem::PLAYER_VARIATION_1 => 1,
            CharacterItem::PLAYER_VARIATION_2 => 2,
            CharacterItem::PLAYER_VARIATION_3 => 3,
            CharacterItem::PLAYER_VARIATION_4 => 4,
            CharacterItem::PLAYER_VARIATION_5 => 5,
            CharacterItem::PLAYER_VARIATION_6 => 6,
            CharacterItem::PLAYER_VARIATION_7 => 7,
            CharacterItem::PLAYER_VARIATION_8 => 8,
            CharacterItem::CPU_VARIATION_1 => 9,
            CharacterItem::CPU_VARIATION_2 => 10,
            CharacterItem::CPU_VARIATION_3 => 11,
            CharacterItem::CPU_VARIATION_4 => 12,
            CharacterItem::CPU_VARIATION_5 => 13,
            CharacterItem::CPU_VARIATION_6 => 14,
            CharacterItem::CPU_VARIATION_7 => 15,
            CharacterItem::CPU_VARIATION_8 => 16,
            _ => panic!("Invalid value in CharacterItem::as_idx: {}", self),
        }
    }
}

byteflags! {
    pub struct MashTrigger {
        pub HIT = "Hitstun",
        pub SHIELDSTUN = "Shieldstun",
        pub PARRY = "Parry",
        pub TUMBLE = "Tumble",
        pub LANDING = "Landing",
        pub TRUMP = "Ledge Trump",
        pub FOOTSTOOL = "Footstool",
        pub CLATTER = "Clatter",
        pub LEDGE = "Ledge Option",
        pub TECH = "Tech Option",
        pub MISTECH = "Mistech Option",
        pub GROUNDED = "Grounded",
        pub AIRBORNE = "Airborne",
        pub DISTANCE_CLOSE = "Distance: Close",
        pub DISTANCE_MID = "Distance: Mid",
        pub DISTANCE_FAR = "Distance: Far",
        pub ALWAYS = "Always",
    }
}

impl_submenutrait!(MashTrigger);

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

byteflags! {
    pub struct DamagePercent {
        pub LOWER = "Lower",
        pub UPPER = "Upper",
    }
}

impl_submenutrait!(DamagePercent);

impl DamagePercent {
    pub const fn default() -> DamagePercent {
        DamagePercent {
            LOWER: 0,
            UPPER: 150,
        }
    }
}

byteflags! {
    pub struct SaveDamage {
        pub DEFAULT = "Default",
        pub SAVED = "Save State",
        pub RANDOM = "Random Value",
    }
}

impl_submenutrait!(SaveDamage);

byteflags! {
    pub struct SaveStateSlot
    {
        pub S1 = "Slot 1",
        pub S2 = "Slot 2",
        pub S3 = "Slot 3",
        pub S4 = "Slot 4",
        pub S5 = "Slot 5",
    }
}

impl_submenutrait!(SaveStateSlot);

impl SaveStateSlot {
    pub fn into_idx(&self) -> Option<usize> {
        match *self {
            SaveStateSlot::S1 => Some(0),
            SaveStateSlot::S2 => Some(1),
            SaveStateSlot::S3 => Some(2),
            SaveStateSlot::S4 => Some(3),
            SaveStateSlot::S5 => Some(4),
            _ => None,
        }
    }
}

byteflags! {
    pub struct RecordSlot {
        pub S1 = "Slot 1",
        pub S2 = "Slot 2",
        pub S3 = "Slot 3",
        pub S4 = "Slot 4",
        pub S5 = "Slot 5",
    }
}

impl_submenutrait!(RecordSlot);

impl RecordSlot {
    pub fn into_idx(&self) -> Option<usize> {
        match *self {
            RecordSlot::S1 => Some(0),
            RecordSlot::S2 => Some(1),
            RecordSlot::S3 => Some(2),
            RecordSlot::S4 => Some(3),
            RecordSlot::S5 => Some(4),
            _ => None,
        }
    }
}

byteflags! {
    pub struct PlaybackSlot {
        pub S1 = "Slot 1",
        pub S2 = "Slot 2",
        pub S3 = "Slot 3",
        pub S4 = "Slot 4",
        pub S5 = "Slot 5",
    }
}

impl_submenutrait!(PlaybackSlot);

impl PlaybackSlot {
    pub fn into_idx(&self) -> Option<usize> {
        match *self {
            PlaybackSlot::S1 => Some(0),
            PlaybackSlot::S2 => Some(1),
            PlaybackSlot::S3 => Some(2),
            PlaybackSlot::S4 => Some(3),
            PlaybackSlot::S5 => Some(4),
            _ => None,
        }
    }
}

// If doing input recording out of hitstun, when does playback begin after?
byteflags! {
    pub struct HitstunPlayback {
        pub HITSTUN = "As Hitstun Ends",
        pub HITSTOP = "As Hitstop Ends",
        pub INSTANT = "As Hitstop Begins",
    }
}

impl_submenutrait!(HitstunPlayback);

byteflags! {
    pub struct RecordTrigger {
        pub COMMAND = "Button Combination",
        pub SAVESTATE = "Save State Load",
    }
}

impl_submenutrait!(RecordTrigger);

byteflags! {
    pub struct RecordingDuration {
        pub F60 = "60",
        pub F90 = "90",
        pub F120 = "120",
        pub F150 = "150",
        pub F180 = "180",
        pub F210 = "210",
        pub F240 = "240",
        pub F270 = "270",
        pub F300 = "300",
        pub F330 = "330",
        pub F360 = "360",
        pub F390 = "390",
        pub F420 = "420",
        pub F450 = "450",
        pub F480 = "480",
        pub F510 = "510",
        pub F540 = "540",
        pub F570 = "570",
        pub F600 = "600",
    }
}

impl_submenutrait!(RecordingDuration);

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
            _ => panic!("Invalid value in RecordingDuration::into_frames: {}", self),
        }
    }
}

byteflags! {
    pub struct  ButtonConfig {
        pub A = "A",
        pub B = "B",
        pub X = "X",
        pub Y = "Y",
        pub L = "Pro L",
        pub R = "Pro R; GCC Z",
        pub ZL = "Pro ZL; GCC L",
        pub ZR = "Pro ZR; GCC R",
        pub DPAD_UP = "DPad Up",
        pub DPAD_DOWN = "DPad Down",
        pub DPAD_LEFT = "DPad Left",
        pub DPAD_RIGHT = "DPad Right",
        pub PLUS = "Plus",
        pub MINUS = "Minus",
        pub LSTICK = "Left Stick Press",
        pub RSTICK = "Right Stick Press",
    }
}

impl_submenutrait!(ButtonConfig);

byteflags! {
    pub struct UpdatePolicy {
        pub STABLE = "Stable",
        pub BETA = "Beta",
        pub DISABLED = "Disabled",
    }
}

impl_submenutrait!(UpdatePolicy);

impl UpdatePolicy {
    pub const fn default() -> UpdatePolicy {
        UpdatePolicy::STABLE
    }
}

byteflags! {
    pub struct InputDisplay {
        pub NONE = "None",
        pub SMASH = "Smash Inputs",
        pub RAW = "Raw Inputs",
        pub STATUS = "Status Only",
    }
}

impl_submenutrait!(InputDisplay);

byteflags! {
    pub struct Locale {
        pub ENGLISH_US = "English (US)",
        pub FRENCH = "French",
    }
}

impl_submenutrait!(Locale);

impl Locale {
    pub const fn default() -> Locale {
        Locale::ENGLISH_US
    }
}

impl From<u8> for Locale {
    fn from(id: u8) -> Locale {
        match id {
            0 => Locale::ENGLISH_US,
            1 => Locale::FRENCH,
            _ => Locale::ENGLISH_US,
        }
    }
}
