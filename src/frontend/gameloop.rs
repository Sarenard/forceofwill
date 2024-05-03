use sdl2::pixels::Color;

use crate::backend;

pub fn gameloop(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, gamestate: &mut backend::game::Game) {
    // we clean the scene
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    
    // we draw the two zones
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.draw_line(
        sdl2::rect::Point::new(
            0, 
            (canvas.output_size().unwrap().1/2).try_into().unwrap()),
        sdl2::rect::Point::new(
            canvas.output_size().unwrap().0.try_into().unwrap(), 
            (canvas.output_size().unwrap().1/2).try_into().unwrap(),
        ),
    ).unwrap();
    
    // we draw the deck
    gamestate.show(canvas);
    
    canvas.present();
}