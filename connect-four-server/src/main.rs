use axum::Router;
use connect_four_lib::game::Game;
use connect_four_lib::player::Player;
use connect_four_lib::web_socket::WsMsg;
use handlers::ws_handler;
use socketioxide::SocketIo;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tracing_subscriber::FmtSubscriber;

mod handlers;

#[derive(Clone)]
struct AppState {
    player_map: Arc<Mutex<HashMap<String, Player>>>,
    red_player: Arc<Mutex<Option<String>>>,
    yellow_player: Arc<Mutex<Option<String>>>,
    game: Arc<Mutex<Game>>,
}

impl AppState {
    fn get_player_for_color(&self, color: Player) -> Option<String> {
        match color {
            Player::One => self.red_player.lock().unwrap().clone(),
            Player::Two => self.yellow_player.lock().unwrap().clone(),
            _ => None,
        }
    }
    fn set_player_for_color(&self, color: Player, id: Option<String>) {
        match color {
            Player::One => {
                *self.red_player.lock().unwrap() = id;
            }
            Player::Two => {
                *self.yellow_player.lock().unwrap() = id;
            }
            // TODO: figure out what to do for spectators
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let state = AppState {
        player_map: Arc::new(Mutex::new(HashMap::new())),
        red_player: Arc::new(Mutex::new(None)),
        yellow_player: Arc::new(Mutex::new(None)),
        game: Arc::new(Mutex::new(Game::default())),
    };
    let (layer, io) = SocketIo::builder().with_state(state).build_layer();
    io.ns("/", ws_handler);

    let app = Router::new().layer(layer);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();

    Ok(())
}
