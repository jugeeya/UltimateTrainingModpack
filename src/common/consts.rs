use crate::common::get_random_int;
use core::f64::consts::PI;
use smash::lib::lua_const::*;

extern crate num;

/// Hitbox Visualization
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HitboxVisualization {
    Off = 0,
    On = 1,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StageHazards {
    Off = 0,
    On = 1,
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
                    return 0;
                }
    
                return self.bits.trailing_zeros();
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

            pub fn to_toggle_strs() -> Vec<String> {
                let all_options = <$e>::all().to_vec();
                all_options.iter().map(|i| i.to_string()).collect()
            }

            pub fn to_toggle_vals() -> Vec<usize> {
                let all_options = <$e>::all().to_vec();
                all_options.iter().map(|i| i.bits() as usize).collect()
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
    }
}

impl Direction {
    pub fn into_angle(self) -> Option<f64> {
        let index = self.into_index();

        if index == 0 {
            return None;
        }

        Some((index as i32 - 1) as f64 * PI / 4.0)
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
            _ => 0,
        }
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
    }
}

impl LedgeOption {
    pub fn into_status(self) -> Option<i32> {
        Some(match self {
            LedgeOption::NEUTRAL => *FIGHTER_STATUS_KIND_CLIFF_CLIMB,
            LedgeOption::ROLL => *FIGHTER_STATUS_KIND_CLIFF_ESCAPE,
            LedgeOption::JUMP => *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
            LedgeOption::ATTACK => *FIGHTER_STATUS_KIND_CLIFF_ATTACK,
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

extra_bitflag_impls! {MissTechFlags}

/// Shield States
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive)]
pub enum Shield {
    None = 0,
    Infinite = 1,
    Hold = 2,
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

extra_bitflag_impls! {Defensive}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OnOff {
    Off = 0,
    On = 1,
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
}

extra_bitflag_impls! {Action}

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

extra_bitflag_impls! {Delay}

bitflags! {
    pub struct BoolFlag : u32 {
        const TRUE = 0x1;
        const FALSE = 0x2;
    }
}

extra_bitflag_impls! {BoolFlag}

impl BoolFlag {
    pub fn into_bool(self) -> bool {
        match self {
            BoolFlag::TRUE => true,
            _ => false,
        }
    }
}

#[repr(C)]
pub struct TrainingModpackMenu {
    pub hitbox_vis: HitboxVisualization,
    pub stage_hazards: StageHazards,
    pub di_state: Direction,
    pub sdi_state: Direction,
    pub air_dodge_dir: Direction,
    pub mash_state: Action,
    pub follow_up: Action,
    pub ledge_state: LedgeOption,
    pub ledge_delay: Delay,
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
}

macro_rules! set_by_str {
    ($obj:ident, $s:ident, $(($field:ident = $rhs:expr))*) => {
        $(
            if $s == stringify!($field) {
                $obj.$field = $rhs.unwrap();
            }
        )*
    }
}

impl TrainingModpackMenu {
    pub fn set(&mut self, s: &str, val: u32) {
        set_by_str!(self, s,
            (di_state = Direction::from_bits(val))
            (sdi_state = Direction::from_bits(val))
            (air_dodge_dir = Direction::from_bits(val))

            (mash_state = Action::from_bits(val))
            (follow_up = Action::from_bits(val))

            (ledge_state = LedgeOption::from_bits(val))
            (ledge_delay = Delay::from_bits(val))
            (tech_state = TechFlags::from_bits(val))
            (miss_tech_state = MissTechFlags::from_bits(val))
            
            (shield_state = num::FromPrimitive::from_u32(val))
            (defensive_state = Defensive::from_bits(val))
            (oos_offset = Delay::from_bits(val))
            (reaction_time = Delay::from_bits(val))

            (fast_fall = BoolFlag::from_bits(val))
            (fast_fall_delay = Delay::from_bits(val))
            (falling_aerials = BoolFlag::from_bits(val))
            (aerial_delay = Delay::from_bits(val))
            (full_hop = BoolFlag::from_bits(val))
        );

        // TODO
        // pub hitbox_vis: HitboxVisualization,
        // pub stage_hazards: StageHazards,
    }
}

// Fighter Ids
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FighterId {
    Player = 0,
    CPU = 1,
}
