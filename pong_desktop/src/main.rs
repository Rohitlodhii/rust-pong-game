use pong_core::{
    BALL_SIZE, GameState, PADDLE_HEIGHT, PADDLE_WIDTH, PlayerInput, SCREEN_HEIGHT, SCREEN_WIDTH,
};
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window(
            "Pong Desktop (SDL2)",
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut state = GameState::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape | Keycode::Q),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        let keys = event_pump.keyboard_state();
        let input = PlayerInput {
            p1_up: keys.is_scancode_pressed(Scancode::W),
            p1_down: keys.is_scancode_pressed(Scancode::S),
            p2_up: keys.is_scancode_pressed(Scancode::Up),
            p2_down: keys.is_scancode_pressed(Scancode::Down),
            space: keys.is_scancode_pressed(Scancode::Space),
        };

        // Update core physics
        state.update(input);

        // Update title bar score
        let title = format!(
            "P1: {} | P2: {} | {}",
            state.p1_score,
            state.p2_score,
            if state.game_started {
                "Playing"
            } else {
                "Press SPACE"
            }
        );
        canvas.window_mut().set_title(&title).unwrap();

        // Render
        canvas.set_draw_color(Color::RGB(20, 20, 20));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let p1_rect = Rect::new(
            30,
            state.p1_y as i32,
            PADDLE_WIDTH as u32,
            PADDLE_HEIGHT as u32,
        );
        let p2_rect = Rect::new(
            (SCREEN_WIDTH - 30.0 - PADDLE_WIDTH) as i32,
            state.p2_y as i32,
            PADDLE_WIDTH as u32,
            PADDLE_HEIGHT as u32,
        );
        let ball_rect = Rect::new(
            state.ball_x as i32,
            state.ball_y as i32,
            BALL_SIZE as u32,
            BALL_SIZE as u32,
        );

        canvas.fill_rect(p1_rect).unwrap();
        canvas.fill_rect(p2_rect).unwrap();
        canvas.fill_rect(ball_rect).unwrap();

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000_u32 / 60));
    }

    Ok(())
}
