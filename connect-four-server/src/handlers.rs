use axum::extract::State;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::Response;
use connect_four_lib::player::Player;
use futures_util::SinkExt;
use futures_util::stream::StreamExt;
use tokio::sync::mpsc::{self};
use tracing::info;

use crate::{AppState, Connection, WsMsg};

pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(move |socket| websocket_connection(socket, State(state)))
}

async fn websocket_connection(socket: WebSocket, State(state): State<AppState>) {
    let connections = state.connections.clone();
    let connection_id = uuid::Uuid::new_v4().to_string();
    let (conn_tx, mut conn_rx) = mpsc::unbounded_channel();

    // Register this connection
    {
        let mut conns = connections.write().await;
        conns.insert(
            connection_id.clone(),
            Connection {
                id: connection_id.clone(),
                tx: conn_tx,
            },
        );
    }

    let (mut sender, mut receiver) = socket.split();
    // Handle incoming messages and broadcast to all
    let conns = connections.clone();
    let recv_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            if let Ok(msg) = msg {
                if let Ok(text) = msg.to_text() {
                    if let Ok(game_msg) = serde_json::from_str::<WsMsg>(text) {
                        match game_msg {
                            WsMsg::ClientJoin { id } => {
                                // Assign color automatically using shared state
                                let player_role = if state
                                    .get_player_for_color(Player::One)
                                    .await
                                    .is_none()
                                {
                                    Player::One
                                } else if state.get_player_for_color(Player::Two).await.is_none() {
                                    Player::Two
                                } else {
                                    Player::Spectator
                                };
                                {
                                    let mut map = state.player_map.write().await;
                                    map.insert(id.clone(), player_role);
                                }
                                let game = state.game.read().await;
                                let active_player = game.current_player();
                                state
                                    .set_player_for_color(player_role, Some(id.clone()))
                                    .await;
                                let join_msg = WsMsg::ServerJoin {
                                    id,
                                    client_player: player_role,
                                    active_player,
                                    game_board: game.get_board().get_board_array(),
                                };
                                // let json = serde_json::to_value(&join_msg).unwrap();
                                info!("sending message {:?}", join_msg);
                                let conns_guard = conns.read().await;
                                for (_, conn) in conns_guard.iter() {
                                    let _ = conn.tx.send(join_msg.clone());
                                }
                            }
                            WsMsg::ClientMove { id, col } => {
                                info!("making move on col {}", col);
                                let mut game = state.game.write().await;
                                let player_that_made_move = game.current_player();
                                match game.make_move(&col.into()) {
                                    Ok((col, row)) => {
                                        let msg = WsMsg::ServerMove {
                                            id,
                                            col: col.into(),
                                            row: row.into(),
                                            active_player: player_that_made_move,
                                        };
                                        info!("sending message {:?}", msg);
                                        let conns_guard = conns.read().await;
                                        for (_, conn) in conns_guard.iter() {
                                            let _ = conn.tx.send(msg.clone());
                                        }
                                    }
                                    Err(_e) => {
                                        // TODO: Handle server error messages
                                        // tracing::error!("Failed to make move: {:?}", e);
                                        // info!("sending message {:?}", msg);
                                        // let conns_guard = conns.read().await;
                                        // for (_, conn) in conns_guard.iter() {
                                        //     let _ = conn.tx.send(e.clone());
                                        // }
                                    }
                                }
                                if game.is_over() {
                                    let winner = game.get_winner().unwrap();
                                    info!("{} wins!", winner);
                                    let msg = WsMsg::GameOver { winner };
                                    info!("sending message {:?}", msg);
                                    let conns_guard = conns.read().await;
                                    for (_, conn) in conns_guard.iter() {
                                        let _ = conn.tx.send(msg.clone());
                                    }
                                }
                            }
                            WsMsg::PlayerLeave { id } => {
                                info!("player {} has left the game", id);
                                let mut map = state.player_map.write().await;
                                if let Some(color) = map.remove(&id) {
                                    state.set_player_for_color(color, None).await;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    });

    // Handle outgoing messages
    let send_task = tokio::spawn(async move {
        while let Some(msg) = conn_rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                sender
                    .send(Message::Text(json.into()))
                    .await
                    .expect("unable to send message");
            }
        }
    });

    tokio::select! {
        _ = recv_task => {},
        _ = send_task => {},
    }

    // Clean up
    let mut conns = connections.write().await;
    conns.remove(&connection_id);
}
