use axum::{
    Router,
    extract::ws::{Message::Text, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
};
use connect_four_lib::player::Player;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

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
}

impl AppState {
    fn get_player_for_color(&self, color: Player) -> Option<String> {
        match color {
            Player::One => self.red_player.lock().unwrap().clone(),
            Player::Two => self.yellow_player.lock().unwrap().clone(),
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
        }
    }
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    state: axum::extract::State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: axum::extract::State<AppState>) {
    while let Some(msg) = socket.recv().await {
        if let Ok(Text(txt)) = msg {
            if let Ok(ws_msg) = serde_json::from_str::<WsMsg>(&txt) {
                match ws_msg {
                    WsMsg::PlayerJoin { id, color: _ } => {
                        println!("player {} is attempting to join the game", id);
                        // Assign color automatically
                        let assigned_color = if state.get_player_for_color(Player::One).is_none() {
                            Player::One
                        } else if state.get_player_for_color(Player::Two).is_none() {
                            Player::Two
                        } else {
                            // Game full!
                            println!("Game is full. Rejecting player {}", id);
                            continue;
                        };
                        // set up join outside critical section
                        let id_clone = id.clone();
                        let color_clone = assigned_color.clone();
                        {
                            let mut map = state.player_map.lock().unwrap();
                            map.insert(id.clone(), assigned_color);
                        }
                        match assigned_color {
                            Player::One => {
                                let mut red = state.red_player.lock().unwrap();
                                *red = Some(id.clone());
                            }
                            Player::Two => {
                                let mut yellow = state.yellow_player.lock().unwrap();
                                *yellow = Some(id.clone());
                            }
                        };
                        let join_msg = WsMsg::PlayerJoin {
                            id: id_clone,
                            color: color_clone,
                        };
                        let json = serde_json::to_string(&join_msg).unwrap();
                        println!("sending message {:?}", join_msg);
                        if socket.send(Text(json)).await.is_err() {
                            break;
                        }
                    }
                    WsMsg::PlayerLeave { id } => {
                        println!("player {} has left the game", id);
                        let mut map = state.player_map.lock().unwrap();
                        if let Some(color) = map.remove(&id) {
                            state.set_player_for_color(color, None);
                        }
                        // drop(map);
                    }
                    WsMsg::PlayerMove { col } => {
                        println!("making move on col {}", col);
                    }
                }
            } else {
                break;
            }
            // if socket.send(msg).await.is_err() {
            //     break;
            // }
        } else {
            break;
        }
    }
}

#[tokio::main]
async fn main() {
    let state = AppState {
        player_map: Arc::new(Mutex::new(HashMap::new())),
        red_player: Arc::new(Mutex::new(None)),
        yellow_player: Arc::new(Mutex::new(None)),
    };
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
