#ifndef TAUNT_TOGGLES_H
#define TAUNT_TOGGLES_H

#define ARRAYSIZE(_ARR)          ((int)(sizeof(_ARR)/sizeof(*_ARR)))         // Size of a static C-style array. Don't use on pointers!

#define NONE 0

// Side Taunt

// DI
/*
 0, 0.785398, 1.570796, 2.356194, -3.14159, -2.356194,  -1.570796, -0.785398
 0, pi/4,     pi/2,     3pi/4,    pi,       5pi/4,      3pi/2,     7pi/4
*/

/* DI */
int DI_STATE = NONE;
#define DI_RANDOM_IN_AWAY 9
const char* di_items[] = { "None", "Away", "Up Away", "Up", "Up In", "In", "Down In", "Down", "Down Away", "Random In/Away"};

// Attack Option
#define MASH_NAIR 0
#define MASH_FAIR 1
#define MASH_BAIR 2
#define MASH_UPAIR 3
#define MASH_DAIR 4
#define MASH_NEUTRAL_B 5
#define MASH_SIDE_B 6
#define MASH_UP_B 7
#define MASH_DOWN_B 8
#define MASH_UP_SMASH 9
#define MASH_GRAB 10
const char* attack_items[] = { "Neutral Air", "Forward Air", "Back Air", "Up Air", "Down Air", "Neutral B", "Side B", "Up B", "Down B", "Up Smash", "Grab" };

// Ledge Option
#define RANDOM_LEDGE 1
#define NEUTRAL_LEDGE 2
#define ROLL_LEDGE 3
#define JUMP_LEDGE 4
#define ATTACK_LEDGE 5
const char* ledge_items[] = { "None", "Random", "Neutral Getup", "Roll", "Jump", "Attack" };

// Tech Option
#define RANDOM_TECH 1
#define TECH_IN_PLACE 2
#define TECH_ROLL 3
#define TECH_MISS 4
const char* tech_items[] = { "None", "Random", "In-Place", "Roll", "Miss Tech" };

// Mash States
#define MASH_AIRDODGE 1
#define MASH_JUMP 2
#define MASH_ATTACK 3
#define MASH_SPOTDODGE 4
#define MASH_RANDOM 5
const char* mash_items[] = { "None", "Airdodge", "Jump", "Attack", "Spotdodge", "Random" };

// Random Mash
const char* random_aerial_mash_items[] = { 
    "Airdodge",
    "Jump",
    "Fair",
    "Dair",
    "Bair",
    "Upair",
    "Nair",
    "Neutral Special",
    "Side Special",
    "Up Special",
    "Down Special"
};

const char* random_aerial_mash_cmd_strs[] = { 
    "FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE",
    "FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON",
    "FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N",
    "FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N",
    "FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N",
    "FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N",
    "FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N",
    "FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N",
    "FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S",
    "FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI",
    "FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW"
};

const char* random_ground_mash_items[] = {
    "Jump",
    "Jab",
    "Forward Tilt",
    "Up Tilt",
    "Down Tilt",
    "Forward Smash",
    "Up Smash",
    "Down Smash",
    "Neutral Special",
    "Side Special",
    "Up Special",
    "Down Special",
    "Grab",
    "Spotdodge",
    "Forward Roll",
    "Back Roll",
    "None"
};

const char* random_ground_mash_cmd_strs[] = {
    "FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON",
    "FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N",
    "FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3",
    "FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3",
    "FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3",
    "FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4",
    "FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4",
    "FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4",
    "FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI",
    "FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S",
    "FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI",
    "FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW",
    "FIGHTER_PAD_CMD_CAT1_FLAG_CATCH",
    "FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE",
    "FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_F",
    "FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_B"
};

// Shield States
#define SHIELD_INFINITE 1
#define SHIELD_HOLD 2
const char* shield_items[] = { "None", "Infinite", "Hold" };

// Defensive States
#define RANDOM_DEFENSIVE 1
#define DEFENSIVE_SPOTDODGE 2
#define DEFENSIVE_ROLL 3
#define DEFENSIVE_JAB 4
#define DEFENSIVE_SHIELD 5
const char* defensive_items[] = { "None", "Random", "Spotdodge", "Roll", "Jab", "Flash Shield" };

struct TrainingModpackMenu {
    bool HITBOX_VIS = 1;
    int DI_STATE = NONE;
    int ATTACK_STATE = MASH_NAIR;
    int LEDGE_STATE = RANDOM_LEDGE;
    int TECH_STATE = RANDOM_TECH;
    int MASH_STATE = NONE;
    int SHIELD_STATE = NONE;
    int DEFENSIVE_STATE = RANDOM_DEFENSIVE;
    int RANDOM_AERIAL_STATE[5] = {0,1,6,ARRAYSIZE(random_aerial_mash_items)-1,ARRAYSIZE(random_aerial_mash_items)-1};
    int RANDOM_GROUND_STATE[5] = {0,1,12,13,ARRAYSIZE(random_ground_mash_items)-1};
    char print_buffer[256];
    u64 print_buffer_len = 0;
} menu;

#endif // TAUNT_TOGGLES_H
