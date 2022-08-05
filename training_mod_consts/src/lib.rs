#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate num_derive;

use core::f64::consts::PI;
use std::collections::HashMap;
#[cfg(feature = "smash")]
use smash::lib::lua_const::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use serde::{Serialize, Deserialize};
use ramhorns::Content;

pub trait ToggleTrait {
    fn to_toggle_strs() -> Vec<&'static str>;
    fn to_toggle_vals() -> Vec<usize>;
}

// bitflag helper function macro
macro_rules! extra_bitflag_impls {
    ($e:ty) => {
        impl core::fmt::Display for $e {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                core::fmt::Debug::fmt(self, f)
            }
        }

        impl $e {
            pub fn to_vec(&self) -> Vec::<$e> {
                let mut vec = Vec::<$e>::new();
                let mut field = <$e>::from_bits_truncate(self.bits);
                while !field.is_empty() {
                    let flag = <$e>::from_bits(1u32 << field.bits.trailing_zeros()).unwrap();
                    field -= flag;
                    vec.push(flag);
                }
                return vec;
            }

            pub fn to_index(&self) -> u32 {
                if self.bits == 0 {
                    0
                } else {
                    self.bits.trailing_zeros()
                }
            }

            pub fn get_random(&self) -> $e {
                let options = self.to_vec();
                match options.len() {
                    0 => {
                        return <$e>::empty();
                    }
                    1 => {
                        return options[0];
                    }
                    _ => {
                        return *random_option(&options);
                    }
                }
            }
        }
        impl ToggleTrait for $e {
            fn to_toggle_strs() -> Vec<&'static str> {
                let all_options = <$e>::all().to_vec();
                all_options.iter().map(|i| i.as_str().unwrap_or("")).collect()
            }

            fn to_toggle_vals() -> Vec<usize> {
                let all_options = <$e>::all().to_vec();
                all_options.iter().map(|i| i.bits() as usize).collect()
            }
        }
        impl ToUrlParam for $e {
            fn to_url_param(&self) -> String {
                self.bits().to_string()
            }
        }
    }
}

pub fn get_random_int(_max: i32) -> i32 {
    #[cfg(feature = "smash")]
    unsafe { smash::app::sv_math::rand(smash::hash40("fighter"), _max) }

    #[cfg(not(feature = "smash"))]
    0
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
bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct Direction : u32 {
        const OUT = 0x1;
        const UP_OUT = 0x2;
        const UP = 0x4;
        const UP_IN = 0x8;
        const IN = 0x10;
        const DOWN_IN = 0x20;
        const DOWN = 0x40;
        const DOWN_OUT = 0x80;
        const NEUTRAL = 0x100;
        const LEFT = 0x200;
        const RIGHT = 0x400;
    }
}

impl Direction {
    pub fn into_angle(self) -> Option<f64> {
        let index = self.into_index();

        if index == 0 {
            None
        } else {
            Some((index as i32 - 1) as f64 * PI / 4.0)
        }
    }
    fn into_index(self) -> i32 {
        match self {
            Direction::OUT => 1,
            Direction::UP_OUT => 2,
            Direction::UP => 3,
            Direction::UP_IN => 4,
            Direction::IN => 5,
            Direction::DOWN_IN => 6,
            Direction::DOWN => 7,
            Direction::DOWN_OUT => 8,
            Direction::NEUTRAL => 0,
            Direction::LEFT => 5,
            Direction::RIGHT => 1,
            _ => 0,
        }
    }

    fn as_str(self) -> Option<&'static str> {
        Some(match self {
            Direction::OUT => "Away",
            Direction::UP_OUT => "Up and Away",
            Direction::UP => "Up",
            Direction::UP_IN => "Up and In",
            Direction::IN => "In",
            Direction::DOWN_IN => "Down and In",
            Direction::DOWN => "Down",
            Direction::DOWN_OUT => "Down and Away",
            Direction::NEUTRAL => "Neutral",
            Direction::LEFT => "Left",
            Direction::RIGHT => "Right",
            _ => return None,
        })
    }
}

extra_bitflag_impls! {Direction}

// Ledge Option
bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct LedgeOption : u32
    {
        const NEUTRAL = 0x1;
        const ROLL = 0x2;
        const JUMP = 0x4;
        const ATTACK = 0x8;
        const WAIT = 0x10;
    }
}

impl LedgeOption {
    pub fn into_status(self) -> Option<i32> {
        #[cfg(feature = "smash")] {
            Some(match self {
                LedgeOption::NEUTRAL => *FIGHTER_STATUS_KIND_CLIFF_CLIMB,
                LedgeOption::ROLL => *FIGHTER_STATUS_KIND_CLIFF_ESCAPE,
                LedgeOption::JUMP => *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
                LedgeOption::ATTACK => *FIGHTER_STATUS_KIND_CLIFF_ATTACK,
                LedgeOption::WAIT => *FIGHTER_STATUS_KIND_CLIFF_WAIT,
                _ => return None,
            })
        }

