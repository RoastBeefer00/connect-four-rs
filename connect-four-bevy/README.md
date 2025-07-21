# Connect Four - Bevy Game Engine

A classic Connect Four game implemented in Rust using the Bevy game engine.

## Features

- **Visual Game Board**: Interactive 7x6 grid with animated piece drops
- **Improved Layout**: Board positioned below UI elements for better visual hierarchy
- **Two Player Gameplay**: Red vs Yellow players take turns
- **Spectators**: Spectators can watch the game
- **Score Tracking**: Keeps track of wins and draws across multiple games
- **Smooth Animations**: Bouncing piece drop animations
- **Game Reset**: Start a new game at any time

## How to Play

### Objective
Connect four of your pieces in a row (horizontally, vertically, or diagonally) before your opponent does.

### Controls

#### Mouse Controls
- **Click on a column**: Drop your piece in that column
- **Hover over columns**: See column highlights when valid
- **Click "New Game" button**: Reset the game

### Game Rules
1. Players alternate turns (Red goes first)
2. Pieces fall to the lowest available position in the selected column
3. A player wins by connecting four pieces in a row:
   - Horizontally (←→)
   - Vertically (↑↓)
   - Diagonally (↗↙ or ↖↘)
4. The game ends in a draw if all 42 spaces are filled without a winner

## Installation & Running

### Normal
### Prerequisites
- Rust (latest stable version)
- Cargo (comes with Rust)

### Nix Prerequisites
Install [Nix](https://nixos.org/download/), [direnv](https://direnv.net/), and [devenv](https://devenv.sh/) (direnv and devenv are install through Nix).  This will make it so all development and running dependencies are automatically loaded when entering this directory.

Why Nix?  Using Nix allows ALL developers on the project to use the EXACT same development environment!  This leaves no "it works on my machine" moments.

### Setup
1. Clone or download this repository
2. Navigate to the project directory:
   ```bash
   cd connect-four-bevy
   ```

3. Run the game locally:
   ```bash
   cargo run
   ```
4. Run the game targetted for the web:
   ```bash
   trunk serve
   ```
5. Run the game targettedd for the web (Nix):
   ```bash
   web-run
   ```

### Development
To build the local version of the game:
```bash
cargo build --release
```

To build the web version of the game:
```bash
trunk build
```
Then find the `.wasm` file in the `dist` folder and compress it:
```bash
gzip -9 wasm-file-name.wasm
```

Or to build for the web with Nix:
```bash
web-build
```

Once the game is built for the web, it will need to be copied into the server's files for static distibution:
```bash
cp ./dist/* ../connect-four-server/dist
```

## Game Architecture

The game is structured into several modules:

- **`main.rs`**: Entry point and system setup
- **`game_logic.rs`**: Core game rules, win detection, and state management
- **`board.rs`**: Visual board representation, input handling, and animations
- **`ui.rs`**: User interface, scoring, and game status display
- **`events.rs`**: Custom events for game interactions
- **`socket.rs`**: Web socket logic to communicate with game server

## Contributing

Feel free to contribute improvements, bug fixes, or new features:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is open source and available under the MIT License.

## Acknowledgments

- Built with the excellent [Bevy Game Engine](https://bevyengine.org/)
- Inspired by the classic Connect Four board game
