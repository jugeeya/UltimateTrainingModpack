use core::f64::consts::PI;
use smash::lib::lua_const::*;

/// Hitbox Visualization
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HitboxVisualization {
    Off = 0,
    On = 1,
}

// DI
/*
 0, 0.785398, 1.570796, 2.356194, -3.14159, -2.356194,  -1.570796, -0.785398
 0, pi/4,     pi/2,     3pi/4,    pi,       5pi/4,      3pi/2,     7pi/4
*/

/// DI
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    None = 0,
    Right = 1,
    UpRight = 2,
    Up = 3,
    UpLeft = 4,
    Left = 5,
    DownLeft = 6,
    Down = 7,
    DownRight = 8,
    // lol what goes here jug smh my head
    Random = 9,
}

impl From<i32> for Direction {
    fn from(x: i32) -> Self {
        match x {
            0 => Direction::None,
            1 => Direction::Right,
            2 => Direction::UpRight,
            3 => Direction::Up,
            4 => Direction::UpLeft,
            5 => Direction::Left,
            6 => Direction::DownLeft,
            7 => Direction::Down,
            8 => Direction::DownRight,
            9 => Direction::Random,
            _ => panic!("Invalid direction {}", x),
        }
    }
}

//pub static FIGHTER_FACING_LEFT: f32 = 1.0;
pub static FIGHTER_FACING_RIGHT: f32 = -1.0;
pub static ANGLE_NONE: f64 = -69.0;
pub fn direction_to_angle(direction: Direction) -> f64 {
    match direction {
        Direction::None => ANGLE_NONE,
        Direction::Random => ANGLE_NONE, // Random Direction should be handled by the calling context
        _ => (direction as i32 - 1) as f64 * PI / 4.0,
    }
}

/// Mash Attack States
#[repr(i32)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Attack {
    Nair = 0,
    Fair = 1,
    Bair = 2,
    UpAir = 3,
    Dair = 4,
    NeutralB = 5,
    SideB = 6,
    UpB = 7,
    DownB = 8,
    UpSmash = 9,
    Grab = 10,
}

impl From<i32> for Attack {
    fn from(x: i32) -> Self {
        use Attack::*;

        match x {
            0 => Nair,
            1 => Fair,
            2 => Bair,
            3 => UpAir,
            4 => Dair,
            5 => NeutralB,
            6 => SideB,
            7 => UpB,
            8 => DownB,
            9 => UpSmash,
            10 => Grab,
            _ => panic!("Invalid mash attack state {}", x),
        }
    }
}

impl Attack {
    pub fn into_attack_air_kind(&self) -> Option<i32> {
        use Attack::*;

        Some(match self {
            Nair => *FIGHTER_COMMAND_ATTACK_AIR_KIND_N,
            Fair => *FIGHTER_COMMAND_ATTACK_AIR_KIND_F,
            Bair => *FIGHTER_COMMAND_ATTACK_AIR_KIND_B,
            Dair => *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW,
            UpAir => *FIGHTER_COMMAND_ATTACK_AIR_KIND_HI,
            _ => return None,
        })
    }
}

// Ledge Option
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LedgeOption {
    None = 0,
    Random = 1,
    Neutral = 2,
    Roll = 3,
    Jump = 4,
    Attack = 5,
}

impl From<i32> for LedgeOption {
    fn from(x: i32) -> Self {
        use LedgeOption::*;

        match x {
            0 => None,
            1 => Random,
            2 => Neutral,
            3 => Roll,
            4 => Jump,
            5 => Attack,
            _ => panic!("Invalid ledge option {}", x),
        }
    }
}

impl LedgeOption {
    pub fn into_status(&self) -> Option<i32> {
        Some(match self {
            LedgeOption::Neutral => *FIGHTER_STATUS_KIND_CLIFF_CLIMB,
            LedgeOption::Roll => *FIGHTER_STATUS_KIND_CLIFF_ESCAPE,
            LedgeOption::Jump => *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
            LedgeOption::Attack => *FIGHTER_STATUS_KIND_CLIFF_ATTACK,
            _ => return None,
        })
    }
}

// Tech Option
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TechOption {
    None = 0,
    Random = 1,
    InPlace = 2,
    Roll = 3,
    Miss = 4,
}

impl From<i32> for TechOption {
    fn from(x: i32) -> Self {
        use TechOption::*;

        match x {
            0 => None,
            1 => Random,
            2 => InPlace,
            3 => Roll,
            4 => Miss,
            _ => panic!("Invalid tech option {}", x),
        }
    }
}

/// Mash States
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mash {
    None = 0,
    Airdodge = 1,
    Jump = 2,
    Attack = 3,
    Spotdodge = 4,
    RollForward = 5,
    RollBack = 6,
    Random = 7,
}

impl From<i32> for Mash {
    fn from(x: i32) -> Self {
        match x {
            0 => Mash::None,
            1 => Mash::Airdodge,
            2 => Mash::Jump,
            3 => Mash::Attack,
            4 => Mash::Spotdodge,
            5 => Mash::RollForward,
            6 => Mash::RollBack,
            7 => Mash::Random,
            _ => panic!("Invalid mash state {}", x),
        }
    }
}

/// Shield States
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shield {
    None = 0,
    Infinite = 1,
    Hold = 2,
}

// Defensive States
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Defensive {
    None = 0,
    Random = 1,
    Spotdodge = 2,
    Roll = 3,
    Jab = 4,
    Shield = 5,
}

impl From<i32> for Defensive {
    fn from(x: i32) -> Self {
        use Defensive::*;

        match x {
            0 => None,
            1 => Random,
            2 => Spotdodge,
            3 => Roll,
            4 => Jab,
            5 => Shield,
            _ => panic!("Invalid mash state {}", x),
        }
    }
}

/// Mash in neutral
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MashInNeutral {
    Off = 0,
    On = 1,
}

#[repr(C)]
pub struct TrainingModpackMenu {
    pub hitbox_vis: HitboxVisualization,
    pub di_state: Direction,
    pub left_stick: Direction, // Currently only used for air dodge direction
    pub mash_attack_state: Attack,
    pub ledge_state: LedgeOption,
    pub tech_state: TechOption,
    pub mash_state: Mash,
    pub shield_state: Shield,
    pub defensive_state: Defensive,
    pub oos_offset: i32,
    pub mash_in_neutral: MashInNeutral,
}

// Fighter Ids
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FighterId {
    Player = 0,
    CPU = 1,
}