        #[cfg(not(feature = "smash"))]
        None
    }

    fn as_str(self) -> Option<&'static str> {
        Some(match self {
            LedgeOption::NEUTRAL => "Neutral Getup",
            LedgeOption::ROLL => "Roll",
            LedgeOption::JUMP => "Jump",
            LedgeOption::ATTACK => "Getup Attack",
            LedgeOption::WAIT => "Wait",
            _ => return None,
        })
    }
}

extra_bitflag_impls! {LedgeOption}

// Tech options
bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct TechFlags : u32 {
        const NO_TECH = 0x1;
        const ROLL_F = 0x2;
        const ROLL_B = 0x4;
        const IN_PLACE = 0x8;
    }
}

impl TechFlags {
    fn as_str(self) -> Option<&'static str> {
        Some(match self {
            TechFlags::NO_TECH => "No Tech",
            TechFlags::ROLL_F => "Roll Forwards",
            TechFlags::ROLL_B => "Roll Backwards",
            TechFlags::IN_PLACE => "Tech In Place",
            _ => return None,
        })
    }
}

extra_bitflag_impls! {TechFlags}

// Missed Tech Options
bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct MissTechFlags : u32 {
        const GETUP = 0x1;
        const ATTACK = 0x2;
        const ROLL_F = 0x4;
        const ROLL_B = 0x8;
    }
}

impl MissTechFlags {
    fn as_str(self) -> Option<&'static str> {
        Some(match self {
            MissTechFlags::GETUP => "Neutral Getup",
            MissTechFlags::ATTACK => "Getup Attack",
            MissTechFlags::ROLL_F => "Roll Forwards",
            MissTechFlags::ROLL_B => "Roll Backwards",
            _ => return None,
        })
    }
}

extra_bitflag_impls! {MissTechFlags}

/// Shield States
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, EnumIter, Serialize, Deserialize)]
pub enum Shield {
    None = 0x0,
    Infinite = 0x1,
    Hold = 0x2,
    Constant = 0x4,
}

impl Shield {
    pub fn as_str(self) -> Option<&'static str> {
        Some(match self {
            Shield::None => "None",
            Shield::Infinite => "Infinite",
            Shield::Hold => "Hold",
            Shield::Constant => "Constant",
        })
    }

    pub fn to_url_param(&self) -> String {
        (*self as i32).to_string()
    }
}

impl ToggleTrait for Shield {
    fn to_toggle_strs() -> Vec<&'static str> {
        Shield::iter().map(|i| i.as_str().unwrap_or("")).collect()
    }

    fn to_toggle_vals() -> Vec<usize> {
        Shield::iter().map(|i| i as usize).collect()
    }
}

// Save State Mirroring
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, EnumIter, Serialize, Deserialize)]
pub enum SaveStateMirroring {
    None = 0x0,
    Alternate = 0x1,
    Random = 0x2,
}

impl SaveStateMirroring {
    pub fn as_str(self) -> Option<&'static str> {
        Some(match self {
            SaveStateMirroring::None => "None",
            SaveStateMirroring::Alternate => "Alternate",
            SaveStateMirroring::Random => "Random",
        })
    }

    fn to_url_param(&self) -> String {
        (*self as i32).to_string()
    }
}

impl ToggleTrait for SaveStateMirroring {
    fn to_toggle_strs() -> Vec<&'static str> {
        SaveStateMirroring::iter().map(|i| i.as_str().unwrap_or("")).collect()
    }

    fn to_toggle_vals() -> Vec<usize> {
        SaveStateMirroring::iter().map(|i| i as usize).collect()
    }
}

// Defensive States
bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct Defensive : u32 {
        const SPOT_DODGE = 0x1;
        const ROLL_F = 0x2;
        const ROLL_B = 0x4;
        const JAB = 0x8;
        const SHIELD = 0x10;
    }
}

impl Defensive {
    fn as_str(self) -> Option<&'static str> {
        Some(match self {
            Defensive::SPOT_DODGE => "Spotdodge",
            Defensive::ROLL_F => "Roll Forwards",
            Defensive::ROLL_B => "Roll Backwards",
            Defensive::JAB => "Jab",
            Defensive::SHIELD => "Shield",
            _ => return None,
        })
    }
}

extra_bitflag_impls! {Defensive}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum OnOff {
    Off = 0,
    On = 1,
}

impl OnOff {
    pub fn from_val(val: u32) -> Option<Self> {
        match val {
            1 => Some(OnOff::On),
            0 => Some(OnOff::Off),
            _ => None,
        }
    }

