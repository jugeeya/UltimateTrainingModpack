# Ultimate Training Modpack Plugin

[![GitHub All Releases](https://img.shields.io/github/downloads/jugeeya/UltimateTrainingModpack/total?logo=download&style=for-the-badge)](https://github.com/jugeeya/UltimateTrainingModpack/releases)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/jugeeya/UltimateTrainingModpack/Rust?style=for-the-badge)](https://github.com/jugeeya/UltimateTrainingModpack/actions)
[![Discord](https://img.shields.io/discord/407970595418931200?label=discord&logo=discord&style=for-the-badge)](https://discord.gg/qU4TBwV)
[![Twitter Follow](https://img.shields.io/twitter/follow/jugeeya?color=brightgreen&logo=twitter&style=for-the-badge)](https://twitter.com/jugeeya)

A [Skyline](https://github.com/shadowninja108/Skyline) plugin using [cargo-skyline](https://github.com/jam1garner/cargo-skyline) for adding features to the training mode. It interfaces with a [Tesla](https://github.com/WerWolv/libtesla), a Switch custom overlay, for use as a menu to the features offered in training mode.

Built releases can be found [here](https://github.com/jugeeya/UltimateTrainingModpack/releases/).

- [Features](#features)
- [Build](#build)

<a name="features"/>

# Features
The features in this modpack are configured through the Tesla menu, which can be accessed at any time with by pressing `L+X+DPad Left`. This button configuration is fully configurable in the file `/config/tesla/config.ini`.
[<img src="https://i.imgur.com/HRKvIb3.jpg">](https://i.imgur.com/HRKvIb3.jpg)
[<img src="https://i.imgur.com/eSrtDyj.png">](https://i.imgur.com/eSrtDyj.png)
[<img src="https://i.imgur.com/7Cd6utU.jpg">](https://i.imgur.com/7Cd6utU.jpg)

#### Frame Advantage. 
*Currently only works on shield*. Practice moves on shield to find out the frame advantage of the moves performed. Best used with Infinite Shield.

#### Save States
At any time in Training Mode, you can press `Grab + Down Taunt` to save the state of training mode. This will save the position, state, and damage of each fighter, which can then be reverted to at any time with `Grab + Up Taunt`. Use this instead of the built-in training mode reset!

#### Hitbox Visualization
Currently, hitboxes and grabboxes are supported. When visualization is active, other move effects are temporarily turned off for easier visualization.

#### Selecting Multiple Options
Any submenu that allows you to toggle multiple options will randomize between only those options. This is the vast majority of items in the menu detailed below, and it's a huge change that allows for really deep practice.


##### Mash Section
###### Mash Toggles
*Note:* Combine this with the shield toggles to force the CPU to perform options OoS when their shield is damaged!

CPUs will mash an option on the first frame possible out of hitstun.

Airdodge has specific logic that the CPU will also flash shield when landing.

###### Followup Toggles
Set a mash option to perform directly after the one specified with Mash Toggles.

###### Mash in Neutral
Set a CPU to mash specified option in neutral/idle state.

##### Left Stick Section
###### DI
CPUs DI in the direction specified, relative to the player's facing position.

###### SDI
Works the same way as the DI toggle, but choose a direction for the CPU to SDI every 4 frames of hitlag. 

###### Airdodge Direction
When a CPU is set to mash airdodge, it will use this direction as its airdodge direction.

##### Chase Section
###### Ledge Option
CPUs will perform a random ledge option among the selected options.

CPUs will also perform a defensive option after getting up.

###### Tech Option
CPUs will perform a random tech option among the selected options.

CPUs will also perform a defensive option after getting up.

###### Defensive Option
CPUs will perform the defensive option a CPU will perform after teching or getting up from the ledge, among the selected options.

##### Shield Section

###### Shield Options
- Infinite: CPUs will hold a shield that does not deteriorate over time or by damage.
- Hold: CPUs will hold a shield that does not deteriorate over time until hit for the first time.

###### OOS Offset
The CPU will delay until the specified number of hits to perform an OoS option.

###### OOS Reaction Time
The CPU will delay a specified number of frames before performing an OoS option.

#### Aerials Section
Edit how the CPU performs aerials.

##### Fast Fall
##### Full Hops 
##### Falling Aerials
##### Fast Fall Delay 
Specified in frames (from apex of CPU's jump).

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
