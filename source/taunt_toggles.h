#ifndef TAUNT_TOGGLES_H
#define TAUNT_TOGGLES_H

#define NONE 0

// Side Taunt

// DI
#define SET_DI 1
#define DI_RANDOM_IN_AWAY 2
#define NUM_DI_STATES 3

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
const char* ledge_items[] = { "Random", "Neutral Getup", "Roll", "Jump", "Attack" };

// Tech Option
#define RANDOM_TECH 1
#define TECH_IN_PLACE 2
#define TECH_ROLL 3
#define TECH_MISS 4
const char* tech_items[] = { "Random", "In-Place", "Roll", "Miss Tech" };

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

struct TrainingModpackMenu {
    bool HITBOX_VIS = 1;
    float DI_stick_x = 0;
    float DI_stick_y = 0;
    int DI_STATE = NONE;
    int ATTACK_STATE = MASH_NAIR;
    int LEDGE_STATE = RANDOM_LEDGE;
    int TECH_STATE = RANDOM_TECH;
    int MASH_STATE = NONE;
    int SHIELD_STATE = NONE;
    char print_buffer[256];
    u64 print_buffer_len = 0;
} menu;

#endif // TAUNT_TOGGLES_H
