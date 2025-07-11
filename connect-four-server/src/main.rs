use axum::Router;
use axum::routing::get;
use connect_four_lib::game::Game;
use connect_four_lib::player::Player;
use connect_four_lib::web_socket::WsMsg;
use handlers::ws_handler;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::sync::{RwLock, mpsc};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

mod handlers;

#[derive(Debug)]
struct Connection {
    id: String,
    tx: mpsc::UnboundedSender<WsMsg>,
}

#[derive(Clone)]
struct AppState {
    player_map: Arc<RwLock<HashMap<String, Player>>>,
    red_player: Arc<RwLock<Option<String>>>,
    yellow_player: Arc<RwLock<Option<String>>>,
    game: Arc<RwLock<Game>>,
    connections: Arc<RwLock<HashMap<String, Connection>>>,
}

impl AppState {
    async fn get_player_for_color(&self, color: Player) -> Option<String> {
        match color {
            Player::One => self.red_player.read().await.clone(),
            Player::Two => self.yellow_player.read().await.clone(),
            _ => None,
        }
    }

    async fn set_player_for_color(&self, color: Player, id: Option<String>) {
        match color {
            Player::One => {
                *self.red_player.write().await = id;
            }
            Player::Two => {
                *self.yellow_player.write().await = id;
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
        player_map: Arc::new(RwLock::new(HashMap::new())),
        red_player: Arc::new(RwLock::new(None)),
        yellow_player: Arc::new(RwLock::new(None)),
        game: Arc::new(RwLock::new(Game::default())),
        connections: Arc::new(RwLock::new(HashMap::new())),
    };
    // let (layer, io) = SocketIo::builder().with_state(state).build_layer();
    // io.ns("/", ws_handler);

    let app = Router::new().route("/", get(ws_handler)).with_state(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();

    Ok(())
}
