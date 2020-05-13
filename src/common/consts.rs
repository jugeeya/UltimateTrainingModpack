pub const NONE: i32 = 0;

// Side Taunt

// DI
/*
 0, 0.785398, 1.570796, 2.356194, -3.14159, -2.356194,  -1.570796, -0.785398
 0, pi/4,     pi/2,     3pi/4,    pi,       5pi/4,      3pi/2,     7pi/4
*/

/* DI */
pub static mut DI_STATE: i32 = NONE;
pub const DI_RANDOM_IN_AWAY: i32 = 9;
// const std::vector<std::string> di_items{"None", "Out", "Up Out", "Up", "Up In", "In", "Down In", "Down", "Down Out", "Random"};

// Attack Option
pub const MASH_NAIR: i32 = 0;
pub const MASH_FAIR: i32 = 1;
pub const MASH_BAIR: i32 = 2;
pub const MASH_UPAIR: i32 = 3;
pub const MASH_DAIR: i32 = 4;
pub const MASH_NEUTRAL_B: i32 = 5;
pub const MASH_SIDE_B: i32 = 6;
pub const MASH_UP_B: i32 = 7;
pub const MASH_DOWN_B: i32 = 8;
pub const MASH_UP_SMASH: i32 = 9;
pub const MASH_GRAB: i32 = 10;
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
pub const TECH_MISS: i32 = 4;
// pub const std::vector<std::string> tech_items{"None", "Random", "In-Place", "Roll", "Miss Tech"};

// Mash States
pub const MASH_AIRDODGE: i32 = 1;
pub const MASH_JUMP: i32 = 2;
pub const MASH_ATTACK: i32 = 3;
pub const MASH_SPOTDODGE: i32 = 4;
pub const MASH_RANDOM: i32 = 5;
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
    pub HITBOX_VIS: bool,
    pub DI_STATE: i32,
    pub ATTACK_STATE: i32,
    pub LEDGE_STATE: i32,
    pub TECH_STATE: i32,
    pub MASH_STATE: i32,
    pub SHIELD_STATE: i32,
    pub DEFENSIVE_STATE: i32,
}
