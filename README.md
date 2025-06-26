# Connect Four
This repo is a Cargo workspace containing everything necessary to create a web-based multiplayer game of Connect Four - entirely powered by Rust.

# connect-four-lib
The library that contains the core logic for the game.  Other repos should utilize this whenever necessary.
# connect-four-server
The web server for the game.  This is an Axum application that can serve both the HTML site containing the game, but also handles all game logic 
processessing and communicates updates to the various bevy clients via web socket.
# connect-four-bevy
The game that users will interact with. This should strictly create the game UI, handle user input, and react to events from the server to update game state.
