mod game;

use sdl2::render::Canvas;
use sdl2::video::Window;

fn main() -> Result<(), String> {
    // Initialize SDL2 Context

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Creating a window

    let window = video_subsystem
        .window("Pong in rust", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    //Creating  canvas

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    // Get the event Pump
    let event_pump = sdl_context.event_pump()?;

    game::run(canvas, event_pump)?;

    Ok(())
}
