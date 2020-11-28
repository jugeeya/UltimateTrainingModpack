use crate::common::get_random_int;
use core::f64::consts::PI;
use smash::lib::lua_const::*;

// bitflag helper function macro
macro_rules! to_vec_impl {
    ($e:ty) => {
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
    }
}

macro_rules! to_index_impl {
    ($e:ty) => {
        pub fn to_index(&self) -> u32 {
            if self.bits == 0 {
                return 0;
            }

            return self.bits.trailing_zeros();
        }
    };
}

pub fn random_option<T>(arg: &[T]) -> &T {
    &arg[get_random_int(arg.len() as i32) as usize]
}

macro_rules! get_random_impl {
    ($e:ty) => {
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
    };
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
    to_vec_impl! {Direction}
    get_random_impl! {Direction}
}

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
    to_vec_impl! {LedgeOption}
    get_random_impl! {LedgeOption}
}

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
    to_vec_impl! {TechFlags}
    get_random_impl! {TechFlags}
}

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
    to_vec_impl! {MissTechFlags}
    get_random_impl! {MissTechFlags}
}

/// Shield States
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shield {
    None = 0,
    Infinite = 1,
    Hold = 2,
    Constant = 3,
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
    to_vec_impl! {Defensive}
    get_random_impl! {Defensive}
}

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
    to_vec_impl! {Action}
    get_random_impl! {Action}
}

bitflags! {
    pub struct AttackAngle : u32 {
        const NEUTRAL = 0x1;
        const UP = 0x2;
        const DOWN = 0x4;
    }
}

impl AttackAngle {
    to_vec_impl! {AttackAngle}
    get_random_impl! {AttackAngle}
}


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

impl Delay {
    to_vec_impl! {Delay}
    get_random_impl! {Delay}
    to_index_impl! {Delay}
}

bitflags! {
    pub struct BoolFlag : u32 {
        const TRUE = 0x1;
        const FALSE = 0x2;
    }
}

impl BoolFlag {
    to_vec_impl! {BoolFlag}
    get_random_impl! {BoolFlag}

    pub fn into_bool(self) -> bool {
        match self {
            BoolFlag::TRUE => true,
            _ => false,
        }
    }
}

#[repr(C)]
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
    pub ledge_delay: Delay,
    pub tech_state: TechFlags,
    pub miss_tech_state: MissTechFlags,
    pub shield_state: Shield,
    pub player_shield: Shield,
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
}

// Fighter Ids
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FighterId {
    Player = 0,
    CPU = 1,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
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
}