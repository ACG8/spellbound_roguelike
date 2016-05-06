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


//dirty LOS computer

pub fn fov(map: &Map, x:isize, y:isize, radius: isize) -> Vec<(usize,usize)> {
	let mut output = vec![];
	for i in -radius..radius+1 {
		for j in -radius..radius+1 {
			if i*i+j*j<radius*radius { 
				match los(map,x,y,x+i,y+j) {
					Some((a,b)) => output.push((a,b)),
					_ => (),
				}
				
			}
		}
	};
	output
}

pub fn los(map: &Map, x0:isize, y0:isize, x1:isize, y1:isize) -> Option<(usize,usize)> {
	if x1 < 0 || y1 < 0 || x1 >= map.width() as isize || y1 >= map.height() as isize { return None };
	let dx = x1 - x0;
	let dy = y1 - y0;
	if dx == 0 && dy == 0 { return Some((x0 as usize,y0 as usize)) };
	let sx = match x0 < x1 {
		true => 1,
		false => -1,
	};
	let sy = match y0 < y1 {
		true => 1,
		false => -1,
	};
    // sx and sy are switches that enable us to compute the LOS in a single quarter of x/y plan
    let mut xnext = x0;
    let mut ynext = y0;
    let denom = ((dx * dx + dy * dy) as f64).sqrt();
    while xnext != x1 || ynext != y1 {
        // check map bounds here if needed
        if !map.grid[ynext as usize][xnext as usize].is_transvisible() && !(xnext==x0 && ynext==y0) // or any equivalent
        {
            return Some((xnext as usize, ynext as usize))
        }
        // Line-to-point distance formula < 0.5
        if ((dy * (xnext - x0 + sx) - dx * (ynext - y0)) as f64).abs() / denom < 0.5 { xnext += sx; }
            
        else if ((dy * (xnext - x0) - dx * (ynext - y0 + sy)) as f64).abs() / denom < 0.5 { ynext += sy; }
        else
        {
            xnext += sx;
            ynext += sy;
        }
    }
    Some((x1 as usize, y1 as usize))
}

use dijkstra_map::DijkstraMap;

pub struct Map{
	/// A vector of rows
	pub grid : Vec< Vec< Tile > >,
	size : usize,
	level : usize,
}

impl Map {
	

	pub fn new(w: &PistonWindow, size: usize) -> Map { //size indicates the width and height of the map.
		let terrain = generate_fractal_dungeon(size,size);
		let mut grid = vec![];
		for j in 0..terrain.len() {
			let mut row = vec![];
			for i in 0..terrain[j].len() {
				row.push( Tile::new(w,terrain[j][i].clone(),i,j) );
			}
			grid.push(row);
		}
		Map{
			grid: grid,//generate_dungeon( w, size, 20, 4, 8, 20 ),//rows,
			size: size,
			level: 0,
		}
	}

	pub fn tile(&self, i: usize, j: usize) -> &Tile { &self.grid[j as usize][i as usize] }

	pub fn width(&self) -> usize {
		self.grid[0].len()
	}

	pub fn height(&self) -> usize {
		self.grid.len()
	}

