/*! Dijkstra algorithm

 As input, accepts a vector of rows, where each row contains one of three values:
 - Impassable
 - Passable
 - Goal

 As output, it returns a dijkstra map object, where each row is one of two values:
 - Impassable
 - Value(int), where int is the number of steps from a goal

*/

pub enum DijkstraTile {
	Impassable,
	Passable,
	Goal,
	Value(f64),
}

// We will need to duplicate tiles, so we will implement clone here.
impl Clone for DijkstraTile {
	fn clone(&self) -> DijkstraTile {
		match * self { 
			DijkstraTile::Impassable => DijkstraTile::Impassable,
			DijkstraTile::Passable => DijkstraTile::Passable,
			DijkstraTile::Goal => DijkstraTile::Goal,
			DijkstraTile::Value(n) => DijkstraTile::Value(n),
		}
	}
}

pub struct DijkstraMap { map: Vec<Vec<DijkstraTile>> }

impl Clone for DijkstraMap {
	fn clone(&self) -> DijkstraMap {
		DijkstraMap { map: self.map.clone()}
	}
}

impl DijkstraMap {
	//constructor function for a brand new map
	pub fn new(map: &Vec<Vec<DijkstraTile>>) -> DijkstraMap {
		// Create an output map filled with impassable tiles
		let mut output = vec![vec![DijkstraTile::Impassable; map[0].len()];map.len()];

		// First, create 3 boxes
		let mut current_nodes = vec![];
		let mut next_nodes = vec![];
		let mut unvisited = vec![];
		//Fill the current_nodes box with goals and the unvisited box with passables
		for row_index in 0..map.len() {
			for column_index in 0..map[row_index].len() {
				match map[row_index][column_index] {
					DijkstraTile::Goal => current_nodes.push((column_index,row_index)),
					DijkstraTile::Passable => unvisited.push((column_index,row_index)),
					DijkstraTile::Value(_) => unvisited.push((column_index,row_index)),
					_ => (),
				}
			}
		}
		// Until we run out of current nodes, use dijkstra's algorithm to label map current_nodes
		let mut distance = 0isize;
		while current_nodes.len() > 0 {
			// First, iterate over current nodes
			for (i,j) in current_nodes {
				//Set the corresponding output to a tile with value distance
				output[j][i] = DijkstraTile::Value(distance as f64);
				// Then move all adjacent nodes from unvisited to next_nodes
				for (i2,j2) in unvisited.clone() { if (i as isize - i2 as isize).abs() <= 1 && (j as isize - j2 as isize ).abs() <= 1 { next_nodes.push((i2,j2)) } };
				unvisited.retain(|&(i2,j2)|(i as isize -i2 as isize ).abs() > 1 || (j as isize - j2 as isize ).abs() > 1);
			}
			// Next, move everything from next_nodes into current_nodes
			current_nodes = next_nodes.clone();
			next_nodes = vec![];
			// Finally, increment distance
			distance += 1;
		}

		// Print dijkstra map for debug
		/*
		for row in output.iter() {
			println!("");
			for tile in row.iter() {
				match tile {
					&DijkstraTile::Impassable => print!("#"),
					&DijkstraTile::Value(x) if x < 10 => print!("{}",x),
					&DijkstraTile::Value(x) => print!("*"),
					_ => print!("x"),
				}
			}
		}
		println!("");
		*/

		// Return a dijkstra map
		DijkstraMap { map: output}
	}

	/// Create a dijkstra map from an existing one by marking given coordinates as impassable
	/// Makes it much faster to create new maps from existing ones
	pub fn with_obstacles(&self, coordinates: Vec<(usize,usize)>) -> DijkstraMap {
		let mut new_map = self.clone();
		for (i,j) in coordinates { new_map.map[j][i] = DijkstraTile::Impassable }
		new_map
	}

	// Function to determine where an object should next step on the map
	pub fn get_next_step(&self, coordinates:(usize,usize)) -> (usize,usize) {
		// by default, next step is to do nothing
		let mut next_step = coordinates;
		let mut best_tile = match coordinates { (i,j) => self.map[j][i].clone() };

		// Create a list of adjacent coordinates. We order them so that motion in straight lines is preferred.
		let mut candidates: Vec<(isize,isize)> = vec![(-1,0),(0,1),(1,0),(0,-1),(-1,1),(1,1),(1,-1),(-1,-1)];
		candidates = match coordinates { (i0, j0) => candidates.iter().map(|&(i,j)| (i + i0 as isize, j + j0 as isize)).collect() };

		//match coordinates { (x0,y0) => for x in -1isize..2isize { for y in -1isize..2isize { candidates.push((x+x0 as isize,y+y0 as isize)) } } };

		// Get rid of coordinates that fall outside the dimensions of the map
		candidates.retain(|&(i,j)| i >= 0 && i < self.map[0].len() as isize && j >= 0 && j < self.map.len() as isize );

		// Look through the tiles. If one of them is better than the current candidate, make the replacement
		for (i,j) in candidates {
			match self.map[j as usize][i as usize] {
				DijkstraTile::Value(m) => {
					match best_tile {
						DijkstraTile::Value(n) if n <= m => (),
						_ => {
							next_step = (i as usize, j as usize );
							best_tile = DijkstraTile::Value(m);
						},
					}
				},
				_ => (),
			}
		};
		// Return the best coordinates
		next_step
	}
}


