use axum::Router;
use connect_four_lib::board::Column;
use connect_four_lib::game::Game;
use connect_four_lib::player::Player;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use socketioxide::SocketIo;
use socketioxide::extract::{Bin, Data, SocketRef};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
enum WsMsg {
    PlayerJoin { id: String, color: Player },
    PlayerLeave { id: String },
    PlayerMove { col: usize },
}

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

fn on_connect(socket: SocketRef, Data(data): Data<Value>, state: AppState) {
    info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);
    socket.emit("auth", data).ok();

    let state_for_join = state.clone();
    let state_for_leave = state.clone();
    let state_for_move = state.clone();

    // Join event
    socket.on(
        "join",
        move |socket: SocketRef, Data::<Value>(data), _bin: Bin| {
            let state = state_for_join.clone();
            info!("Received join event: {:?}", data);
            if let Ok(WsMsg::PlayerJoin { id, color: _ }) = serde_json::from_value::<WsMsg>(data) {
                // Assign color automatically using shared state
                let assigned_color = if state.get_player_for_color(Player::One).is_none() {
                    Player::One
                } else if state.get_player_for_color(Player::Two).is_none() {
                    Player::Two
                } else {
                    info!("Game is full. Rejecting player {}", id);
                    return;
                };
                {
                    let mut map = state.player_map.lock().unwrap();
                    map.insert(id.clone(), assigned_color);
                }
                state.set_player_for_color(assigned_color, Some(id.clone()));
                let join_msg = WsMsg::PlayerJoin {
                    id,
                    color: assigned_color,
                };
                let json = serde_json::to_string(&join_msg).unwrap();
                info!("sending message {:?}", join_msg);
                socket.emit("joined", json).ok();
            }
        },
    );

    // Leave event
    socket.on(
        "leave",
        move |socket: SocketRef, Data::<Value>(data), _bin: Bin| {
            let state = state_for_leave.clone();
            info!("Received leave event: {:?}", data);
            if let Ok(WsMsg::PlayerLeave { id }) = serde_json::from_value::<WsMsg>(data) {
                info!("player {} has left the game", id);
                let mut map = state.player_map.lock().unwrap();
                if let Some(color) = map.remove(&id) {
                    state.set_player_for_color(color, None);
                }
            }
        },
    );

    // Move event
    socket.on(
        "move",
        move |socket: SocketRef, Data::<Value>(data), _bin: Bin| {
            let _state = state_for_move.clone();
            info!("Received move event: {:?}", data);
            if let Ok(WsMsg::PlayerMove { col }) = serde_json::from_value::<WsMsg>(data) {
                info!("making move on col {}", col);
                let mut game = state.game.lock().unwrap();
                game.make_move(&Column::from(col));
            }
        },
    );
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let (layer, io) = SocketIo::new_layer();
    io.ns("/", on_connect);
    let state = AppState {
        player_map: Arc::new(Mutex::new(HashMap::new())),
        red_player: Arc::new(Mutex::new(None)),
        yellow_player: Arc::new(Mutex::new(None)),
        game: Arc::new(Mutex::new(Game::default())),
    };

    let app = Router::new().layer(layer).with_state(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();

    Ok(())
}
