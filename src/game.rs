use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

pub fn run(mut canvas: Canvas<Window>, mut event_pump: EventPump) -> Result<(), String> {
    // Paddle positions
    let mut p1_y: f32 = 220.0;
    let mut p2_y: f32 = 220.0;
    let paddle_speed: f32 = 6.0;

    // Scoring & State Variables
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut game_started = false;

    // Ball state
    let mut ball_x = 400.0f32;
    let mut ball_y = 300.0f32;
    let mut ball_vel_x = 5.0f32;
    let mut ball_vel_y = 3.0f32;

    // Dimensions
    const SCREEN_WIDTH: u32 = 800;
    const SCREEN_HEIGHT: u32 = 600;
    const PADDLE_WIDTH: u32 = 20;
    const PADDLE_HEIGHT: u32 = 100;
    const BALL_SIZE: u32 = 20;

    'running: loop {
        // 1. Event Handling
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape | Keycode::Q),
                    ..
                } => {
                    break 'running;
                }
                // Listen for Spacebar to start the game round
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    if !game_started {
                        game_started = true;
                    }
                }
                _ => {}
            }
        }

        // 2. Keyboard Input for Paddles
        let keyboard_state = event_pump.keyboard_state();

        if keyboard_state.is_scancode_pressed(Scancode::W) {
            p1_y -= paddle_speed;
        }
        if keyboard_state.is_scancode_pressed(Scancode::S) {
            p1_y += paddle_speed;
        }
        if keyboard_state.is_scancode_pressed(Scancode::Up) {
            p2_y -= paddle_speed;
        }
        if keyboard_state.is_scancode_pressed(Scancode::Down) {
            p2_y += paddle_speed;
        }

        // 3. Clamping Paddles
        p1_y = p1_y.clamp(0.0, (SCREEN_HEIGHT - PADDLE_HEIGHT) as f32);
        p2_y = p2_y.clamp(0.0, (SCREEN_HEIGHT - PADDLE_HEIGHT) as f32);

        // 4. Ball Physics & Scoring Logic
        if game_started {
            ball_x += ball_vel_x;
            ball_y += ball_vel_y;

            // Top/Bottom wall bounce
            if ball_y <= 0.0 {
                ball_y = 0.0;
                ball_vel_y = -ball_vel_y;
            } else if ball_y >= (SCREEN_HEIGHT - BALL_SIZE) as f32 {
                ball_y = (SCREEN_HEIGHT - BALL_SIZE) as f32;
                ball_vel_y = -ball_vel_y;
            }

            // Paddle bounds for collision
            let p1_left = 30.0;
            let p1_right = (30 + PADDLE_WIDTH) as f32;
            let p1_top = p1_y;
            let p1_bottom = p1_y + PADDLE_HEIGHT as f32;

            let p2_left = (SCREEN_WIDTH - 30 - PADDLE_WIDTH) as f32;
            let p2_right = (SCREEN_WIDTH - 30) as f32;
            let p2_top = p2_y;
            let p2_bottom = p2_y + PADDLE_HEIGHT as f32;

            let ball_left = ball_x;
            let ball_right = ball_x + BALL_SIZE as f32;
            let ball_top = ball_y;
            let ball_bottom = ball_y + BALL_SIZE as f32;

            // Player 1 collision
            if ball_left < p1_right
                && ball_right > p1_left
                && ball_top < p1_bottom
                && ball_bottom > p1_top
            {
                ball_vel_x = -ball_vel_x;
                ball_x = p1_right;
            }

            // Player 2 collision
            if ball_left < p2_right
                && ball_right > p2_left
                && ball_top < p2_bottom
                && ball_bottom > p2_top
            {
                ball_vel_x = -ball_vel_x;
                ball_x = p2_left - BALL_SIZE as f32;
            }

            // Scoring (Out of bounds on left or right walls)
            if ball_x < 0.0 {
                p2_score += 1; // Player 2 scores
                ball_x = ((SCREEN_WIDTH / 2) - (BALL_SIZE / 2)) as f32;
                ball_y = ((SCREEN_HEIGHT / 2) - (BALL_SIZE / 2)) as f32;
                ball_vel_x = ball_vel_x.abs(); // Reset serve direction
                game_started = false; // Pause game until Space is pressed
            } else if ball_x > SCREEN_WIDTH as f32 {
                p1_score += 1; // Player 1 scores
                ball_x = ((SCREEN_WIDTH / 2) - (BALL_SIZE / 2)) as f32;
                ball_y = ((SCREEN_HEIGHT / 2) - (BALL_SIZE / 2)) as f32;
                ball_vel_x = -ball_vel_x.abs(); // Reset serve direction
                game_started = false; // Pause game until Space is pressed
            }
        } else {
            // Keep the ball locked to the center while waiting for spacebar
            ball_x = ((SCREEN_WIDTH / 2) - (BALL_SIZE / 2)) as f32;
            ball_y = ((SCREEN_HEIGHT / 2) - (BALL_SIZE / 2)) as f32;
        }

        // Update the window title bar with scores and status text
        let window_title = format!(
            "Pong Game  |  Player 1 (W/S): {}  -  Player 2 (Up/Down): {}  |  {}",
            p1_score,
            p2_score,
            if game_started {
                "Playing"
            } else {
                "Press [SPACE] to Start"
            }
        );
        canvas.window_mut().set_title(&window_title).unwrap();

        // 5. Rendering
        canvas.set_draw_color(Color::RGB(20, 20, 20));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));

        // Draw Player 1 Paddle
        let p1_rect = Rect::new(30, p1_y as i32, PADDLE_WIDTH, PADDLE_HEIGHT);
        canvas.fill_rect(p1_rect).unwrap();

        // Draw Player 2 Paddle
        let p2_rect = Rect::new(
            (SCREEN_WIDTH - 30 - PADDLE_WIDTH) as i32,
            p2_y as i32,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        );
        canvas.fill_rect(p2_rect).unwrap();

        // Draw Ball
        let ball_rect = Rect::new(ball_x as i32, ball_y as i32, BALL_SIZE, BALL_SIZE);
        canvas.fill_rect(ball_rect).unwrap();

        canvas.present();

        // 6. Frame Pacing (~60 FPS)
        std::thread::sleep(Duration::new(0, 1_000_000_000_u32 / 60));
    }

    Ok(())
}
