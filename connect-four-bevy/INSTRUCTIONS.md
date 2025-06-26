# Connect Four - Setup and Play Instructions

## Quick Start

1. **Install Rust**: Download from [https://rustup.rs/](https://rustup.rs/)
2. **Run the Game**: 
   ```bash
   cd connect-four-bevy
   cargo run
   ```

## Game Overview

Connect Four is a classic strategy game where two players take turns dropping colored pieces into a 7x6 grid. The objective is to be the first player to connect four of your pieces in a row, either horizontally, vertically, or diagonally.

## Controls

### Mouse Controls
- **Left Click on Column**: Drop your piece in the selected column
- **Hover over Board**: See column highlights showing where you can drop pieces
- **Click "New Game" Button**: Start a fresh game

### Keyboard Shortcuts
- **Keys 1-7**: Drop piece in columns 1-7 respectively
- **R Key**: Reset the game and start over

## Game Rules

1. **Turn Order**: Red player always goes first
2. **Piece Placement**: Pieces fall to the lowest available position in the selected column
3. **Winning Conditions**: Connect four pieces in any direction:
   - **Horizontal**: Four in a row (←→)
   - **Vertical**: Four in a column (↑↓)
   - **Diagonal**: Four diagonally (↗↙ or ↖↘)
4. **Draw**: Game ends in a draw if all 42 spaces are filled without a winner

## Visual Elements

- **Red Pieces**: Player 1 (goes first)
- **Yellow Pieces**: Player 2
- **Blue Board**: The game grid with darker holes for empty spaces
- **Column Highlights**: White transparent overlay when hovering over valid columns
- **Animated Drops**: Pieces bounce when they land

## Game Interface

### Top Panel
- **Current Player Display**: Shows whose turn it is with color coding
- **Game Title**: "CONNECT FOUR" centered
- **Score Tracker**: Displays wins for each player and draws

### Bottom Panel
- **Game Status**: Shows instructions or win/draw messages
- **New Game Button**: Click to reset the game

## Scoring System

The game tracks statistics across multiple rounds:
- **Red Wins**: Number of games won by the red player
- **Yellow Wins**: Number of games won by the yellow player
- **Draws**: Number of games that ended without a winner

## Strategy Tips

1. **Center Control**: Try to get pieces in the center columns (3-5) for more winning opportunities
2. **Block Opponents**: Watch for your opponent's potential four-in-a-row and block them
3. **Create Multiple Threats**: Set up situations where you can win in multiple ways
4. **Think Ahead**: Consider how your moves affect future placement options

## Technical Requirements

- **Operating System**: Windows, macOS, or Linux
- **Rust**: Latest stable version (1.70+)
- **Graphics**: OpenGL-compatible graphics card
- **Memory**: Minimum 512MB RAM
- **Storage**: ~50MB for the game and dependencies

## Compilation Options

### Development Mode (Faster compilation)
```bash
cargo run
```

### Release Mode (Better performance)
```bash
cargo run --release
```

### Running Tests
```bash
cargo test
```

### Running Examples
```bash
cargo run --example game_demo
```

## Troubleshooting

### Game Won't Start
- Ensure Rust is properly installed: `rustc --version`
- Update Rust: `rustup update`
- Clear build cache: `cargo clean`

### Performance Issues
- Run in release mode: `cargo run --release`
- Close other applications to free up system resources
- Check graphics drivers are up to date

### Compilation Errors
- Update dependencies: `cargo update`
- Check Rust version: `rustup show`
- Verify all source files are present

## Advanced Features

### Keyboard Navigation
- Use number keys 1-7 for quick column selection
- Press R to quickly restart the game

### Visual Feedback
- Hover effects show valid move locations
- Smooth animations provide satisfying gameplay
- Color-coded UI elements for easy identification

## Educational Use

This game is perfect for:
- Learning Rust programming
- Understanding game development with Bevy
- Teaching game logic and algorithms
- Demonstrating 2D graphics programming

## Support

For issues or questions:
1. Check the README.md for detailed technical information
2. Run the game_demo example to test core functionality
3. Verify your Rust installation is up to date
4. Review the source code for customization options

Enjoy playing Connect Four! 🔴🟡