    pub fn as_str(self) -> Option<&'static str> {
        Some(match self {
            OnOff::Off => "Off",
            OnOff::On => "On",
        })
    }

    pub fn to_url_param(&self) -> String {
        (*self as i32).to_string()
    }
}

impl ToggleTrait for OnOff {
    fn to_toggle_strs() -> Vec<&'static str> {
        vec!["Off", "On"]
    }
    fn to_toggle_vals() -> Vec<usize> {
        vec![0, 1]
    }
}

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct Action : u32 {
        const AIR_DODGE = 0x1;
        const JUMP = 0x2;
        const SHIELD = 0x4;
        const SPOT_DODGE = 0x8;
        const ROLL_F = 0x10;
        const ROLL_B = 0x20;
        const NAIR = 0x40;
        const FAIR = 0x80;
        const BAIR = 0x100;
        const UAIR = 0x200;
        const DAIR = 0x400;
        const NEUTRAL_B = 0x800;
        const SIDE_B = 0x1000;
        const UP_B = 0x2000;
        const DOWN_B = 0x4000;
        const F_SMASH = 0x8000;
        const U_SMASH = 0x10000;
        const D_SMASH = 0x20000;
        const JAB = 0x40000;
        const F_TILT = 0x80000;
        const U_TILT  = 0x0010_0000;
        const D_TILT  = 0x0020_0000;
        const GRAB = 0x0040_0000;
        // TODO: Make work
        const DASH = 0x0080_0000;
        const DASH_ATTACK = 0x0100_0000;
    }
}

impl Action {
    pub fn into_attack_air_kind(self) -> Option<i32> {
        #[cfg(feature = "smash")] {
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

    pub fn as_str(self) -> Option<&'static str> {
        Some(match self {
            Action::AIR_DODGE => "Airdodge",
            Action::JUMP => "Jump",
            Action::SHIELD => "Shield",
            Action::SPOT_DODGE => "Spotdodge",
            Action::ROLL_F => "Roll Forwards",
            Action::ROLL_B => "Roll Backwards",
            Action::NAIR => "Neutral Aerial",
            Action::FAIR => "Forward Aerial",
            Action::BAIR => "Backward Aerial",
            Action::UAIR => "Up Aerial",
            Action::DAIR => "Down Aerial",
            Action::NEUTRAL_B => "Neutral Special",
            Action::SIDE_B => "Side Special",
            Action::UP_B => "Up Special",
            Action::DOWN_B => "Down Special",
            Action::F_SMASH => "Forward Smash",
            Action::U_SMASH => "Up Smash",
            Action::D_SMASH => "Down Smash",
            Action::JAB => "Jab",
            Action::F_TILT => "Forward Tilt",
            Action::U_TILT => "Up Tilt",
            Action::D_TILT => "Down Tilt",
            Action::GRAB => "Grab",
            Action::DASH => "Dash",
            Action::DASH_ATTACK => "Dash Attack",
            _ => return None,
        })
    }
}

extra_bitflag_impls! {Action}

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct AttackAngle : u32 {
        const NEUTRAL = 0x1;
        const UP = 0x2;
        const DOWN = 0x4;
    }
}

impl AttackAngle {
    pub fn as_str(self) -> Option<&'static str> {
        Some(match self {
            AttackAngle::NEUTRAL => "Neutral",
            AttackAngle::UP => "Up",
            AttackAngle::DOWN => "Down",
            _ => return None,
        })
    }
}

extra_bitflag_impls! {AttackAngle}

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct Delay : u32 {
        const D0 = 0x1;
        const D1 = 0x2;
        const D2 = 0x4;
        const D3 = 0x8;
        const D4 = 0x10;
        const D5 = 0x20;
        const D6 = 0x40;
        const D7 = 0x80;
        const D8 = 0x100;
        const D9 = 0x200;
        const D10 = 0x400;
        const D11 = 0x800;
        const D12 = 0x1000;
        const D13 = 0x2000;
        const D14 = 0x4000;
        const D15 = 0x8000;
        const D16 = 0x10000;
        const D17 = 0x20000;
        const D18 = 0x40000;
        const D19 = 0x80000;
        const D20 = 0x0010_0000;
        const D21 = 0x0020_0000;
        const D22 = 0x0040_0000;
        const D23 = 0x0080_0000;
        const D24 = 0x0100_0000;
        const D25 = 0x0200_0000;
        const D26 = 0x0400_0000;
        const D27 = 0x0800_0000;
        const D28 = 0x1000_0000;
        const D29 = 0x2000_0000;
        const D30 = 0x4000_0000;
    }
}

// Throw Option
bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct ThrowOption : u32
    {
        const NONE = 0x1;
        const FORWARD = 0x2;
        const BACKWARD = 0x4;
        const UP = 0x8;
        const DOWN = 0x10;
    }
}

