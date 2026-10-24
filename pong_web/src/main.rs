use macroquad::prelude::*;
use pong_core::{
    BALL_SIZE, GameState, PADDLE_HEIGHT, PADDLE_WIDTH, PlayerInput, SCREEN_HEIGHT, SCREEN_WIDTH,
};

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
    let mut state = GameState::new();

    loop {
        // Get elapsed time since last frame in seconds (e.g. ~0.016 for 60fps, ~0.007 for 144fps)
        let dt = get_frame_time();

        let input = PlayerInput {
            p1_up: is_key_down(KeyCode::W),
            p1_down: is_key_down(KeyCode::S),
            p2_up: is_key_down(KeyCode::Up),
            p2_down: is_key_down(KeyCode::Down),
            space: is_key_pressed(KeyCode::Space),
        };

        // Pass dt into update
        state.update(input, dt);

        clear_background(Color::new(0.08, 0.08, 0.08, 1.0));

        draw_rectangle(30.0, state.p1_y, PADDLE_WIDTH, PADDLE_HEIGHT, WHITE);
        draw_rectangle(
            SCREEN_WIDTH - 30.0 - PADDLE_WIDTH,
            state.p2_y,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
            WHITE,
        );
        draw_rectangle(state.ball_x, state.ball_y, BALL_SIZE, BALL_SIZE, WHITE);

        let score_text = format!("P1: {}  -  P2: {}", state.p1_score, state.p2_score);
        draw_text(&score_text, SCREEN_WIDTH / 2.0 - 65.0, 40.0, 30.0, WHITE);

        if !state.game_started {
            draw_text(
                "Press SPACE to Start",
                SCREEN_WIDTH / 2.0 - 120.0,
                SCREEN_HEIGHT / 2.0,
                25.0,
                GRAY,
            );
        }

        next_frame().await;
    }
}
