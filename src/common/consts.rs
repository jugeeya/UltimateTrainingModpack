use crate::common::get_random_int;
use core::f64::consts::PI;
use smash::lib::lua_const::*;
use strum_macros::EnumIter;

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

            pub fn to_toggle_strs() -> Vec<&'static str> {
                let all_options = <$e>::all().to_vec();
                all_options.iter().map(|i| i.as_str().unwrap_or("")).collect()
            }

            pub fn to_toggle_vals() -> Vec<usize> {
                let all_options = <$e>::all().to_vec();
                all_options.iter().map(|i| i.bits() as usize).collect()
            }
            pub fn to_url_param(&self) -> String {
                self.to_vec()
                    .into_iter()
                    .map(|field| field.bits().to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            }
        }
    }
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
    pub struct Direction : u32
    {
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
        Some(match self {
            LedgeOption::NEUTRAL => *FIGHTER_STATUS_KIND_CLIFF_CLIMB,
            LedgeOption::ROLL => *FIGHTER_STATUS_KIND_CLIFF_ESCAPE,
            LedgeOption::JUMP => *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
            LedgeOption::ATTACK => *FIGHTER_STATUS_KIND_CLIFF_ATTACK,
            LedgeOption::WAIT => *FIGHTER_STATUS_KIND_CLIFF_WAIT,
            _ => return None,
        })
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
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, EnumIter)]
pub enum Shield {
    None = 0,
    Infinite = 1,
    Hold = 2,
    Constant = 3,
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

// Save State Mirroring
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, EnumIter)]
pub enum SaveStateMirroring {
    None = 0,
    Alternate = 1,
    Random = 2,
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

// Defensive States
bitflags! {
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
#[derive(Debug, Clone, Copy, PartialEq)]
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

bitflags! {
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
        Some(match self {
            Action::NAIR => *FIGHTER_COMMAND_ATTACK_AIR_KIND_N,
            Action::FAIR => *FIGHTER_COMMAND_ATTACK_AIR_KIND_F,
            Action::BAIR => *FIGHTER_COMMAND_ATTACK_AIR_KIND_B,
            Action::DAIR => *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW,
            Action::UAIR => *FIGHTER_COMMAND_ATTACK_AIR_KIND_HI,
            _ => return None,
        })
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
        Some(match self {
            ThrowOption::NONE => 0,
            ThrowOption::FORWARD => *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_F,
            ThrowOption::BACKWARD => *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_B,
            ThrowOption::UP => *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_HI,
            ThrowOption::DOWN => *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_LW,
            _ => return None,
        })
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

    fn as_str(self) -> Option<&'static str> {
        Some(match self {
            BuffOption::ACCELERATLE => "Acceleratle",
            BuffOption::OOMPH => "Oomph",
            BuffOption::PSYCHE => "Psyche Up",
            BuffOption::BOUNCE => "Bounce",
            BuffOption::BREATHING => "Deep Breathing",
            BuffOption::ARSENE => "Arsene",
            BuffOption::LIMIT => "Limit Break",
            BuffOption::KO => "KO Punch",
            BuffOption::WING => "One-Winged Angel",
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, EnumIter)]
pub enum SdiStrength {
    Normal = 0,
    Medium = 1,
    High = 2,
}

impl SdiStrength {
    pub fn into_u32(self) -> u32 {
        match self {
            SdiStrength::Normal => 8,
            SdiStrength::Medium => 6,
            SdiStrength::High => 4,
        }
    }

    pub fn as_str(self) -> Option<&'static str> {
        Some(match self {
            SdiStrength::Normal => "Normal",
            SdiStrength::Medium => "Medium",
            SdiStrength::High => "High",
        })
    }

    pub fn to_url_param(&self) -> String {
        (*self as u32).to_string()
    }
}

// For input delay
trait ToUrlParam {
    fn to_url_param(&self) -> String;
}

impl ToUrlParam for i32 {
    fn to_url_param(&self) -> String {
        self.to_string()
    }
}

// Macro to build the url parameter string
macro_rules! url_params {
    (
        #[derive($($trait_name:ident, )*)]
        pub struct $e:ident {
            $(pub $field_name:ident: $field_type:ty,)*
        }
    ) => {
        #[derive($($trait_name, )*)]
        pub struct $e {
            $(pub $field_name: $field_type,)*
        }
        impl $e {
            pub fn to_url_params(&self) -> String {
                let mut s = "?".to_string();
                $(
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

#[repr(C)]
url_params! {
    #[derive(Clone, Copy, )]
    pub struct TrainingModpackMenu {
        pub hitbox_vis: OnOff,
        pub stage_hazards: OnOff,
        pub di_state: Direction,
        pub sdi_state: Direction,
        pub sdi_strength: SdiStrength,
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
        pub input_delay: i32,
        pub save_damage: OnOff,
        pub save_state_mirroring: SaveStateMirroring,
        pub frame_advantage: OnOff,
        pub save_state_enable: OnOff,
        pub throw_state: ThrowOption,
        pub throw_delay: MedDelay,
        pub pummel_delay: MedDelay,
        pub buff_state: BuffOption,
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

impl TrainingModpackMenu {
    pub fn set(&mut self, s: &str, val: u32) {
        set_by_str!(
            self,
            s,
            aerial_delay = Delay::from_bits(val),
            air_dodge_dir = Direction::from_bits(val),
            attack_angle = AttackAngle::from_bits(val),
            defensive_state = Defensive::from_bits(val),
            di_state = Direction::from_bits(val),
            falling_aerials = BoolFlag::from_bits(val),
            fast_fall_delay = Delay::from_bits(val),
            fast_fall = BoolFlag::from_bits(val),
            follow_up = Action::from_bits(val),
            full_hop = BoolFlag::from_bits(val),
            hitbox_vis = OnOff::from_val(val),
            input_delay = Some(val as i32),
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
            throw_state = ThrowOption::from_bits(val),
            throw_delay = MedDelay::from_bits(val),
            pummel_delay = MedDelay::from_bits(val),
            buff_state = BuffOption::from_bits(val),
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