impl ThrowOption {
    pub fn into_cmd(self) -> Option<i32> {
        #[cfg(feature = "smash")] {
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

    pub fn as_str(self) -> Option<&'static str> {
        Some(match self {
            ThrowOption::NONE => "None",
            ThrowOption::FORWARD => "Forward Throw",
            ThrowOption::BACKWARD => "Back Throw",
            ThrowOption::UP => "Up Throw",
            ThrowOption::DOWN => "Down Throw",
            _ => return None,
        })
    }
}

extra_bitflag_impls! {ThrowOption}

// Buff Option
bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct BuffOption : u32
    {
        const ACCELERATLE = 0x1;
        const OOMPH = 0x2;
        const PSYCHE = 0x4;
        const BOUNCE = 0x8;
        const ARSENE = 0x10;
        const BREATHING = 0x20;
        const LIMIT = 0x40;
        const KO = 0x80;
        const WING = 0x100;
    }
}

impl BuffOption {
    pub fn into_int(self) -> Option<i32> {
        #[cfg(feature = "smash")] {
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
                _ => return None,
            })
        }

        #[cfg(not(feature = "smash"))]
        None
    }

    fn as_str(self) -> Option<&'static str> {
        Some(match self {
            BuffOption::ACCELERATLE => "Acceleratle",
            BuffOption::OOMPH => "Oomph",
            BuffOption::BOUNCE => "Bounce",
            BuffOption::PSYCHE => "Psyche Up",
            BuffOption::BREATHING => "Deep Breathing",
            BuffOption::ARSENE => "Arsene",
            BuffOption::LIMIT => "Limit Break",
            BuffOption::KO => "KO Punch",
            BuffOption::WING => "1-Winged Angel",
            _ => return None,
        })
    }
}

extra_bitflag_impls! {BuffOption}

impl Delay {
    pub fn as_str(self) -> Option<&'static str> {
        Some(match self {
            Delay::D0 => "0",
            Delay::D1 => "1",
            Delay::D2 => "2",
            Delay::D3 => "3",
            Delay::D4 => "4",
            Delay::D5 => "5",
            Delay::D6 => "6",
            Delay::D7 => "7",
            Delay::D8 => "8",
            Delay::D9 => "9",
            Delay::D10 => "10",
            Delay::D11 => "11",
            Delay::D12 => "12",
            Delay::D13 => "13",
            Delay::D14 => "14",
            Delay::D15 => "15",
            Delay::D16 => "16",
            Delay::D17 => "17",
            Delay::D18 => "18",
            Delay::D19 => "19",
            Delay::D20 => "20",
            Delay::D21 => "21",
            Delay::D22 => "22",
            Delay::D23 => "23",
            Delay::D24 => "24",
            Delay::D25 => "25",
            Delay::D26 => "26",
            Delay::D27 => "27",
            Delay::D28 => "28",
            Delay::D29 => "29",
            Delay::D30 => "30",
            _ => return None,
        })
    }

    pub fn into_delay(&self) -> u32 {
        self.to_index()
    }
}

extra_bitflag_impls! {Delay}

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct MedDelay : u32 {
        const D0 = 0x1;
        const D5 = 0x2;
        const D10 = 0x4;
        const D15 = 0x8;
        const D20 = 0x10;
        const D25 = 0x20;
        const D30 = 0x40;
        const D35 = 0x80;
        const D40 = 0x100;
        const D45 = 0x200;
        const D50 = 0x400;
        const D55 = 0x800;
        const D60 = 0x1000;
        const D65 = 0x2000;
        const D70 = 0x4000;
        const D75 = 0x8000;
        const D80 = 0x10000;
        const D85 = 0x20000;
        const D90 = 0x40000;
        const D95 = 0x80000;
        const D100 = 0x0010_0000;
        const D105 = 0x0020_0000;
        const D110 = 0x0040_0000;
        const D115 = 0x0080_0000;
        const D120 = 0x0100_0000;
        const D125 = 0x0200_0000;
        const D130 = 0x0400_0000;
        const D135 = 0x0800_0000;
        const D140 = 0x1000_0000;
        const D145 = 0x2000_0000;
        const D150 = 0x4000_0000;
    }
}

