use piston_window::*;
use rand::*;
use gfx_device_gl::{Resources, Output, CommandBuffer};
use gfx_graphics::GfxGraphics;
use sprite::Sprite;
/// A single game level, represented by a vector of vectors, where each vector is of type tile.

pub struct Map{
	/// A vector of rows
	grid : Vec< Vec< Tile > >,
	size : usize,
	level : usize,
}

impl Map {
	pub fn new(w: &PistonWindow, size: usize) -> Map { //size indicates the width and height of the map.
		/*let mut rows = vec![];
		for n in 0..size { // iterate over the rows
			let mut row = vec![];
			for m in 0..size { // iterate over the columns
				if n == 0 || n == size-1 || m == 0 || m == size-1 {
					row.push( Tile::new(w,Terrain_Type::Wall, m, n) );
				} else {
					row.push( Tile::new(w,Terrain_Type::Floor, m, n) );
				};
			}
			rows.push(row);
		};*/

		// Random map generator goes here
		Map{
			grid: generate_dungeon( w, size, 20, 4, 8, 20 ),//rows,
			size: size,
			level: 0,
		}
	}

	pub fn tile(&self, i: usize, j: usize) -> &Tile { &self.grid[j as usize][i as usize] }

	pub fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer<Resources>, Output>, view: math::Matrix2d) {
		for row in self.grid.iter() {
			for tile in row.iter() {
				tile.render(g,view);
			}
		}
	}
}

struct Rect {
	x1 : usize,
	x2 : usize,
	y1 : usize,
	y2 : usize,
	coord : (usize, usize), //for use in iteration
}

impl Rect {
	pub fn new(x:usize,y:usize,w:usize,h:usize) -> Rect {
		Rect { x1: x, x2: x+w, y1: y, y2: y+h, coord: (x,y) }
	}
	pub fn new_rand(x:usize, y:usize, min:usize, max:usize) -> Rect { //create a rectangle with random size
		use rand::distributions::{IndependentSample,Range};
		use rand;
		let between = Range::new(min,max);
		let mut rng = rand::thread_rng();
		Rect {
			x1: x,
			y1: y,
			x2: x + between.ind_sample(&mut rng),
			y2: x + between.ind_sample(&mut rng),
			coord: (x,y),
		}
	}/*
	pub fn overlaps(&self, other: &Rect) -> bool {
		!( self.x1 > other.x2 || other.x1 > self.x2 || self.y1 > other.y2 || other.y1 > self.y2 )
	}*/

	pub fn touches(&self, other: &Rect) -> bool {
		!( self.x1 > other.x2 + 1 || other.x1 > self.x2 + 1 || self.y1 > other.y2 + 1 || other.y1 > self.y2 + 1 )
	}
}

impl Iterator for Rect {
	//We iterate over x,y coordinates (x first)
	type Item = (usize,usize);

	fn next(&mut self) -> Option<(usize,usize)> {
		let value = self.coord;
		match value {
			(x,y) if x <= self.x2 => {
				self.coord = (x+1,y);
				println!("right {}",value.0);
				Some(value)
			}
			(x,y) if y <= self.y2 => {
				self.coord = (self.x1,y+1);
				println!("down {}",value.0);
				Some(value)
			}
			_ => {
				self.coord = (self.x1,self.y1);
				println!("none: {}",value.0);
				None
			},
		}
	}
}
/*
struct Region { // A contiguous region of coordinates
	pub coordinates: Vec<(usize,usize)>,
}

impl Region {
	pub fn new_empty() -> Region {
		Region { coordinates: Vec::new() }
	}
	pub fn new_merge(reg1:Region,connector:(usize,usize),reg2:Region) -> Region {
		let coordinates = reg1.coordinates.append(reg2.coordinates);
		coordinates.push(connector);
		Region { coordinates: coordinates }
	}
	pub fn new_from_room(room:Rect) -> Region {
		let coordinates = vec![];
		for c in Room { coordinates.push(c) };
		Region { coordinates: coordinates }
	}
	pub fn new_from_point(coordinate:(usize,usize)) -> Region {
		Region { coordinates: vec![coordinate] }
	}
}
*/
fn generate_dungeon(w: &PistonWindow, size: usize, room_attempts: usize, min_room:usize, max_room:usize, extra_connecter_chance: usize) -> Vec< Vec< Tile > > {
	use rand::distributions::{IndependentSample,Range};
	use rand;
	// First, fill the dungeon with solid tiles
	let mut rows = vec![];
		for n in 0..size { // iterate over the rows
			let mut row = vec![];
			// iterate over the tiles
			for m in 0..size { row.push( Tile::new(w,Terrain_Type::Wall, m, n) ); }
			rows.push(row);
		};
	// Create a list of non-overlapping rooms
	let between = Range::new(1,size-1-max_room);
	let mut rng = rand::thread_rng();
	let mut rooms = vec![];
	for i in 0..room_attempts {
		//First room originates at (1,1)
		let (x,y) = match i {
			0 => (1,1),
			_ => (between.ind_sample(&mut rng),between.ind_sample(&mut rng))
		};
		let mut room = Rect::new_rand(x,y,min_room,max_room);
		//Ensure that the rooms don't touch
		if rooms.iter().fold(true,|acc,r| acc && !room.touches(r)) { rooms.push(room); }
	}
	// Carve out the rooms
	for room in rooms {
		//println!("newroom");
		for (i,j) in room {
			rows[j][i] = Tile::new(w,Terrain_Type::Floor, i, j);
		}
	}

	rows
}

pub enum Terrain_Type {
	Wall,
	Floor,
	Window,
	Door,
}

/// A tile of a map
pub struct Tile {
	name : String,
	sprite: Sprite,
	passable : bool,	//can you move onto it?
	transvisible: bool,	//can you see through it?
    pub i: usize,
    pub j: usize,
}

impl Tile {
	pub fn new(w: &PistonWindow, terrain: Terrain_Type, i:usize, j: usize) -> Tile {
		match terrain {
			Terrain_Type::Wall => Tile {
				name : "wall".to_string(),
				sprite: Sprite::new(w,"wall.png"),
				passable : false,
				transvisible: false,
			    i: i,
			    j: j,
			},
			_ => Tile {
				name : "floor".to_string(),
				sprite: Sprite::new(w,"floor.png"),
				passable : true,
				transvisible: true,
			    i: i,
			    j: j,
			}
		}
	}
	pub fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer<Resources>, Output>, view: math::Matrix2d) {
		self.sprite.render(self.x(),self.y(),g,view)
    }

    fn x(&self) -> f64 { (self.i as f64)*32.0 }
    fn y(&self) -> f64 { (self.j as f64)*32.0 }

    pub fn is_passable(&self) -> bool {self.passable}
    pub fn is_transvisible(&self) -> bool {self.transvisible}
}