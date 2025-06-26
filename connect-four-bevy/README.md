# Connect Four - Bevy Game Engine

A classic Connect Four game implemented in Rust using the Bevy game engine.

## Features

- **Visual Game Board**: Interactive 7x6 grid with animated piece drops
- **Improved Layout**: Board positioned below UI elements for better visual hierarchy
- **Two Player Gameplay**: Red vs Yellow players take turns
- **Win Detection**: Automatic detection of horizontal, vertical, and diagonal wins
- **Score Tracking**: Keeps track of wins and draws across multiple games
- **Smooth Animations**: Bouncing piece drop animations
- **Multiple Input Methods**: Mouse clicks and keyboard shortcuts
- **Game Reset**: Start a new game at any time

## How to Play

### Objective
Connect four of your pieces in a row (horizontally, vertically, or diagonally) before your opponent does.

### Controls

#### Mouse Controls
- **Click on a column**: Drop your piece in that column
- **Hover over columns**: See column highlights when valid
- **Click "New Game" button**: Reset the game

#### Keyboard Controls
- **Keys 1-7**: Drop piece in columns 1-7 respectively
- **R key**: Reset the game

### Game Rules
1. Players alternate turns (Red goes first)
2. Pieces fall to the lowest available position in the selected column
3. A player wins by connecting four pieces in a row:
   - Horizontally (←→)
   - Vertically (↑↓)
   - Diagonally (↗↙ or ↖↘)
4. The game ends in a draw if all 42 spaces are filled without a winner

## Installation & Running

### Prerequisites
- Rust (latest stable version)
- Cargo (comes with Rust)

### Setup
1. Clone or download this repository
2. Navigate to the project directory:
   ```bash
   cd connect-four-bevy
   ```

3. Run the game:
   ```bash
   cargo run
   ```

### Development
For development with faster compile times, use the dev profile:
```bash
cargo run --bin connect_four
```

For optimized release build:
```bash
cargo run --release
```

## Game Architecture

The game is structured into several modules:

- **`main.rs`**: Entry point and system setup
- **`game_logic.rs`**: Core game rules, win detection, and state management
- **`board.rs`**: Visual board representation, input handling, and animations
- **`ui.rs`**: User interface, scoring, and game status display
- **`events.rs`**: Custom events for game interactions

### Key Components

- **GameState**: Tracks board state, current player, and game status
- **GameScore**: Maintains win/draw statistics across games
- **AnimatingPiece**: Handles smooth piece drop animations
- **BoardCell**: Represents individual board positions
- **GamePiece**: Static pieces on the board

## Technical Details

- **Engine**: Bevy 0.12
- **Language**: Rust (2021 edition)
- **Graphics**: 2D sprites with custom colors and improved spacing
- **Animation**: Custom easing functions for piece drops
- **Input**: Mouse and keyboard support
- **Layout**: Optimized UI positioning for better user experience

## Testing

Run the included tests to verify game logic:
```bash
cargo test
```

The tests cover:
- Game initialization
- Piece dropping mechanics
- Column full detection
- Win condition detection (horizontal, vertical, diagonal)

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