	pub fn render(&self, i: usize, j:usize, g: &mut GfxGraphics<Resources, CommandBuffer<Resources>, Output>, view: math::Matrix2d) {
		for coordinate in fov(self,i as isize,j as isize,14) {
			self.grid[coordinate.1][coordinate.0].render(g,view)
		}
		/*for row in self.grid.iter() {
			for tile in row.iter() {
				tile.render(g,view);
			}
		}*/
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


fn generate_fractal_dungeon(width:usize,height:usize) -> Vec<Vec<TerrainType>> {
	//Initialize dungeon to walls with floor in middle
	let mut dungeon = vec![];
		for n in 0..height { // iterate over the rows
			let mut row = vec![];
			// iterate over the tiles
			for m in 0..width { 
				match (m,n) {
					(m,n) if m==0 || n==0 || m==width-1 || n==height-1 => row.push( TerrainType::Wall ),
					_ => row.push( TerrainType::Floor ),
				}
			};
			dungeon.push(row);
		};
	/// Define recursive splitting function
	/// This function takes a rectangle and splits it into two smaller rectangles
	/// by drawing a line through it. It places up to 2 doors and 1 window on that line.
	/// The recursion terminates once the rectangles reach size 3.
	/// Lines are only drawn on even numbered rows/columns
	fn rec_split(d: &mut Vec<Vec<TerrainType>>, x0:usize, x1:usize, y0:usize, y1:usize) {
		use rand::distributions::{IndependentSample,Range};
		use rand;

		let width = x1-x0+1;
		let height = y1-y0+1;
		// First, exit if the rectangle is too small
		if width <= 5 || height <= 5 { return; }
		// Split rectangle across longer axis

		match width < height {
			true => { // draw horizontal line. First, select a random row
				//Create rng
				let range = Range::new(1,height/2-1);
				let mut rng = rand::thread_rng();
				let mut row = y0+4;
				//Select a point.
				let mut splitpoints = vec![];
				for y in y0+2..y1-2 { if y%2==0 { splitpoints.push(y) } };
				rand::thread_rng().shuffle(&mut splitpoints);
				loop {
					match splitpoints.pop() {
						Some(y) => match (d[y][x0].clone(),d[y][x1].clone()) {
							(TerrainType::Wall,TerrainType::Wall) => {
								for x in x0..x1 { 
									row=y; 
									d[y][x] = TerrainType::Wall;
								};
								break; 
							},
							_ => (),
						},
						_ => return,
					}
				}
				//Then we randomly place up to 2 doors and 1 window on the new wall
				//First, get a vector of even locations that are free
				let mut locations = vec![];
				let mut terrains = vec![TerrainType::Door,TerrainType::Door,TerrainType::Window];
				for x in x0+2..x1-2 { if x%2==0 { locations.push(x) } };
				//Then shuffle it. Pop the first three elements (up to) and turn them into doors and a window
				rand::thread_rng().shuffle(&mut locations);
				for _ in 0..3 {
					let terrain_type = match terrains.pop() {
						Some(t) => t,
						None => unreachable!(),
					};
					match locations.pop() {
						Some(x) => d[row][x] = terrain_type,
						None => break,
					};
				}
				// Now call this function on each of the two smaller rooms
				
				rec_split(d, x0, x1, y0, row);
				rec_split(d, x0, x1, row, y1);
			}
			false => { // draw vertical line. First, select a random column
				//Create rng
				let range = Range::new(1,width/2-1);
				let mut rng = rand::thread_rng();
				let mut column = x0 + 4;
				//Select a point.
				let mut splitpoints = vec![];
				for x in x0+2..x1-2 { if x%2==0 { splitpoints.push(x) } };
				rand::thread_rng().shuffle(&mut splitpoints);
				loop {
					match splitpoints.pop() {
						Some(x) => match (d[y0][x].clone(),d[y1][x].clone()) {
							(TerrainType::Wall,TerrainType::Wall) => {
								for y in y0..y1 { 
									column = x;  
									d[y][x] = TerrainType::Wall;
								};
								break;
							},
							_ => (),
						},
						_ => return,
					}
				}
				//Then we randomly place up to 2 doors and 1 window on the new wall
				//First, get a vector of even locations that are free
				let mut locations = vec![];
				let mut terrains = vec![TerrainType::Door,TerrainType::Door,TerrainType::Window];
				for y in y0+2..y1-2 { if y%2==0 { locations.push(y) } };
				//Then shuffle it. Pop the first three elements (up to) and turn them into doors and a window
				rand::thread_rng().shuffle(&mut locations);
				for _ in 0..3 {
					let terrain_type = match terrains.pop() {
						Some(t) => t,
						None => unreachable!(),
					};
					match locations.pop() {
						Some(y) => d[y][column] = terrain_type,
						None => break,
					};
				}
				// Now call this function on each of the two smaller rooms
				rec_split(d, x0, column, y0, y1);
				rec_split(d, column, x1, y0, y1);
			}
		}
	}
	rec_split(&mut dungeon,0,width-1,0,height-1);
	dungeon
}


pub enum TerrainType {
	Wall,
	Floor,
	Window,
	Door,
}

impl Clone for TerrainType {
	fn clone(&self) -> TerrainType {
		match *self {
			TerrainType::Wall => TerrainType::Wall,
			TerrainType::Floor => TerrainType::Floor,
			TerrainType::Window => TerrainType::Window,
			TerrainType::Door => TerrainType::Door,
		}
	}
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
	pub fn new(w: &PistonWindow, terrain: TerrainType, i:usize, j: usize) -> Tile {
		match terrain {
			TerrainType::Wall => Tile {
				name : "wall".to_string(),
				sprite: Sprite::new(w,"wall.png"),
				passable : false,
				transvisible: false,
			    i: i,
			    j: j,
			},
			TerrainType::Floor => Tile {
				name : "floor".to_string(),
				sprite: Sprite::new(w,"floor.png"),
				passable : true,
				transvisible: true,
			    i: i,
			    j: j,
			},
			TerrainType::Door => Tile {
				name : "door".to_string(),
				sprite: Sprite::new(w,"door.png"),
				passable : true,
				transvisible: false,
			    i: i,
			    j: j,
			},
			TerrainType::Window => Tile {
				name : "window".to_string(),
				sprite: Sprite::new(w,"window.png"),
				passable : false,
				transvisible: true,
			    i: i,
			    j: j,
			},
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