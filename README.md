# Ultimate Training Modpack Plugin

[![Github all releases](https://img.shields.io/github/downloads/jugeeya/UltimateTrainingModpack/total.svg)](https://GitHub.com/jugeeya/UltimateTrainingModpack/releases/)

A [SaltyNX](https://github.com/shinyquagsire23/SaltyNX) plugin for adding features to the training mode. It interfaces with a fork of [Layoff](https://github.com/crc-32/layoff), a Switch custom overlay, for use as a menu to the features offered in training mode.

Built releases can be found [here](https://github.com/jugeeya/UltimateTrainingModpack/releases/).

- [Features](#features)
- [Build](#build)

<a name="features"/>

# Features
The features in this modpack are configured through the Layoff menu, which can be accessed at any time with by long pressing the Home button on a right Joy-con or Switch Pro controller. 

#### Hitbox Visualization
Currently, hitboxes and grabboxes are supported.


##### Mash Toggles
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
CPUs DI randomly in or away when the player presses left taunt.

<a name="build"/>

# Build from Source

Requires [DEVKITPRO](https://devkitpro.org/wiki/Getting_Started) in path.

```sh
# building the modpack ELF itself
git clone --recursive https://github.com/jugeeya/UltimateTrainingModpack.git
cd UltimateTrainingModpack/
make
# building the Layoff menu
cd layoff/libnx
make
cd ..
make
# make_layeredfs.bat for windows
./make_layeredfs.sh 
```


