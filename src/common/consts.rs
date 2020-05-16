use smash::lib::lua_const::*;

pub const NONE: i32 = 0;

// Side Taunt

// DI
/*
 0, 0.785398, 1.570796, 2.356194, -3.14159, -2.356194,  -1.570796, -0.785398
 0, pi/4,     pi/2,     3pi/4,    pi,       5pi/4,      3pi/2,     7pi/4
*/

/* DI */
pub const DI_RANDOM_IN_AWAY: i32 = 9;
// const std::vector<std::string> di_items{"None", "Out", "Up Out", "Up", "Up In", "In", "Down In", "Down", "Down Out", "Random"};

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
    Grab = 10
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
            _ => panic!("Invalid mash attack state {}", x)
        }
    }
}

impl Attack {
    pub fn into_attack_air_kind(&self) -> Option<i32> {
        use Attack::*;

        Some(
            match self {
                Nair => *FIGHTER_COMMAND_ATTACK_AIR_KIND_N,
                Fair => *FIGHTER_COMMAND_ATTACK_AIR_KIND_F,
                Bair => *FIGHTER_COMMAND_ATTACK_AIR_KIND_B,
                Dair => *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW,
                UpAir => *FIGHTER_COMMAND_ATTACK_AIR_KIND_HI,
                _ => return None,
            }
        )
    }
}

// pub const std::vector<std::string> attack_items{"Neutral Air", "Forward Air", "Back Air", "Up Air", "Down Air", "Neutral B", "Side B", "Up B", "Down B", "Up Smash", "Grab"};

// Ledge Option
pub const RANDOM_LEDGE: i32 = 1;
pub const NEUTRAL_LEDGE: i32 = 2;
pub const ROLL_LEDGE: i32 = 3;
pub const JUMP_LEDGE: i32 = 4;
pub const ATTACK_LEDGE: i32 = 5;
// pub const std::vector<std::string> ledge_items{"None", "Random", "Ntrl. Getup", "Roll", "Jump", "Attack"};

// Tech Option
pub const RANDOM_TECH: i32 = 1;
pub const TECH_IN_PLACE: i32 = 2;
pub const TECH_ROLL: i32 = 3;
#[allow(dead_code)]
pub const TECH_MISS: i32 = 4;
// pub const std::vector<std::string> tech_items{"None", "Random", "In-Place", "Roll", "Miss Tech"};

/// Mash States
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mash {
    None = 0,
    Airdodge = 1,
    Jump = 2,
    Attack = 3,
    Spotdodge = 4,
    Random = 5
}

impl From<i32> for Mash {
    fn from(x: i32) -> Self {
        match x {
            0 => Mash::None,
            1 => Mash::Airdodge,
            2 => Mash::Jump,
            3 => Mash::Attack,
            4 => Mash::Spotdodge,
            5 => Mash::Random,
            _ => panic!("Invalid mash state {}", x)
        }
    }
}

// pub const std::vector<std::string> mash_items{"None", "Airdodge", "Jump", "Attack", "Spotdodge", "Random"};

// Shield States
pub const SHIELD_INFINITE: i32 = 1;
pub const SHIELD_HOLD: i32 = 2;
// pub const std::vector<std::string> shield_items{"None", "Infinite", "Hold"};

// Defensive States
pub const RANDOM_DEFENSIVE: i32 = 1;
pub const DEFENSIVE_SPOTDODGE: i32 = 2;
pub const DEFENSIVE_ROLL: i32 = 3;
pub const DEFENSIVE_JAB: i32 = 4;
pub const DEFENSIVE_SHIELD: i32 = 5;
// pub const std::vector<std::string> defensive_items{"None", "Random", "Spotdodge", "Roll", "Jab", "Flash Shield"};

#[repr(C)]
pub struct TrainingModpackMenu {
    pub hitbox_vis: bool,
    pub di_state: i32,
    pub mash_attack_state: Attack,
    pub ledge_state: i32,
    pub tech_state: i32,
    pub mash_state: Mash,
    pub shield_state: i32,
    pub defensive_state: i32,
}
