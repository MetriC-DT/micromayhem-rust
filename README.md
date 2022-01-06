# MicroMayhem
MicroMayhem is a shooting game inspired by the flash game *Gun Mayhem*.
Gun down your opponents from the platform, and avoid getting shot down yourself.

## Usage

## Explanation of File Hierarchy
- `resources` - resources for the project, such as fonts, spritesheets, images, etc.
- `src` - main entry point of the program, does the graphics drawing.
- `crates` - contains the different "subsystems" that make up the game. See more information below.

## The Different Crates:
### [Game](./crates/game/README.md)
Represents the game logic and physics. Runs independently of graphics.

### [Gui](./crates/gui/README.md)
Contains code to load GUI elements for game menu as well as in game sprites.

### [Network](./crates/network/README.md)
Netcode for the client and server. Currently requires massive amounts of refactoring
in order to include elements of security and data redundancy and checksums.

## Attributions
- Marta Nowaczyk - [Platform and Sprite Art](https://opengameart.org/users/aetherna)
- [GGEZ Game Engine](https://github.com/ggez/ggez)