impl MedDelay {
    pub fn as_str(self) -> Option<&'static str> {
        Some(match self {
            MedDelay::D0 => "0",
            MedDelay::D5 => "5",
            MedDelay::D10 => "10",
            MedDelay::D15 => "15",
            MedDelay::D20 => "20",
            MedDelay::D25 => "25",
            MedDelay::D30 => "30",
            MedDelay::D35 => "35",
            MedDelay::D40 => "40",
            MedDelay::D45 => "45",
            MedDelay::D50 => "50",
            MedDelay::D55 => "55",
            MedDelay::D60 => "60",
            MedDelay::D65 => "65",
            MedDelay::D70 => "70",
            MedDelay::D75 => "75",
            MedDelay::D80 => "80",
            MedDelay::D85 => "85",
            MedDelay::D90 => "90",
            MedDelay::D95 => "95",
            MedDelay::D100 => "100",
            MedDelay::D105 => "105",
            MedDelay::D110 => "110",
            MedDelay::D115 => "115",
            MedDelay::D120 => "120",
            MedDelay::D125 => "125",
            MedDelay::D130 => "130",
            MedDelay::D135 => "135",
            MedDelay::D140 => "140",
            MedDelay::D145 => "145",
            MedDelay::D150 => "150",
            _ => return None,
        })
    }

    pub fn into_meddelay(&self) -> u32 {
        self.to_index() * 5
    }
}

extra_bitflag_impls! {MedDelay}

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct LongDelay : u32 {
        const D0 = 0x1;
        const D10 = 0x2;
        const D20 = 0x4;
        const D30 = 0x8;
        const D40 = 0x10;
        const D50 = 0x20;
        const D60 = 0x40;
        const D70 = 0x80;
        const D80 = 0x100;
        const D90 = 0x200;
        const D100 = 0x400;
        const D110 = 0x800;
        const D120 = 0x1000;
        const D130 = 0x2000;
        const D140 = 0x4000;
        const D150 = 0x8000;
        const D160 = 0x10000;
        const D170 = 0x20000;
        const D180 = 0x40000;
        const D190 = 0x80000;
        const D200 = 0x0010_0000;
        const D210 = 0x0020_0000;
        const D220 = 0x0040_0000;
        const D230 = 0x0080_0000;
        const D240 = 0x0100_0000;
        const D250 = 0x0200_0000;
        const D260 = 0x0400_0000;
        const D270 = 0x0800_0000;
        const D280 = 0x1000_0000;
        const D290 = 0x2000_0000;
        const D300 = 0x4000_0000;
    }
}

impl LongDelay {
    pub fn as_str(self) -> Option<&'static str> {
        Some(match self {
            LongDelay::D0 => "0",
            LongDelay::D10 => "10",
            LongDelay::D20 => "20",
            LongDelay::D30 => "30",
            LongDelay::D40 => "40",
            LongDelay::D50 => "50",
            LongDelay::D60 => "60",
            LongDelay::D70 => "70",
            LongDelay::D80 => "80",
            LongDelay::D90 => "90",
            LongDelay::D100 => "100",
            LongDelay::D110 => "110",
            LongDelay::D120 => "120",
            LongDelay::D130 => "130",
            LongDelay::D140 => "140",
            LongDelay::D150 => "150",
            LongDelay::D160 => "160",
            LongDelay::D170 => "170",
            LongDelay::D180 => "180",
            LongDelay::D190 => "190",
            LongDelay::D200 => "200",
            LongDelay::D210 => "210",
            LongDelay::D220 => "220",
            LongDelay::D230 => "230",
            LongDelay::D240 => "240",
            LongDelay::D250 => "250",
            LongDelay::D260 => "260",
            LongDelay::D270 => "270",
            LongDelay::D280 => "280",
            LongDelay::D290 => "290",
            LongDelay::D300 => "300",
            _ => return None,
        })
    }

    pub fn into_longdelay(&self) -> u32 {
        self.to_index() * 10
    }
}

extra_bitflag_impls! {LongDelay}

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct BoolFlag : u32 {
        const TRUE = 0x1;
        const FALSE = 0x2;
    }
}

extra_bitflag_impls! {BoolFlag}

impl BoolFlag {
    pub fn into_bool(self) -> bool {
        matches!(self, BoolFlag::TRUE)
    }

    pub fn as_str(self) -> Option<&'static str> {
        Some(match self {
            BoolFlag::TRUE => "True",
            _ => "False",
        })
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, EnumIter, Serialize, Deserialize)]
pub enum InputFrequency {
    None = 0,
    Normal = 1,
    Medium = 2,
    High = 4,
}

impl InputFrequency {
    pub fn into_u32(self) -> u32 {
        match self {
            InputFrequency::None => u32::MAX,
            InputFrequency::Normal => 8,
            InputFrequency::Medium => 6,
            InputFrequency::High => 4,
        }
    }

    pub fn as_str(self) -> Option<&'static str> {
        Some(match self {
            InputFrequency::None => "None",
            InputFrequency::Normal => "Normal",
            InputFrequency::Medium => "Medium",
            InputFrequency::High => "High",
        })
    }

    pub fn to_url_param(&self) -> String {
        (*self as u32).to_string()
    }
}

impl ToggleTrait for InputFrequency {
    fn to_toggle_strs() -> Vec<&'static str> {
        InputFrequency::iter().map(|i| i.as_str().unwrap_or("")).collect()
    }

    fn to_toggle_vals() -> Vec<usize> {
        InputFrequency::iter().map(|i| i as usize).collect()
    }
}

