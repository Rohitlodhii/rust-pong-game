pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 600.0;
pub const PADDLE_WIDTH: f32 = 20.0;
pub const PADDLE_HEIGHT: f32 = 100.0;
pub const BALL_SIZE: f32 = 20.0;
pub const PADDLE_SPEED: f32 = 6.0;

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
            ball_vel_x: 5.0,
            ball_vel_y: 3.0,
            p1_score: 0,
            p2_score: 0,
            game_started: false,
        }
    }

    pub fn update(&mut self, input: PlayerInput) {
        // Start game round on Space
        if input.space && !self.game_started {
            self.game_started = true;
        }

        // Paddle inputs
        if input.p1_up { self.p1_y -= PADDLE_SPEED; }
        if input.p1_down { self.p1_y += PADDLE_SPEED; }
        if input.p2_up { self.p2_y -= PADDLE_SPEED; }
        if input.p2_down { self.p2_y += PADDLE_SPEED; }

        // Clamp paddles
        self.p1_y = self.p1_y.clamp(0.0, SCREEN_HEIGHT - PADDLE_HEIGHT);
        self.p2_y = self.p2_y.clamp(0.0, SCREEN_HEIGHT - PADDLE_HEIGHT);

        if !self.game_started {
            self.ball_x = SCREEN_WIDTH / 2.0 - BALL_SIZE / 2.0;
            self.ball_y = SCREEN_HEIGHT / 2.0 - BALL_SIZE / 2.0;
            return;
        }

        // Move ball
        self.ball_x += self.ball_vel_x;
        self.ball_y += self.ball_vel_y;

        // Top/Bottom wall bounce
        if self.ball_y <= 0.0 {
            self.ball_y = 0.0;
            self.ball_vel_y = -self.ball_vel_y;
        } else if self.ball_y >= SCREEN_HEIGHT - BALL_SIZE {
            self.ball_y = SCREEN_HEIGHT - BALL_SIZE;
            self.ball_vel_y = -self.ball_vel_y;
        }

        // Collision bounds
        let p1_left = 30.0;
        let p1_right = 30.0 + PADDLE_WIDTH;
        let p1_top = self.p1_y;
        let p1_bottom = self.p1_y + PADDLE_HEIGHT;

        let p2_left = SCREEN_WIDTH - 30.0 - PADDLE_WIDTH;
        let p2_right = SCREEN_WIDTH - 30.0;
        let p2_top = self.p2_y;
        let p2_bottom = self.p2_y + PADDLE_HEIGHT;

        let b_left = self.ball_x;
        let b_right = self.ball_x + BALL_SIZE;
        let b_top = self.ball_y;
        let b_bottom = self.ball_y + BALL_SIZE;

        // Player 1 collision
        if b_left < p1_right && b_right > p1_left && b_top < p1_bottom && b_bottom > p1_top {
            self.ball_vel_x = -self.ball_vel_x;
            self.ball_x = p1_right;
        }

        // Player 2 collision
        if b_left < p2_right && b_right > p2_left && b_top < p2_bottom && b_bottom > p2_top {
            self.ball_vel_x = -self.ball_vel_x;
            self.ball_x = p2_left - BALL_SIZE;
        }

        // Scoring
        if self.ball_x < 0.0 {
            self.p2_score += 1;
            self.game_started = false;
            self.ball_vel_x = self.ball_vel_x.abs();
        } else if self.ball_x > SCREEN_WIDTH {
            self.p1_score += 1;
            self.game_started = false;
            self.ball_vel_x = -self.ball_vel_x.abs();
        }
    }
}
