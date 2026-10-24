use macroquad::prelude::*;
use pong_core::{
    BALL_SIZE, GameState, PADDLE_HEIGHT, PADDLE_WIDTH, PlayerInput, SCREEN_HEIGHT, SCREEN_WIDTH,
};

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn is_online_mode() -> i32;
    fn get_player_num() -> i32;
    fn get_online_state(out: *mut f32) -> i32;
}

#[cfg(not(target_arch = "wasm32"))]
unsafe fn is_online_mode() -> i32 {
    0
}

#[cfg(not(target_arch = "wasm32"))]
unsafe fn get_player_num() -> i32 {
    0
}

#[cfg(not(target_arch = "wasm32"))]
unsafe fn get_online_state(_out: *mut f32) -> i32 {
    0
}

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Pong Web (Macroquad)"),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut offline_state = GameState::new();
    let mut online_state = GameState::new();
    let mut was_online = false;

    let mut state_buf = [0.0f32; 7];

    loop {
        let dt = get_frame_time();
        let online = unsafe { is_online_mode() } != 0;
        let player_num = unsafe { get_player_num() };

        if online {
            was_online = true;
            let has_state = unsafe { get_online_state(state_buf.as_mut_ptr()) } != 0;
            if has_state {
                online_state.p1_y = state_buf[0];
                online_state.p2_y = state_buf[1];
                online_state.ball_x = state_buf[2];
                online_state.ball_y = state_buf[3];
                online_state.p1_score = state_buf[4] as u32;
                online_state.p2_score = state_buf[5] as u32;
                online_state.game_started = state_buf[6] > 0.5;
            }

            clear_background(Color::new(0.08, 0.08, 0.08, 1.0));

            // Draw center dividing dashed line
            let mut y = 10.0;
            while y < SCREEN_HEIGHT {
                draw_rectangle(
                    SCREEN_WIDTH / 2.0 - 1.0,
                    y,
                    2.0,
                    15.0,
                    Color::new(0.2, 0.2, 0.2, 1.0),
                );
                y += 25.0;
            }

            // Draw paddles with player indicators
            let p1_color = if player_num == 1 {
                Color::new(0.2, 0.8, 0.2, 1.0)
            } else {
                WHITE
            };
            let p2_color = if player_num == 2 {
                Color::new(0.2, 0.8, 0.2, 1.0)
            } else {
                WHITE
            };

            draw_rectangle(
                30.0,
                online_state.p1_y,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
                p1_color,
            );
            draw_rectangle(
                SCREEN_WIDTH - 30.0 - PADDLE_WIDTH,
                online_state.p2_y,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
                p2_color,
            );

            // Draw player indicator tags
            if player_num == 1 {
                draw_text("YOU (P1)", 20.0, 30.0, 20.0, Color::new(0.2, 0.8, 0.2, 1.0));
                draw_text(
                    "OPPONENT",
                    SCREEN_WIDTH - 130.0,
                    30.0,
                    20.0,
                    Color::new(0.6, 0.6, 0.6, 1.0),
                );
            } else if player_num == 2 {
                draw_text("OPPONENT", 20.0, 30.0, 20.0, Color::new(0.6, 0.6, 0.6, 1.0));
                draw_text(
                    "YOU (P2)",
                    SCREEN_WIDTH - 110.0,
                    30.0,
                    20.0,
                    Color::new(0.2, 0.8, 0.2, 1.0),
                );
            }

            // Draw ball
            draw_rectangle(
                online_state.ball_x,
                online_state.ball_y,
                BALL_SIZE,
                BALL_SIZE,
                WHITE,
            );

            let score_text = format!(
                "P1: {}  -  P2: {}",
                online_state.p1_score, online_state.p2_score
            );
            draw_text(&score_text, SCREEN_WIDTH / 2.0 - 65.0, 40.0, 30.0, WHITE);

            if !online_state.game_started {
                draw_text(
                    "Get Ready...",
                    SCREEN_WIDTH / 2.0 - 60.0,
                    SCREEN_HEIGHT / 2.0,
                    25.0,
                    GRAY,
                );
            }
        } else {
            if was_online {
                was_online = false;
                offline_state = GameState::new();
            }

            let input = PlayerInput {
                p1_up: is_key_down(KeyCode::W),
                p1_down: is_key_down(KeyCode::S),
                p2_up: is_key_down(KeyCode::Up),
                p2_down: is_key_down(KeyCode::Down),
                space: is_key_pressed(KeyCode::Space),
            };

            offline_state.update(input, dt);

            clear_background(Color::new(0.08, 0.08, 0.08, 1.0));

            // Draw center dividing dashed line
            let mut y = 10.0;
            while y < SCREEN_HEIGHT {
                draw_rectangle(
                    SCREEN_WIDTH / 2.0 - 1.0,
                    y,
                    2.0,
                    15.0,
                    Color::new(0.2, 0.2, 0.2, 1.0),
                );
                y += 25.0;
            }

            draw_rectangle(30.0, offline_state.p1_y, PADDLE_WIDTH, PADDLE_HEIGHT, WHITE);
            draw_rectangle(
                SCREEN_WIDTH - 30.0 - PADDLE_WIDTH,
                offline_state.p2_y,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
                WHITE,
            );
            draw_rectangle(
                offline_state.ball_x,
                offline_state.ball_y,
                BALL_SIZE,
                BALL_SIZE,
                WHITE,
            );

            let score_text = format!(
                "P1: {}  -  P2: {}",
                offline_state.p1_score, offline_state.p2_score
            );
            draw_text(&score_text, SCREEN_WIDTH / 2.0 - 65.0, 40.0, 30.0, WHITE);

            if !offline_state.game_started {
                draw_text(
                    "Press SPACE to Start",
                    SCREEN_WIDTH / 2.0 - 120.0,
                    SCREEN_HEIGHT / 2.0,
                    25.0,
                    GRAY,
                );
            }
        }

        next_frame().await;
    }
}
