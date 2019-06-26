#ifndef TAUNT_TOGGLES_H
#define TAUNT_TOGGLES_H

#define NONE 0

/* Up Taunt */
bool HITBOX_VIS = 1;

/* Side Taunt */

/* DI */
float DI_stick_x = 0;
float DI_stick_y = 0;
int DI_STATE = NONE;
#define SET_DI 1
#define DI_RANDOM_IN_AWAY 2
#define NUM_DI_STATES 3

/* Attack Option */
#define MASH_NAIR 0
#define MASH_FAIR 1
#define MASH_BAIR 2
#define MASH_UPAIR 3
#define MASH_DAIR 4
#define MASH_NEUTRAL_B 5
#define MASH_SIDE_B 6
#define MASH_UP_B 7
#define MASH_DOWN_B 8

int ATTACK_STATE = MASH_NAIR;
#define NUM_ATTACK_STATES 9

/* Ledge Option */
#define RANDOM_LEDGE 0
#define NEUTRAL_LEDGE 1
#define ROLL_LEDGE 2
#define JUMP_LEDGE 3
#define ATTACK_LEDGE 4

int LEDGE_STATE = RANDOM_LEDGE;
#define NUM_LEDGE_STATES 5

// Down Taunt
#define MASH_TOGGLES 0
#define ESCAPE_TOGGLES 1
#define SHIELD_TOGGLES 2
int TOGGLE_STATE = MASH_TOGGLES;
#define NUM_TOGGLE_STATES 3

// Mash States
#define MASH_AIRDODGE 1
#define MASH_JUMP 2
#define MASH_RANDOM 3
int MASH_STATE = NONE;
#define NUM_MASH_STATES 4
#define MASH_ATTACK 4 // unused for now

// Escape States
#define ESCAPE_LEDGE 1
int ESCAPE_STATE = ESCAPE_LEDGE;
#define NUM_ESCAPE_STATES 2

// Shield States
#define SHIELD_INFINITE 1
#define SHIELD_HOLD 2
int SHIELD_STATE = NONE;
#define NUM_SHIELD_STATES 3



#endif // TAUNT_TOGGLES_H
