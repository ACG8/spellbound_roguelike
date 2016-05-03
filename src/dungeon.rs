use piston_window::*;
use rand::*;
use gfx_device_gl::{Resources, Output, CommandBuffer};
use gfx_graphics::GfxGraphics;
use sprite::Sprite;
/// A single game level, represented by a vector of vectors, where each vector is of type tile.
/*
fn rec_shadowcaster(origin: (usize,usize), min_range: usize, max_range: usize, start_slope: f64,end_slope: f64) -> Vec<(usize,usize)> {
	//returns a list of coordinates that are in fov
	//call it with min_range = 1
	
}
*/
use dijkstra_map::DijkstraMap;

pub struct Map{
	/// A vector of rows
	grid : Vec< Vec< Tile > >,
	size : usize,
	level : usize,
}

impl Map {
	

	pub fn new(w: &PistonWindow, size: usize) -> Map { //size indicates the width and height of the map.
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
	//Function that returns a dijkstra map given the input goal cells
	pub fn get_dijkstra_map(&self, goals: Vec<(usize,usize)>) -> DijkstraMap {
		use dijkstra_map::DijkstraTile;
		//Create a map of dijkstra tiles
		let mut map: Vec<Vec<DijkstraTile>> = self.grid.iter().map(
			|row| row.iter().map(
				|tile| match tile.is_passable() {
					true => DijkstraTile::Passable,
					false => DijkstraTile::Impassable,
				}
			).collect()
		).collect();

		//Add the goals. Only goals on passable ground are added.
		for (i,j) in goals {
			match map[j][i] {
				DijkstraTile::Passable => map[j][i] = DijkstraTile::Goal,
				_ => (),
			}
		};

		//Construct and return the dijkstra map
		DijkstraMap::new(&map)
	}

	/*

	pub fn compute_fov(&self, origin: (usize,usize), range: usize) {//-> Vec<(usize,usize)> {
		// Call recursive FOV finder for each direction
		//rec_fov(origin)
		println!("")
	}

	fn rec_fov(&self,origin: (usize,usize), distance: usize, max_distance: usize, min_slope: f64, max_slope: f64, direction: Dir) -> Vec<(usize,usize)> {
		// Get an adjusted origin poin 
		// First, depending on direction, split:
		match direction {
			Dir::Left => {
				//First, find the starting points based on distance
				let min_j = (min_slope * (- distance as f64) + origin.1 as f64) as isize;
				let max_j = (max_slope * (- distance as f64) + origin.1 as f64) as isize;
				let i = -distance as isize + origin.0 as isize;
				//If i is outside the bounds of the screen, return. Adjust j so that they are not outside the bounds of the screen.
				if i < 0 || i > self.grid[0].len() as isize { return vec![] };

			}

			Dir::Right => (),
			Dir::Up => (),
			Dir::Down => (),
		} ;
	}
	*/
}

struct Rect {
	x1 : usize,
	x2 : usize,
	y1 : usize,
	y2 : usize,
	coord : (usize, usize), //for use in iteration
}

enum Dir { //enum for directions
	Left,
	Right,
	Up,
	Down,
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
		let newrect = Rect {
			x1: x,
			y1: y,
			x2: x + between.ind_sample(&mut rng),
			y2: y + between.ind_sample(&mut rng),
			coord: (x,y),
		};
		newrect
	}

	pub fn is_within_distance(&self, other: &Rect, distance: usize) -> bool {//checks whether two rects are within given distance apart
		!( self.x1 > other.x2 + distance || other.x1 > self.x2 + distance || self.y1 > other.y2 + distance || other.y1 > self.y2 + distance )
	}

	pub fn center(&self) -> (usize,usize) {//returns center of the room
		((self.x1+self.x2)/2,(self.y1+self.y2)/2)
	}
}

impl Iterator for Rect {
	//We iterate over x,y coordinates (x first)
	type Item = (usize,usize);

	fn next(&mut self) -> Option<(usize,usize)> {
		let value = self.coord.clone();
		match value {
			(x,y) if x < self.x2 => {
				self.coord = (x+1,y);
				Some(value)
			}
			(x,y) if y < self.y2 => {
				self.coord = (self.x1,y+1);
				Some(value)
			}
			(x,y) if x == self.x2 && y == self.y2 => {
				self.coord = (x+1,y);
				Some(value)
			}
			_ => {
				self.coord = (self.x1,self.y1);
				None
			},
		}
	}
}

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
		if rooms.iter().fold(true,|acc,r| acc && !room.is_within_distance(r,1)) { rooms.push(room); }
	}
	let mut regions = vec![];
	while rooms.len() > 1 {
		let room = match rooms.pop() {
			Some(r) => r,
			None => unreachable!(),
		};
		//Select a random room
		let between = Range::new(0,rooms.len());
		let mut rng = rand::thread_rng();
		let room2 = &rooms[between.ind_sample(&mut rng)];
		//Create two rooms that form a corridor between the rooms
		match room.x1 < room2.x1 {
			true => regions.push(Rect::new(room.center().0,room.center().1,room2.x1-room.x1,0)),
			false => regions.push(Rect::new(room2.center().0,room2.center().1,room.x1-room2.x1,0)),
		};
		match room.y1 < room2.y1 {
			true => regions.push(Rect::new(room.center().0,room.center().1,0,room2.y1-room.y1)),
			false => regions.push(Rect::new(room2.center().0,room2.center().1,0,room.y1-room2.y1)),
		};
		regions.push(room);
	}
	match rooms.pop() {
		Some(room) => regions.push(room),
		_ => unreachable!(),
	}
	// Carve out the rooms
	for room in regions {
		for (i,j) in room {
			rows[j][i] = Tile::new(w,Terrain_Type::Floor, i, j);
		}
	}
	// Carve corridors between the rooms


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