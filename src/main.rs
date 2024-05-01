extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use std::time::Duration;

mod backend;
mod frontend;

const FPS: u32 = 30;
const WIDTH: u32 = 1200;
const HEIGHT: u32 = 800;

pub fn main() -> Result<(), String> {

    let mut gamestate = backend::game::Game::new();

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", WIDTH, HEIGHT)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    // init
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
        //println!("{} {}", canvas.output_size()?.0, canvas.output_size()?.1);
        (canvas, gamestate) = frontend::gameloop::gameloop(canvas, gamestate)?;
    }

    Ok(())
}