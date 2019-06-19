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
#define INFINITE_SHIELD 3
#define HOLD_SHIELD 4
#define LEDGE_OPTION 5

int TOGGLE_STATE = NONE;
#define NUM_TOGGLE_STATES 6

#endif // TAUNT_TOGGLES_H
