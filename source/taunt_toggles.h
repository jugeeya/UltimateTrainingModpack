#ifndef TAUNT_TOGGLES_H
#define TAUNT_TOGGLES_H

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
const char* di_items[] = { "None", "Away", "Up Away", "Up", "Up In", "In", "Down In", "Down", "Down In", "Random In/Away"};

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
const char* attack_items[] = { "Neutral Air", "Forward Air", "Back Air", "Up Air", "Down Air", "Neutral B", "Side B", "Up B", "Down B" };

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
#define MASH_RANDOM 4
const char* mash_items[] = { "None", "Airdodge", "Jump", "Attack", "Random" };

// Shield States
#define SHIELD_INFINITE 1
#define SHIELD_HOLD 2
const char* shield_items[] = { "None", "Infinite", "Hold" };

// Defensive States
#define RANDOM_DEFENSIVE 1
#define DEFENSIVE_SHIELD 2
#define DEFENSIVE_SPOTDODGE 3
#define DEFENSIVE_JAB 4
const char* defensive_items[] = { "None", "Random", "Flash Shield", "Spotdodge", "Jab" };

struct TrainingModpackMenu {
    bool HITBOX_VIS = 1;
    int DI_STATE = NONE;
    int ATTACK_STATE = MASH_NAIR;
    int LEDGE_STATE = RANDOM_LEDGE;
    int TECH_STATE = RANDOM_TECH;
    int MASH_STATE = NONE;
    int SHIELD_STATE = NONE;
    int DEFENSIVE_STATE = RANDOM_DEFENSIVE;
    char print_buffer[256];
    u64 print_buffer_len = 0;
} menu;

#endif // TAUNT_TOGGLES_H
