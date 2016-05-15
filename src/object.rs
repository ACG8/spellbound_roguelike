use piston_window::*;
use gfx_device_gl::{Resources, Output, CommandBuffer};
use gfx_graphics::GfxGraphics;
use sprite::Sprite;
use dungeon::Map;
use dijkstra_map::DijkstraMap;
use game::*;

pub enum Behavior {
    Player,
    Coward,
}

pub struct Creature {
    pub object: Object,
    pub hp: usize,
    pub ai: Behavior,
}

impl Creature {
    pub fn new(pos:(usize,usize), w:&PistonWindow, graphic: &str, hp:usize, ai:Behavior) -> Creature {
        Creature {
            object: Object::new(pos,w,graphic),
            hp: hp,
            ai: ai,
        }
    }

    pub fn coordinates(&self) -> (usize,usize) {self.object.coordinates()}
}

pub struct Object {
    pub i: usize,
    pub j: usize,
    sprite: Sprite,
}

#[allow(dead_code)]
impl Object {

    pub fn new(pos:(usize,usize), w:&PistonWindow, filename: &str) -> Object {
        Object {i : pos.0, j: pos.1, sprite: Sprite::new(w,filename)}
    }
    //Functions to translate between discrete and continuous coordinates. One cell is 32 units.
    pub fn x(&self) -> f64 { (self.i as f64)*32.0 }
    pub fn y(&self) -> f64 { (self.j as f64)*32.0 }

    pub fn coordinates(&self) -> (usize,usize) {(self.i,self.j)}

    pub fn mov(&mut self, i:isize, j:isize) {
        self.i = (self.i as isize + i) as usize;
        self.j = (self.j as isize + j) as usize;
    }

    pub fn visible(&self, map: &Map) -> bool {
        map.tile(self.i,self.j).visible()
    }

    pub fn automove(&mut self, dmap: &DijkstraMap) {
        let (i,j) = dmap.get_next_step((self.i,self.j));
        self.i = i;
        self.j = j;
    }

    pub fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer<Resources>, Output>, view: math::Matrix2d, map: &Map) {
        if self.visible(map) {self.sprite.render(self.x(),self.y(),g,view)}
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