// For input delay
trait ToUrlParam {
    fn to_url_param(&self) -> String;
}

// The i32 is now in log form, need to exponentiate
// back into 2^X when we pass back to the menu
impl ToUrlParam for i32 {
    fn to_url_param(&self) -> String {
        2_i32.pow(*self as u32).to_string()
    }
}

/// Item Selections
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, EnumIter, Serialize, Deserialize)]
pub enum CharacterItem {
    None = 0,
    PlayerVariation1 = 0x1,
    PlayerVariation2 = 0x2,
    PlayerVariation3 = 0x4,
    PlayerVariation4 = 0x8,
    PlayerVariation5 = 0x10,
    PlayerVariation6 = 0x20,
    PlayerVariation7 = 0x40,
    PlayerVariation8 = 0x80,
    CpuVariation1 = 0x100,
    CpuVariation2 = 0x200,
    CpuVariation3 = 0x400,
    CpuVariation4 = 0x800,
    CpuVariation5 = 0x1000,
    CpuVariation6 = 0x2000,
    CpuVariation7 = 0x4000,
    CpuVariation8 = 0x8000,
}

impl CharacterItem {
    pub fn as_idx(self) -> u32 {
        log_2(self as i32 as u32)
    }

    pub fn as_str(self) -> Option<&'static str> {
        Some(match self {
            CharacterItem::PlayerVariation1 => "Player 1st Var.",
            CharacterItem::PlayerVariation2 => "Player 2nd Var.",
            CharacterItem::PlayerVariation3 => "Player 3rd Var.",
            CharacterItem::PlayerVariation4 => "Player 4th Var.",
            CharacterItem::PlayerVariation5 => "Player 5th Var.",
            CharacterItem::PlayerVariation6 => "Player 6th Var.",
            CharacterItem::PlayerVariation7 => "Player 7th Var.",
            CharacterItem::PlayerVariation8 => "Player 8th Var.",
            CharacterItem::CpuVariation1 => "CPU 1st Var.",
            CharacterItem::CpuVariation2 => "CPU 2nd Var.",
            CharacterItem::CpuVariation3 => "CPU 3rd Var.",
            CharacterItem::CpuVariation4 => "CPU 4th Var.",
            CharacterItem::CpuVariation5 => "CPU 5th Var.",
            CharacterItem::CpuVariation6 => "CPU 6th Var.",
            CharacterItem::CpuVariation7 => "CPU 7th Var.",
            CharacterItem::CpuVariation8 => "CPU 8th Var.",
            _ => "None",
        })
    }

    pub fn to_url_param(&self) -> String {
        (*self as i32).to_string()
    }
}

impl ToggleTrait for CharacterItem {
    fn to_toggle_strs() -> Vec<&'static str> {
        CharacterItem::iter().map(|i| i.as_str().unwrap_or("")).collect()
    }

    fn to_toggle_vals() -> Vec<usize> {
        CharacterItem::iter().map(|i| i as usize).collect()
    }
}

// Macro to build the url parameter string
macro_rules! url_params {
    (
        #[repr(C)]
        #[derive($($trait_name:ident, )*)]
        pub struct $e:ident {
            $(pub $field_name:ident: $field_type:ty,)*
        }
    ) => {
        #[repr(C)]
        #[derive($($trait_name, )*)]
        pub struct $e {
            $(pub $field_name: $field_type,)*
        }
        impl $e {
            pub fn to_url_params(&self, defaults: bool) -> String {
                let mut s = "".to_string();
                let defaults_str = match defaults {
                    true => &"__",  // Prefix field name with "__" if it is for the default menu
                    false => &"",
                };
                $(
                    s.push_str(defaults_str);
                    s.push_str(stringify!($field_name));
                    s.push_str(&"=");
                    s.push_str(&self.$field_name.to_url_param());
                    s.push_str(&"&");
                )*
                s.pop();
                s
            }
        }
    }
}

