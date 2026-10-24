use serde::{Deserialize, Serialize};

pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 600.0;
pub const PADDLE_WIDTH: f32 = 20.0;
pub const PADDLE_HEIGHT: f32 = 100.0;
pub const BALL_SIZE: f32 = 20.0;
pub const PADDLE_SPEED: f32 = 450.0;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AppState {
    Menu,
    OfflineGame,
    OnlineLobby,
    OnlineGame,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ClientMessage {
    CreateRoom,
    JoinRoom { room_id: String },
    PlayerInput { up: bool, down: bool },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ServerMessage {
    RoomCreated {
        room_id: String,
    },
    JoinedRoom {
        room_id: String,
        player_num: u8,
    },
    GameStateUpdate {
        p1_y: f32,
        p2_y: f32,
        ball_x: f32,
        ball_y: f32,
        p1_score: u32,
        p2_score: u32,
        game_started: bool,
    },
    Error {
        message: String,
    },
}

pub struct PlayerInput {
    pub p1_up: bool,
    pub p1_down: bool,
    pub p2_up: bool,
    pub p2_down: bool,
    pub space: bool,
}

pub struct GameState {
    pub p1_y: f32,
    pub p2_y: f32,
    pub ball_x: f32,
    pub ball_y: f32,
    pub ball_vel_x: f32,
    pub ball_vel_y: f32,
    pub p1_score: u32,
    pub p2_score: u32,
    pub game_started: bool,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            p1_y: 220.0,
            p2_y: 220.0,
            ball_x: SCREEN_WIDTH / 2.0 - BALL_SIZE / 2.0,
            ball_y: SCREEN_HEIGHT / 2.0 - BALL_SIZE / 2.0,
            ball_vel_x: 400.0,
            ball_vel_y: 250.0,
            p1_score: 0,
            p2_score: 0,
            game_started: false,
        }
    }

    pub fn update(&mut self, input: PlayerInput, dt: f32) {
        if input.space && !self.game_started {
            self.game_started = true;
        }

        if input.p1_up {
            self.p1_y -= PADDLE_SPEED * dt;
        }
        if input.p1_down {
            self.p1_y += PADDLE_SPEED * dt;
        }
        if input.p2_up {
            self.p2_y -= PADDLE_SPEED * dt;
        }
        if input.p2_down {
            self.p2_y += PADDLE_SPEED * dt;
        }

        self.p1_y = self.p1_y.clamp(0.0, SCREEN_HEIGHT - PADDLE_HEIGHT);
        self.p2_y = self.p2_y.clamp(0.0, SCREEN_HEIGHT - PADDLE_HEIGHT);

        if !self.game_started {
            self.ball_x = SCREEN_WIDTH / 2.0 - BALL_SIZE / 2.0;
            self.ball_y = SCREEN_HEIGHT / 2.0 - BALL_SIZE / 2.0;
            return;
        }

        self.ball_x += self.ball_vel_x * dt;
        self.ball_y += self.ball_vel_y * dt;

        if self.ball_y <= 0.0 {
            self.ball_y = 0.0;
            self.ball_vel_y = -self.ball_vel_y;
        } else if self.ball_y >= SCREEN_HEIGHT - BALL_SIZE {
            self.ball_y = SCREEN_HEIGHT - BALL_SIZE;
            self.ball_vel_y = -self.ball_vel_y;
        }

        let p1_right = 30.0 + PADDLE_WIDTH;
        let p2_left = SCREEN_WIDTH - 30.0 - PADDLE_WIDTH;

        // P1 Collision
        if self.ball_x <= p1_right
            && self.ball_x >= 30.0
            && self.ball_y + BALL_SIZE >= self.p1_y
            && self.ball_y <= self.p1_y + PADDLE_HEIGHT
        {
            self.ball_vel_x = -self.ball_vel_x;
            self.ball_x = p1_right;
        }

        // P2 Collision
        if self.ball_x + BALL_SIZE >= p2_left
            && self.ball_x <= SCREEN_WIDTH - 30.0
            && self.ball_y + BALL_SIZE >= self.p2_y
            && self.ball_y <= self.p2_y + PADDLE_HEIGHT
        {
            self.ball_vel_x = -self.ball_vel_x;
            self.ball_x = p2_left - BALL_SIZE;
        }

        // Scoring
        if self.ball_x < 0.0 {
            self.p2_score += 1;
            self.game_started = false;
            self.ball_vel_x = 400.0;
        } else if self.ball_x > SCREEN_WIDTH {
            self.p1_score += 1;
            self.game_started = false;
            self.ball_vel_x = -400.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_client_messages() {
        let msg_create: Result<ClientMessage, _> = serde_json::from_str("\"CreateRoom\"");
        assert_eq!(msg_create.unwrap(), ClientMessage::CreateRoom);

        let msg_create_null: Result<ClientMessage, _> =
            serde_json::from_str("{\"CreateRoom\":null}");
        println!("CreateRoom null result: {:?}", msg_create_null);

        let msg_join: ClientMessage =
            serde_json::from_str("{\"JoinRoom\":{\"room_id\":\"ABCD\"}}").unwrap();
        assert_eq!(
            msg_join,
            ClientMessage::JoinRoom {
                room_id: "ABCD".into()
            }
        );

        let msg_input: ClientMessage =
            serde_json::from_str("{\"PlayerInput\":{\"up\":true,\"down\":false}}").unwrap();
        assert_eq!(
            msg_input,
            ClientMessage::PlayerInput {
                up: true,
                down: false
            }
        );
    }
}
