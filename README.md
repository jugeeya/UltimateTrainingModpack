# Ultimate Training Modpack Plugin

[![Github all releases](https://img.shields.io/github/downloads/jugeeya/UltimateTrainingModpack/total.svg)](https://GitHub.com/jugeeya/UltimateTrainingModpack/releases/)

A [Skyline](https://github.com/shadowninja108/Skyline) plugin using [cargo-skyline](https://github.com/jam1garner/cargo-skyline) for adding features to the training mode. It interfaces with a [Tesla](https://github.com/WerWolv/libtesla), a Switch custom overlay, for use as a menu to the features offered in training mode.

Built releases can be found [here](https://github.com/jugeeya/UltimateTrainingModpack/releases/).

- [Features](#features)
- [Build](#build)

<a name="features"/>

# Features
The features in this modpack are configured through the Tesla menu, which can be accessed at any time with by pressing `L+X+DPad Left`. This button configuration is fully configurable in the file `/config/tesla/config.ini`.

#### Save States
At any time in Training Mode, you can press `Grab + Down Taunt` to save the state of training mode. This will save the position, state, and damage of each fighter, which can then be reverted to at any time with `Grab + Up Taunt`. Use this instead of the built-in training mode reset!

#### Hitbox Visualization
Currently, hitboxes and grabboxes are supported. When visualization is active, other move effects are temporarily turned off for easier visualization.


##### Mash Toggles
*Note:* Combine this with the shield toggles to force the CPU to perform options OoS when their shield is damaged!

###### Airdodge
CPUs will mash airdodge on the first frame out of hitstun.

CPUs will also shield quickly if they are hit and remain grounded.

###### Jump
CPUs will mash jump on the first frame out of hitstun.

###### Attack
CPUs will mash an attack on the first frame out of hitstun and when landing. 
Attacks that can be chosen include:
- All aerials, followed by all specials

###### Random
CPUs will mash an aerial or grounded option on the first frame out of hitstun and when landing. 
The aerial options include:
- Airdodge, jump, all aerials, all specials

The grounded options include:
- Jump, jab, all tilts, all smashes, all specials, grab, spotdodge, and rolls

##### Ledge Option
CPUs will perform a random ledge option. 
Specific ledge options that can be chosen include:
- Normal, roll, jump, and attack

CPUs will also perform a defensive option after getting up.

##### Tech Option
CPUs will perform a random tech option. 
Specific tech options that can be chosen include:
- In place, roll, and miss tech

CPUs will also perform a defensive option after getting up.

##### Defensive Option
Choose the defensive option a CPU will perform after teching or getting up from the ledge. 
Specific options include:
    Flash shield, spotdodge, and jab

##### Shield

###### Infinite
CPUs will hold a shield that does not deteriorate over time or by damage.

###### Hold
CPUs will hold a normal shield.

#### Force CPU DI
##### All DI Toggles

##### Specified Direction
CPUs DI in the direction specified, relative to the player's facing position.

##### Random Direction
CPUs DI randomly in or away.

<a name="build"/>

# Build from Source

The overall process can be found in the [Github Actions specification file](https://github.com/jugeeya/UltimateTrainingModpack/blob/master/.github/workflows/rust.yml) as well.

## Prerequisites
- Rust environment with [cargo-skyline](https://github.com/jam1garner/cargo-skyline)
- [DEVKITPRO](https://devkitpro.org/wiki/Getting_Started) `switch-dev` installation 
- Built [Skyline](https://github.com/shadowninja108/Skyline), [Tesla nx-ovlloader and Tesla Menu](https://gbatemp.net/threads/tesla-the-nintendo-switch-overlay-menu.557362/), and [libnro_hook.nro](https://github.com/ultimate-research/nro-hook-plugin)

## Build steps
```bash
# clone the repository recursively
git clone --recursive 

# to build the training mod Skyline plugin
# resulting build is found in target/aarch64-skyline-switch/release/libtraining_modpack.nro
cargo skyline build --release

# to build the training mod Tesla overlay
# resulting build is ovlTrainingModpack.ovl
cd TrainingModpackOverlay && make
```
