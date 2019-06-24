# Ultimate Training Modpack Plugin

[![Github all releases](https://img.shields.io/github/downloads/jugeeya/UltimateTrainingModpack/total.svg)](https://GitHub.com/jugeeya/UltimateTrainingModpack/releases/)

A [SaltyNX](https://github.com/shinyquagsire23/SaltyNX) plugin for adding features to the training mode. 

Built releases can be found [here](https://github.com/jugeeya/UltimateTrainingModpack/releases/).

- [Features](#features)
- [Build](#build)

<a name="features"/>

# Features
Toggles are changed with taunts, with the following functionality:
- **Up taunt**: Hitbox Visualization
- **Down taunt**: Force CPU Options
- **Side taunt**: Force CPU DI

#### Hitbox Visualization
Hitbox visualization is toggled on or off with up taunt. Currently, hitboxes and grabboxes are supported.

#### Force CPU Options
##### Mash airdodge
CPUs will mash airdodge on the first frame out of hitstun.

CPUs will also shield quickly if they are hit and remain grounded.

##### Mash jump
CPUs will mash jump on the first frame out of hitstun.

##### Mash attack
CPUs will mash an attack on the first frame out of hitstun and when landing. 
Attacks can be chosen with side taunt while on this toggle, and include:
- All aerials, followed by all specials

##### Mash random
CPUs will mash an aerial or grounded option on the first frame out of hitstun and when landing. 
The aerial options include:
- Airdodge, jump, all aerials, all specials

The grounded options include:
- Jump, jab, all tilts, all smashes, all specials, grab, spotdodge, and rolls

##### Infinite shield
CPUs will hold a shield that does not deteriorate over time or by damage.

##### Hold shield
CPUs will hold a normal shield.

##### Ledge Option
CPUs will perform a random ledge option. 
Specific ledge options can be chosen with side taunt while this toggle is active, and include:
- Normal, roll, jump, and attack getups

#### Force CPU DI
##### All DI Toggles

##### Specified Direction
CPUs DI in the direction specified by the taunt text.

##### Random Direction
CPUs DI randomly in or away.

<a name="build"/>

# Build from Source

Requires [DEVKITPRO](https://devkitpro.org/wiki/Getting_Started) in path.

```sh
   git clone https://github.com/jugeeya/UltimateTrainingModpack.git
   cd UltimateTrainingModpack/
   make
```


