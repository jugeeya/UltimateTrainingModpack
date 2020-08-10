#pragma once

#define NONE 0

#include <vector>

#include "cpp_utils.hpp"

const std::vector<std::string> on_off{"Off", "On"};

// Frame Advantage
const std::vector<std::string> frame_advantage_items{""};
const std::string              frame_advantage_help = R""""(
TODO)"""";

// Side Taunt

// DI / Left Stick
/*
 0, 0.785398, 1.570796, 2.356194, -3.14159, -2.356194,  -1.570796, -0.785398
 0, pi/4,     pi/2,     3pi/4,    pi,       5pi/4,      3pi/2,     7pi/4
*/

/* DI */
#define DI_RANDOM_IN_AWAY 9
const std::vector<std::string> di_items{"None", "Out", "Up Out", "Up", "Up In", "In", "Down In", "Down", "Down Out", "Random"};
const std::string              di_help = R""""(
Specified Direction
CPUs DI in the direction specified
(relative to the player's facing
position).

Random Direction
CPUs DI randomly in or away.)"""";

// Left Stick
const std::string left_stick_help = R""""(
Specified Direction
CPUs left stick will be
in the direction specified
(relative to the player's facing
position).

Currently only used for
- air dodge

)"""";

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
const std::vector<std::string> attack_items{"Neutral Air",
                                            "Forward Air",
                                            "Back Air",
                                            "Up Air",
                                            "Down Air",
                                            "Neutral B",
                                            "Side B",
                                            "Up B",
                                            "Down B",
                                            "Up Smash",
                                            "F Smash",
                                            "D Smash",
                                            "Grab",
                                            "Jab",
                                            "Ftilt",
                                            "Utilt",
                                            "Dtilt",
                                            "Dash Attack"};
const std::string              attack_help = R""""(
Only active when Mash Toggle is
set to Attack.
)"""";

// Ledge Option
// clang-format off
#define ENUM_CLASS_LedgeFlag(type,x) \
    x(type,Neutral,"Neutral") \
    x(type,Roll,"Roll") \
    x(type,Jump,"Jump") \
    x(type,Attack,"Attack")

// clang-format on

DEFINE_ENUM_CLASS(LedgeFlag);
const std::string ledge_help = R""""(
CPUs will perform a ledge option.

Specific ledge options can be
chosen and include:
    Normal, roll, jump, and attack

CPUs will also perform a defensive
option after getting up.
)"""";

// Tech Option

// clang-format off
#define ENUM_CLASS_TechFlag(type,x) \
	x(type,Miss,"Miss Tech") \
	x(type,Roll,"Roll") \
	x(type,InPlace,"In Place")

// clang-format on
DEFINE_ENUM_CLASS(TechFlag);

constexpr const char* const tech_help = R""""(
CPUs will perform a random
tech option.

Specific tech options can be chosen and include:
    In place, roll, and miss tech

CPUs will also perform a defensive
option after getting up.)"""";

// Mash States
#define MASH_AIRDODGE 1
#define MASH_JUMP 2
#define MASH_ATTACK 3
#define MASH_SPOTDODGE 4
#define MASH_ROLL_F 5
#define MASH_ROLL_B 6
#define MASH_RANDOM 7
const std::vector<std::string> mash_items{"None", "Airdodge", "Jump", "Attack", "Spotdodge", "Roll F", "Roll B", "Random"};
const std::string              mash_help = R""""(
Use this toggle along with the Shield
Options toggle to practice moves on
shield.

CPUs will mash on the first frame out
of hitstun, out of specific states.

Airdodge
- Hitstun
CPUs will also shield quickly if they
are hit and remain grounded.

Jump
- Hitstun, shieldstun

Attack
- Hitstun, shieldstun, landing.

Spotdodge
- Hitstun, shieldstun, landing.

Random
- Hitstun, shieldstun, landing.)"""";

// Action items (Follow Up only atm)
const std::vector<std::string> action_items{"None",        "Airdodge",    "Jump",     "Spotdodge", "Roll F",   "Roll B",
                                            "Neutral Air", "Forward Air", "Back Air", "Up Air",    "Down Air", "Neutral B",
                                            "Side B",      "Up B",        "Down B",   "Up Smash",  "F Smash",  "D Smash",
                                            "Grab",        "Jab",         "Filt",     "Utilt",     "Dtilt",    "Dash Attack"};
const std::string              follow_up_help = R""""(
Action to buffer
after the first mash option
)"""";

// Shield States
#define SHIELD_INFINITE 1
#define SHIELD_HOLD 2
const std::vector<std::string> shield_items{"None", "Infinite", "Hold"};
const std::string              shield_help = R""""(
Use these toggles in conjunction
with Mash toggles to practice
moves on shield.

Infinite
CPUs will hold a shield that does
not deteriorate over time or
by damage.

Hold
CPUs will hold a normal shield.)"""";

// Defensive States
#define RANDOM_DEFENSIVE 1
#define DEFENSIVE_SPOTDODGE 2
#define DEFENSIVE_ROLL 3
#define DEFENSIVE_JAB 4
#define DEFENSIVE_SHIELD 5
const std::vector<std::string> defensive_items{"None", "Random", "Spotdodge", "Roll", "Jab", "Flash Shield"};
const std::string              defensive_help = R""""(
Choose the defensive option a CPU
will perform after teching or
getting up from the ledge.

Specific options include:
    Flash shield, spotdodge, and jab
)"""";

// Hitbox visualization
const std::string hitbox_help = R""""(
Currently, hitboxes and
grabboxes are supported.

Original move effects are
paused during normal attacks
and specials when hitbox
visualization is active.)"""";

// Save states
const std::vector<std::string> save_state_items{""};
const std::string              save_states_help = R""""(
Press Grab + Down Taunt at any
time to save the state of the
training mode for you and the
CPU.

Press Grab + Up Taunt at any
time to revert to a
previously saved state.

The following attributes
are saved:
- Percent
- Position
- Facing direction)"""";

// OOS
const std::vector<std::string> number_list{
    "0",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
};
const std::string oos_help = R""""(
Option to delay oos options
until a certain number of hits
have connected.

Consecutive hits that keep the
CPU locked in shield stun
between hits will count
as a single hit.)"""";

const std::string reaction_time_help = R""""(
Additional reaction time
in frames

Used to delay OOS Options.)"""";

// Mash in neutral
const std::string mash_neutral_help = R""""(
Force mash options to
always occur, not just
out of specific states.)"""";

const std::vector<std::string> number_list_big{
    "0",  "1",  "2",  "3",  "4",  "5",  "6",  "7",  "8",  "9",  "10", "11", "12", "13", "14",
    "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29",
};
