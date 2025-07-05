use connect_four_lib::board::Column;
use connect_four_lib::player::Player;
use serde_json::Value;
use socketioxide::extract::{Bin, Data, SocketRef, State};
use tracing::info;

use crate::{AppState, WsMsg};

pub fn ws_handler(socket: SocketRef, Data(data): Data<Value>, State(state): State<AppState>) {
    info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);

    // Join game room
    socket.join("game").ok();

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
                let player_role = if state.get_player_for_color(Player::One).is_none() {
                    Player::One
                } else if state.get_player_for_color(Player::Two).is_none() {
                    Player::Two
                } else {
                    Player::Spectator
                };
                {
                    let mut map = state.player_map.lock().unwrap();
                    map.insert(id.clone(), player_role);
                }
                state.set_player_for_color(player_role, Some(id.clone()));
                let join_msg = WsMsg::PlayerJoin {
                    id,
                    color: player_role,
                };
                let json = serde_json::to_value(&join_msg).unwrap();
                info!("sending message {:?}", join_msg);
                // Send to all clients within game room
                socket.within("game").emit("joined", json).ok();
            }
        },
    );

    // Leave event
    socket.on(
        "leave",
        move |_socket: SocketRef, Data::<Value>(data), _bin: Bin| {
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
            if let Ok(WsMsg::PlayerMove { id, col }) = serde_json::from_value::<WsMsg>(data) {
                info!("making move on col {}", col);
                let mut game = state.game.lock().unwrap();
                if let Err(e) = game.make_move(&Column::from(col)) {
                    let _ = socket.emit("error", serde_json::to_value(&e).ok());
                    tracing::error!("Failed to make move: {:?}", e);
                } else {
                    let msg = WsMsg::PlayerMove { id, col };
                    socket
                        .emit("move", serde_json::to_value(&msg).unwrap())
                        .ok();
                }
            }
        },
    );
}
