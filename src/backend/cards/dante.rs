use sdl2::image::{InitFlag, LoadTexture};
use sdl2::rect::Rect;

use crate::backend::game::Card;
pub struct DanteCard {
    
}

impl Card for DanteCard {
    fn get_path(&self) -> String {
        "assets/cards/DANTE.jpg".to_string()
    }

    fn show(&self, x: i32, y: i32, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture(
            self.get_path()
        ).unwrap();
        canvas.copy(&texture, None, 
            Rect::new(x, y, 48, 67)
        ).unwrap();
    }
}
