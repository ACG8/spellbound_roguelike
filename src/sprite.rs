use piston_window::*;
use gfx_device_gl::{Resources, Output, CommandBuffer};
use gfx_graphics::GfxGraphics;
use find_folder::Search;

pub struct Sprite {
	image: Option<Texture<Resources>>,
}

impl Sprite {
	pub fn new(w:&PistonWindow,filename: &str) -> Sprite {
		let assets = Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    	let image = assets.join(filename);
    	let image = Texture::from_path(
            &mut *w.factory.borrow_mut(),
            &image,
            Flip::None,
            &TextureSettings::new())
            .unwrap();
        Sprite{image:Some(image)}
	}
	pub fn render(&self, x: f64, y:f64, g: &mut GfxGraphics<Resources, CommandBuffer<Resources>, Output>, view: math::Matrix2d) {
        let square = rectangle::square(0.0, 0.0, 100.0);
        let red = [1.0, 0.0, 0.0, 1.0];
        match self.image {
            None => { //TODO - replace with error sprite
                rectangle(red, square, view.trans(x,y).trans(-50.0, -50.0), g); 
            }
            Some(ref sprite) => {
                image(sprite, view.trans(x,y), g);
            }
        }
    }
}