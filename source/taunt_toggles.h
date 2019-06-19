#ifndef TAUNT_TOGGLES_H
#define TAUNT_TOGGLES_H

#define NONE 0

/* Up Taunt */
bool HITBOX_VIS = 1;

/* Side Taunt
 0, 0.785398, 1.570796, 2.356194, -3.14159, -2.356194,  -1.570796, -0.785398
 0, pi/4,     pi/2,     3pi/4,    pi,       5pi/4,      3pi/2,     7pi/4
*/

/* DI */
int DI_STATE = NONE;
#define DI_RANDOM_IN_AWAY 9
#define NUM_DI_STATES 10

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
#define MASH_AIRDODGE 1
#define MASH_JUMP 2
#define MASH_ATTACK 3
#define INFINITE_SHIELD 4
#define HOLD_SHIELD 5
#define LEDGE_OPTION 6

int TOGGLE_STATE = NONE;
#define NUM_TOGGLE_STATES 7

#endif // TAUNT_TOGGLES_H