/*
	Here we define infix addition and multiplication for dijkstra maps. We want to allow for the ability to
	operate on dijkstra maps both destructively (by passing the map itself) or non-destructively (by passing 
	a reference). This requires a total of 8 definitions, four for each operation.
*/

///Infix multiplication definition

use std::ops::Mul;
impl <'a> Mul<&'a DijkstraMap> for f64 {
	type Output = DijkstraMap;

	fn mul (self, rhs: &'a DijkstraMap) -> DijkstraMap {
		let mut new_map = rhs.clone();
		for j in 0..new_map.map.len() {
			for i in 0..new_map.map[0].len() {
				match new_map.map[j][i] {
					DijkstraTile::Value(n) => new_map.map[j][i] = DijkstraTile::Value(n*self),
					_ => (),
				};
			};
		};
		new_map
	}
}

impl <'a>Mul<f64> for &'a DijkstraMap {
	type Output = DijkstraMap;

	fn mul (self, rhs: f64) -> DijkstraMap {
		let mut new_map = self.clone();
		for j in 0..new_map.map.len() {
			for i in 0..new_map.map[0].len() {
				match new_map.map[j][i] {
					DijkstraTile::Value(n) => new_map.map[j][i] = DijkstraTile::Value(n*rhs),
					_ => (),
				};
			};
		};
		new_map
	}
}

impl  Mul<DijkstraMap> for f64 {
	type Output = DijkstraMap;

	fn mul (self, rhs: DijkstraMap) -> DijkstraMap {
		let mut new_map = rhs.clone();
		for j in 0..new_map.map.len() {
			for i in 0..new_map.map[0].len() {
				match new_map.map[j][i] {
					DijkstraTile::Value(n) => new_map.map[j][i] = DijkstraTile::Value(n*self),
					_ => (),
				};
			};
		};
		new_map
	}
}

impl Mul<f64> for DijkstraMap {
	type Output = DijkstraMap;

	fn mul (mut self, rhs: f64) -> DijkstraMap {
		for j in 0..self.map.len() {
			for i in 0..self.map[0].len() {
				match self.map[j][i] {
					DijkstraTile::Value(n) => self.map[j][i] = DijkstraTile::Value(n*rhs),
					_ => (),
				};
			};
		};
		self
	}
}

///Infix addition definition
use std::ops::Add;

impl <'a,'b>Add<&'a DijkstraMap> for &'b DijkstraMap {
	type Output = DijkstraMap;

	fn add(self,rhs: &'a DijkstraMap) -> DijkstraMap {
		let mut new_map = self.clone();
		for j in 0..new_map.map.len() {
			for i in 0..new_map.map[0].len() {
				match new_map.map[j][i] {
					DijkstraTile::Value(n) => {
						match rhs.map[j][i] {
							DijkstraTile::Value(m) => new_map.map[j][i] = DijkstraTile::Value(n+m),
							_ => (),
						}
					}
					_ => (),
				};
			};
		};
		new_map
	}
}

impl <'a>Add<&'a DijkstraMap> for DijkstraMap {
	type Output = DijkstraMap;

	fn add(mut self,rhs: &'a DijkstraMap) -> DijkstraMap {
		for j in 0..self.map.len() {
			for i in 0..self.map[0].len() {
				match self.map[j][i] {
					DijkstraTile::Value(n) => {
						match rhs.map[j][i] {
							DijkstraTile::Value(m) => self.map[j][i] = DijkstraTile::Value(n+m),
							_ => (),
						}
					}
					_ => (),
				};
			};
		};
		self
	}
}

impl <'a>Add<DijkstraMap> for &'a DijkstraMap {
	type Output = DijkstraMap;

	fn add(self,rhs: DijkstraMap) -> DijkstraMap {
		let mut new_map = self.clone();
		for j in 0..new_map.map.len() {
			for i in 0..new_map.map[0].len() {
				match new_map.map[j][i] {
					DijkstraTile::Value(n) => {
						match rhs.map[j][i] {
							DijkstraTile::Value(m) => new_map.map[j][i] = DijkstraTile::Value(n+m),
							_ => (),
						}
					}
					_ => (),
				};
			};
		};
		new_map
	}
}

impl Add<DijkstraMap> for DijkstraMap {
	type Output = DijkstraMap;

	fn add(mut self,rhs: DijkstraMap) -> DijkstraMap {
		for j in 0..self.map.len() {
			for i in 0..self.map[0].len() {
				match self.map[j][i] {
					DijkstraTile::Value(n) => {
						match rhs.map[j][i] {
							DijkstraTile::Value(m) => self.map[j][i] = DijkstraTile::Value(n+m),
							_ => (),
						}
					}
					_ => (),
				};
			};
		};
		self
	}
}