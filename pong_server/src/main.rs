use futures_util::{SinkExt, StreamExt};
use pong_core::{ClientMessage, GameState, ServerMessage};
use rand::Rng;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message;

struct Room {
    game_state: GameState,
    p1_tx: Option<tokio::sync::mpsc::UnboundedSender<Message>>,
    p2_tx: Option<tokio::sync::mpsc::UnboundedSender<Message>>,
    p1_up: bool,
    p1_down: bool,
    p2_up: bool,
    p2_down: bool,
}

type Rooms = Arc<Mutex<HashMap<String, Room>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr).await?;
    println!("Pong Multiplayer Server running on ws://{}", addr);

    let rooms: Rooms = Arc::new(Mutex::new(HashMap::new()));

    while let Ok((mut stream, _)) = listener.accept().await {
        let rooms_clone = Arc::clone(&rooms);
        tokio::spawn(async move {
            // Check if client is requesting the HTTP health endpoint
            let mut peek_buf = [0u8; 512];
            let n = stream.peek(&mut peek_buf).await.unwrap_or(0);
            let peek_str = std::str::from_utf8(&peek_buf[..n]).unwrap_or("");

            if peek_str.starts_with("GET /health") {
                let mut discard = [0u8; 1024];
                let _ = stream.read(&mut discard).await;
                let resp = "HTTP/1.1 200 OK\r\n\
Content-Type: application/json\r\n\
Access-Control-Allow-Origin: *\r\n\
Access-Control-Allow-Methods: GET, OPTIONS\r\n\
Access-Control-Allow-Headers: *\r\n\
Content-Length: 15\r\n\
Connection: close\r\n\r\n\
{\"status\":\"ok\"}";
                let _ = stream.write_all(resp.as_bytes()).await;
                let _ = stream.flush().await;
                return;
            }

            if peek_str.starts_with("OPTIONS") {
                let mut discard = [0u8; 1024];
                let _ = stream.read(&mut discard).await;
                let resp = "HTTP/1.1 204 No Content\r\n\
Access-Control-Allow-Origin: *\r\n\
Access-Control-Allow-Methods: GET, OPTIONS\r\n\
Access-Control-Allow-Headers: *\r\n\
Connection: close\r\n\r\n";
                let _ = stream.write_all(resp.as_bytes()).await;
                let _ = stream.flush().await;
                return;
            }

            let ws_stream = match tokio_tungstenite::accept_async(stream).await {
                Ok(ws) => ws,
                Err(e) => {
                    eprintln!("Failed to accept websocket: {e}");
                    return;
                }
            };
            let (mut ws_tx, mut ws_rx) = ws_stream.split();

            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Message>();

            // Forward channel messages to websocket
            tokio::spawn(async move {
                while let Some(msg) = rx.recv().await {
                    if ws_tx.send(msg).await.is_err() {
                        break;
                    }
                }
            });

            let mut current_room: Option<String> = None;
            let mut player_num: u8 = 0;

            while let Some(result) = ws_rx.next().await {
                if let Ok(msg) = result {
                    if msg.is_text() || msg.is_binary() {
                        let text = match msg {
                            Message::Text(ref s) => s.as_str(),
                            Message::Binary(ref b) => match std::str::from_utf8(b) {
                                Ok(s) => s,
                                Err(_) => continue,
                            },
                            _ => continue,
                        };

                        if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(text) {
                            let mut rooms_lock = rooms_clone.lock().await;

                            match client_msg {
                                ClientMessage::CreateRoom => {
                                    let room_id: String = rand::thread_rng()
                                        .sample_iter(&rand::distributions::Alphanumeric)
                                        .take(4)
                                        .map(char::from)
                                        .collect::<String>()
                                        .to_uppercase();

                                    rooms_lock.insert(
                                        room_id.clone(),
                                        Room {
                                            game_state: GameState::new(),
                                            p1_tx: Some(tx.clone()),
                                            p2_tx: None,
                                            p1_up: false,
                                            p1_down: false,
                                            p2_up: false,
                                            p2_down: false,
                                        },
                                    );

                                    current_room = Some(room_id.clone());
                                    player_num = 1;

                                    let resp = ServerMessage::RoomCreated { room_id };
                                    let _ = tx
                                        .send(Message::Text(serde_json::to_string(&resp).unwrap()));
                                }
                                ClientMessage::JoinRoom { room_id } => {
                                    let room_id_upper = room_id.trim().to_uppercase();
                                    if let Some(room) = rooms_lock.get_mut(&room_id_upper) {
                                        if room.p2_tx.is_none() {
                                            room.p2_tx = Some(tx.clone());
                                            current_room = Some(room_id_upper.clone());
                                            player_num = 2;

                                            // Notify Player 1 that game is starting
                                            let p1_resp = ServerMessage::JoinedRoom {
                                                room_id: room_id_upper.clone(),
                                                player_num: 1,
                                            };
                                            if let Some(ref p1_tx) = room.p1_tx {
                                                let _ = p1_tx.send(Message::Text(
                                                    serde_json::to_string(&p1_resp).unwrap(),
                                                ));
                                            }

                                            // Notify Player 2 that they joined
                                            let p2_resp = ServerMessage::JoinedRoom {
                                                room_id: room_id_upper.clone(),
                                                player_num: 2,
                                            };
                                            let _ = tx.send(Message::Text(
                                                serde_json::to_string(&p2_resp).unwrap(),
                                            ));

                                            // Spawn room game loop tick
                                            start_room_loop(
                                                room_id_upper.clone(),
                                                Arc::clone(&rooms_clone),
                                            );
                                        } else {
                                            let err = ServerMessage::Error {
                                                message: "Room is full".into(),
                                            };
                                            let _ = tx.send(Message::Text(
                                                serde_json::to_string(&err).unwrap(),
                                            ));
                                        }
                                    } else {
                                        let err = ServerMessage::Error {
                                            message: "Room not found".into(),
                                        };
                                        let _ = tx.send(Message::Text(
                                            serde_json::to_string(&err).unwrap(),
                                        ));
                                    }
                                }
                                ClientMessage::PlayerInput { up, down } => {
                                    if let Some(ref room_id) = current_room {
                                        if let Some(room) = rooms_lock.get_mut(room_id) {
                                            if player_num == 1 {
                                                room.p1_up = up;
                                                room.p1_down = down;
                                            } else if player_num == 2 {
                                                room.p2_up = up;
                                                room.p2_down = down;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Client disconnected: clean up room
            if let Some(ref room_id) = current_room {
                let mut rooms_lock = rooms_clone.lock().await;
                if let Some(room) = rooms_lock.get_mut(room_id) {
                    let err = ServerMessage::Error {
                        message: "Opponent disconnected".into(),
                    };
                    let err_json = Message::Text(serde_json::to_string(&err).unwrap());
                    if player_num == 1 {
                        room.p1_tx = None;
                        if let Some(ref p2_tx) = room.p2_tx {
                            let _ = p2_tx.send(err_json);
                        }
                    } else if player_num == 2 {
                        room.p2_tx = None;
                        if let Some(ref p1_tx) = room.p1_tx {
                            let _ = p1_tx.send(err_json);
                        }
                    }
                }
                if let Some(room) = rooms_lock.get(room_id) {
                    if room.p1_tx.is_none() && room.p2_tx.is_none() {
                        rooms_lock.remove(room_id);
                    }
                }
            }
        });
    }

    Ok(())
}

fn start_room_loop(room_id: String, rooms: Rooms) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(16)); // ~60 FPS
        let mut pause_ticks = 60; // 1-second delay before game starts
        loop {
            interval.tick().await;
            let mut rooms_lock = rooms.lock().await;
            if let Some(room) = rooms_lock.get_mut(&room_id) {
                // If either player disconnected, stop loop
                if room.p1_tx.is_none() || room.p2_tx.is_none() {
                    break;
                }

                let should_start = if pause_ticks > 0 {
                    pause_ticks -= 1;
                    false
                } else {
                    true
                };

                let prev_started = room.game_state.game_started;

                // Construct combined inputs and update game state
                let input = pong_core::PlayerInput {
                    p1_up: room.p1_up,
                    p1_down: room.p1_down,
                    p2_up: room.p2_up,
                    p2_down: room.p2_down,
                    space: should_start,
                };

                room.game_state.update(input, 0.016);

                if prev_started && !room.game_state.game_started {
                    pause_ticks = 60; // Pause briefly after a point
                }

                let update_msg = ServerMessage::GameStateUpdate {
                    p1_y: room.game_state.p1_y,
                    p2_y: room.game_state.p2_y,
                    ball_x: room.game_state.ball_x,
                    ball_y: room.game_state.ball_y,
                    p1_score: room.game_state.p1_score,
                    p2_score: room.game_state.p2_score,
                    game_started: room.game_state.game_started,
                };
                let json = Message::Text(serde_json::to_string(&update_msg).unwrap());

                let mut p1_active = true;
                let mut p2_active = true;

                if let Some(ref tx) = room.p1_tx {
                    if tx.send(json.clone()).is_err() {
                        p1_active = false;
                    }
                }
                if let Some(ref tx) = room.p2_tx {
                    if tx.send(json).is_err() {
                        p2_active = false;
                    }
                }

                if !p1_active || !p2_active {
                    break;
                }
            } else {
                break;
            }
        }
    });
}