url_params! {
    #[repr(C)]
    #[derive(Clone, Copy, Serialize, Deserialize, Debug, )]
    pub struct TrainingModpackMenu {
        pub hitbox_vis: OnOff,
        pub stage_hazards: OnOff,
        pub di_state: Direction,
        pub sdi_state: Direction,
        pub sdi_strength: InputFrequency,
        pub clatter_strength: InputFrequency,
        pub air_dodge_dir: Direction,
        pub mash_state: Action,
        pub follow_up: Action,
        pub attack_angle: AttackAngle,
        pub ledge_state: LedgeOption,
        pub ledge_delay: LongDelay,
        pub tech_state: TechFlags,
        pub miss_tech_state: MissTechFlags,
        pub shield_state: Shield,
        pub defensive_state: Defensive,
        pub oos_offset: Delay,
        pub reaction_time: Delay,
        pub shield_tilt: Direction,
        pub mash_in_neutral: OnOff,
        pub fast_fall: BoolFlag,
        pub fast_fall_delay: Delay,
        pub falling_aerials: BoolFlag,
        pub aerial_delay: Delay,
        pub full_hop: BoolFlag,
        pub crouch: OnOff,
        pub input_delay: i32,
        pub save_damage: OnOff,
        pub save_state_mirroring: SaveStateMirroring,
        pub frame_advantage: OnOff,
        pub save_state_enable: OnOff,
        pub save_state_autoload: OnOff,
        pub throw_state: ThrowOption,
        pub throw_delay: MedDelay,
        pub pummel_delay: MedDelay,
        pub buff_state: BuffOption,
        pub character_item: CharacterItem,
        pub quick_menu: OnOff,
    }
}

macro_rules! set_by_str {
    ($obj:ident, $s:ident, $($field:ident = $rhs:expr,)*) => {
        $(
            if $s == stringify!($field) {
                $obj.$field = $rhs.unwrap();
            }
        )*
    }
}

const fn num_bits<T>() -> usize { std::mem::size_of::<T>() * 8 }

fn log_2(x: u32) -> u32 {
    if x == 0 { 0 }
    else {
        num_bits::<u32>() as u32 - x.leading_zeros() - 1
    }
}

impl TrainingModpackMenu {
    pub fn set(&mut self, s: &str, val: u32) {
        set_by_str!(
            self,
            s,
            aerial_delay = Delay::from_bits(val),
            air_dodge_dir = Direction::from_bits(val),
            attack_angle = AttackAngle::from_bits(val),
            clatter_strength = num::FromPrimitive::from_u32(val),
            crouch = OnOff::from_val(val),
            defensive_state = Defensive::from_bits(val),
            di_state = Direction::from_bits(val),
            falling_aerials = BoolFlag::from_bits(val),
            fast_fall_delay = Delay::from_bits(val),
            fast_fall = BoolFlag::from_bits(val),
            follow_up = Action::from_bits(val),
            full_hop = BoolFlag::from_bits(val),
            hitbox_vis = OnOff::from_val(val),
            input_delay = Some(log_2(val) as i32),
            ledge_delay = LongDelay::from_bits(val),
            ledge_state = LedgeOption::from_bits(val),
            mash_in_neutral = OnOff::from_val(val),
            mash_state = Action::from_bits(val),
            miss_tech_state = MissTechFlags::from_bits(val),
            oos_offset = Delay::from_bits(val),
            reaction_time = Delay::from_bits(val),
            sdi_state = Direction::from_bits(val),
            sdi_strength = num::FromPrimitive::from_u32(val),
            shield_state = num::FromPrimitive::from_u32(val),
            shield_tilt = Direction::from_bits(val),
            stage_hazards = OnOff::from_val(val),
            tech_state = TechFlags::from_bits(val),
            save_damage = OnOff::from_val(val),
            frame_advantage = OnOff::from_val(val),
            save_state_mirroring = num::FromPrimitive::from_u32(val),
            save_state_enable = OnOff::from_val(val),
            save_state_autoload = OnOff::from_val(val),
            throw_state = ThrowOption::from_bits(val),
            throw_delay = MedDelay::from_bits(val),
            pummel_delay = MedDelay::from_bits(val),
            buff_state = BuffOption::from_bits(val),
            character_item = num::FromPrimitive::from_u32(val),
            quick_menu = OnOff::from_val(val),
        );
    }
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
    pub fn from_str(s : &str) -> SubMenuType {
        match s {
            "toggle" => SubMenuType::TOGGLE,
            "slider" => SubMenuType::SLIDER,
            _ => panic!("Unexpected SubMenuType!")
        }
    }
}

pub static DEFAULTS_MENU: TrainingModpackMenu = TrainingModpackMenu {
    hitbox_vis: OnOff::On,
    stage_hazards: OnOff::Off,
    di_state: Direction::empty(),
    sdi_state: Direction::empty(),
    sdi_strength: InputFrequency::None,
    clatter_strength: InputFrequency::None,
    air_dodge_dir: Direction::empty(),
    mash_state: Action::empty(),
    follow_up: Action::empty(),
    attack_angle: AttackAngle::empty(),
    ledge_state: LedgeOption::all(),
    ledge_delay: LongDelay::empty(),
    tech_state: TechFlags::all(),
    miss_tech_state: MissTechFlags::all(),
    shield_state: Shield::None,
    defensive_state: Defensive::all(),
    oos_offset: Delay::empty(),
    shield_tilt: Direction::empty(),
    reaction_time: Delay::empty(),
    mash_in_neutral: OnOff::Off,
    fast_fall: BoolFlag::empty(),
    fast_fall_delay: Delay::empty(),
    falling_aerials: BoolFlag::empty(),
    aerial_delay: Delay::empty(),
    full_hop: BoolFlag::empty(),
    crouch: OnOff::Off,
    input_delay: 0,
    save_damage: OnOff::On,
    save_state_mirroring: SaveStateMirroring::None,
    frame_advantage: OnOff::Off,
    save_state_enable: OnOff::On,
    save_state_autoload: OnOff::Off,
    throw_state: ThrowOption::NONE,
    throw_delay: MedDelay::empty(),
    pummel_delay: MedDelay::empty(),
    buff_state: BuffOption::empty(),
    character_item: CharacterItem::None,
    quick_menu: OnOff::Off,
};

