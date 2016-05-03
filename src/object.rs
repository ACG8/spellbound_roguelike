use piston_window::*;
use gfx_device_gl::{Resources, Output, CommandBuffer};
use gfx_graphics::GfxGraphics;
use sprite::Sprite;
use dungeon::Map;
use dijkstra_map::DijkstraMap;

pub enum Behavior {
    Player,
    Coward,
}

pub struct Creature {
    pub object: Object,
    pub hp: usize,
    ai: Behavior,
}

impl Creature {
    pub fn new(name: &str, pos:(usize,usize), w:&PistonWindow, graphic: &str, hp:usize, ai:Behavior) -> Creature {
        Creature {
            object: Object::new(name,pos,w,graphic),
            hp: hp,
            ai: ai,
        }
    }
}

pub struct Object {
    name: String,
    pub i: usize,
    pub j: usize,
    sprite: Sprite,
}

#[allow(dead_code)]
impl Object {

    pub fn new(name:&str, pos:(usize,usize), w:&PistonWindow, filename: &str) -> Object {
        Object {name:name.to_string(),i : pos.0, j: pos.1, sprite: Sprite::new(w,filename)}
    }
    //Functions to translate between discrete and continuous coordinates. One cell is 32 units.
    pub fn x(&self) -> f64 { (self.i as f64)*32.0 }
    pub fn y(&self) -> f64 { (self.j as f64)*32.0 }

    pub fn mov(&mut self, map: &Map, i:isize, j:isize) {
        let i2 = (self.i as isize + i) as usize;
        let j2 = (self.j as isize + j) as usize;

        if map.tile(i2,j2).is_passable() {
            self.i = i2;
            self.j = j2;
        }
    }

    pub fn automove(&mut self, dmap: &DijkstraMap) {
        let (i,j) = dmap.get_next_step((self.i,self.j));
        self.i = i;
        self.j = j;
    }

    pub fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer<Resources>, Output>, view: math::Matrix2d) {
        self.sprite.render(self.x(),self.y(),g,view)
        /*
        let square = rectangle::square(0.0, 0.0, 100.0);
        let red = [1.0, 0.0, 0.0, 1.0];
        match self.sprite {
            None => { //TODO - replace with error sprite
                rectangle(red, square, view.trans(self.x(), self.y()).trans(-50.0, -50.0), g); 
            }
            Some(ref sprite) => {
                image(sprite, view.trans(self.x(), self.y()), g);//.trans(-50.0, -50.0), g);
            }
        }*/
    }/*
    pub fn set_sprite(&mut self, sprite: Texture<Resources>) {
        /*let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
        let sprite = assets.join(filename);
        let sprite = Texture::from_path(
                &mut *w.factory.borrow_mut(),
                &sprite,
                Flip::None,
                &TextureSettings::new())
                .unwrap();
    */
        self.sprite = Some(sprite);
    }*/
}