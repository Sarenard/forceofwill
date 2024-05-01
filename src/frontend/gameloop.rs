use sdl2::pixels::Color;
use crate::HEIGHT;
use crate::WIDTH;
use crate::backend;

pub fn gameloop(mut canvas: sdl2::render::Canvas<sdl2::video::Window>, mut gamestate: backend::game::Game) -> 
    Result<(sdl2::render::Canvas<sdl2::video::Window>, backend::game::Game), String> {
    // we clean the scene
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    
    // we draw the two zones
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.draw_line(
        sdl2::rect::Point::new(0, (HEIGHT/2).try_into().unwrap()),
        sdl2::rect::Point::new(WIDTH.try_into().unwrap(), (HEIGHT/2).try_into().unwrap()),
    )?;
    
    canvas.present();

    Ok((canvas, gamestate))
}