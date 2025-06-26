//! A simple demonstration of the Connect Four game logic
//! This example shows how to use the game programmatically without the GUI

use connect_four_bevy::game_logic::*;

fn main() {
    println!("🔴🟡 Connect Four Game Logic Demo 🟡🔴");
    println!("=====================================\n");

    // Create a new game
    let mut game = GameState::new();

    // Display initial state
    print_game_state(&game);

    // Simulate a game sequence
    let moves = vec![
        (3, "Red drops in column 4"),
        (3, "Yellow drops in column 4"),
        (2, "Red drops in column 3"),
        (4, "Yellow drops in column 5"),
        (1, "Red drops in column 2"),
        (5, "Yellow drops in column 6"),
        (0, "Red drops in column 1 - WINS!"),
    ];

    println!("Simulating a game...\n");

    for (column, description) in moves {
        if game.status == GameStatus::Playing {
            println!("Move: {}", description);

            if let Some(row) = game.drop_piece(column) {
                println!("✅ Piece dropped at row {}, column {}", row, column);
                print_board(&game);

                match game.status {
                    GameStatus::Won(player) => {
                        println!("🎉 Game Over! {:?} player wins!\n", player);
                        break;
                    }
                    GameStatus::Draw => {
                        println!("🤝 Game Over! It's a draw!\n");
                        break;
                    }
                    GameStatus::Playing => {
                        println!("Next player: {:?}\n", game.current_player);
                    }
                }
            } else {
                println!("❌ Invalid move! Column {} is full or invalid\n", column);
            }
        }
    }

    // Demonstrate win detection
    println!("\n🧪 Testing Win Detection");
    println!("========================");

    test_horizontal_win();
    test_vertical_win();
    test_diagonal_win();

    println!("\n✅ All game logic tests completed!");
}

fn print_game_state(game: &GameState) {
    println!("Game Status: {:?}", game.status);
    println!("Current Player: {:?}", game.current_player);
    println!("Moves Made: {}", game.move_count);
    print_board(game);
}

fn print_board(game: &GameState) {
    println!("\nBoard State:");
    println!(" 1 2 3 4 5 6 7");
    println!("┌─┬─┬─┬─┬─┬─┬─┐");

    for row in 0..6 {
        print!("│");
        for col in 0..7 {
            match game.board[row][col] {
                Some(Player::Red) => print!("🔴"),
                Some(Player::Yellow) => print!("🟡"),
                None => print!(" "),
            }
            print!("│");
        }
        println!();
        if row < 5 {
            println!("├─┼─┼─┼─┼─┼─┼─┤");
        }
    }
    println!("└─┴─┴─┴─┴─┴─┴─┘");
    println!(" 1 2 3 4 5 6 7\n");
}

fn test_horizontal_win() {
    println!("\n🔍 Testing Horizontal Win...");
    let mut game = GameState::new();

    // Create horizontal win for Red in bottom row
    let moves = vec![0, 0, 1, 1, 2, 2, 3]; // Red wins on column 3

    for column in moves {
        game.drop_piece(column);
    }

    match game.status {
        GameStatus::Won(Player::Red) => println!("✅ Horizontal win detected correctly!"),
        _ => println!("❌ Horizontal win not detected"),
    }
}

fn test_vertical_win() {
    println!("\n🔍 Testing Vertical Win...");
    let mut game = GameState::new();

    // Create vertical win for Red in column 0
    let moves = vec![0, 1, 0, 1, 0, 1, 0]; // Red wins on 4th piece in column 0

    for column in moves {
        game.drop_piece(column);
    }

    match game.status {
        GameStatus::Won(Player::Red) => println!("✅ Vertical win detected correctly!"),
        _ => println!("❌ Vertical win not detected"),
    }
}

fn test_diagonal_win() {
    println!("\n🔍 Testing Diagonal Win...");
    let mut game = GameState::new();

    // Create a diagonal win pattern
    // This creates a specific pattern for Red to win diagonally
    let moves = vec![
        0, 1, 1, 2, 2, 3, 2, 3, 3, 6, 3, // Red gets diagonal win
    ];

    for column in moves {
        if game.status == GameStatus::Playing {
            game.drop_piece(column);
        }
    }

    match game.status {
        GameStatus::Won(_) => println!("✅ Diagonal win detected correctly!"),
        _ => println!(
            "❌ Diagonal win not detected (this test pattern may not create a diagonal win)"
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_demo_runs() {
        // Just ensure the demo functions don't panic
        let game = GameState::new();
        print_board(&game);
        // This test mainly ensures the demo code compiles and runs
    }
}