pub static mut MENU: TrainingModpackMenu = DEFAULTS_MENU;

#[derive(Content, Clone)]
pub struct Slider {
    pub min: usize,
    pub max: usize,
    pub index: usize,
    pub value: usize,
}

#[derive(Content, Clone)]
pub struct Toggle<'a> {
    pub toggle_value: usize,
    pub toggle_title: &'a str,
    pub checked: bool,
}

#[derive(Content, Clone)]
pub struct SubMenu<'a> {
    pub submenu_title: &'a str,
    pub submenu_id: &'a str,
    pub help_text: &'a str,
    pub is_single_option: bool,
    pub toggles: Vec<Toggle<'a>>,
    pub _type: &'a str,
}

impl<'a> SubMenu<'a> {
    pub fn add_toggle(
        &mut self,
        toggle_value: usize,
        toggle_title: &'a str
    ) {
        self.toggles.push(
            Toggle {
                toggle_value: toggle_value,
                toggle_title: toggle_title,
                checked: false
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
                _type: "toggle"
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
pub struct UiMenu<'a> {
    pub tabs: Vec<Tab<'a>>,
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



pub unsafe fn get_menu() -> UiMenu<'static> {
    let mut overall_menu = UiMenu {
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
    defensive_tab.add_submenu_with_toggles::<InputFrequency>(
        "SDI Strength",
        "sdi_strength",
        "SDI Strength: Relative strength of the smash directional influence inputs",
        true,
    );
    defensive_tab.add_submenu_with_toggles::<InputFrequency>(
        "Clatter Strength",
        "clatter_strength",
        "Clatter Strength: Relative strength of the mashing out of grabs, buries, etc.",
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
        "Escape Toggles",
        "defensive_state",
        "Escape Options: Actions to take after a ledge option, tech option, or mistech option",
        false,
    );
    defensive_tab.add_submenu_with_toggles::<BuffOption>(
        "Buff Options",
        "buff_state",
        "Buff Options: Buff(s) to be applied to respective character when loading save states",
        false,
    );
    defensive_tab.add_submenu_with_toggles::<CharacterItem>(
        "Character Item",
        "character_item",
        "Character Item: CPU/Player item to hold when loading a save state",
        true
    );
    defensive_tab.add_submenu_with_toggles::<OnOff>(
        "Crouch",
        "crouch",
        "Crouch: Should the CPU crouch when on the ground",
        true
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
        "Save States Autoload",
        "save_state_autoload",
        "Save States Autoload: Load save state when any fighter dies",
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
    misc_tab.add_submenu_with_toggles::<OnOff>(
        "Quick Menu",
        "quick_menu",
        "Quick Menu: Should use quick or web menu",
        true
    );
    overall_menu.tabs.push(misc_tab);

    let non_ui_menu = MENU;
    let url_params = non_ui_menu.to_url_params(false);
    let toggle_values_all = url_params.split("&");
    let mut sub_menu_id_to_vals : HashMap<&str, u32> = HashMap::new();
    for toggle_values in toggle_values_all {
        let toggle_value_split = toggle_values.split('=').collect::<Vec<&str>>();
        let mut sub_menu_id = toggle_value_split[0];
        if sub_menu_id.is_empty() { continue }
        sub_menu_id = sub_menu_id.strip_prefix("__").unwrap_or(sub_menu_id);

        let full_bits: u32 = toggle_value_split[1].parse().unwrap_or(0);
        sub_menu_id_to_vals.insert(sub_menu_id, full_bits);
    }
    overall_menu.tabs.iter_mut()
        .for_each(|tab| {
            tab.tab_submenus.iter_mut().for_each(|sub_menu| {
                let sub_menu_id = sub_menu.submenu_id;
                sub_menu.toggles.iter_mut().for_each(|toggle| {
                    if sub_menu_id_to_vals.contains_key(sub_menu_id) &&
                        (sub_menu_id_to_vals[sub_menu_id] & (toggle.toggle_value as u32) != 0) {
                        toggle.checked = true
                    }
                })
            })
        });

    overall_menu
}
