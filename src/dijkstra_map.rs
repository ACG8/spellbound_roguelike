// Dijkstra algorithm

// As input, accepts a vector of rows, where each row contains one of three values:
// - Impassable
// - Passable
// - Goal

// As output, it returns a dijkstra map object, where each row is one of two values:
// - Impassable
// - Value(int), where int is the number of steps from a goal

// 

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

pub struct DijkstraMap {
	map: Vec<Vec<DijkstraTile>>,
	//iter_coord: (usize,usize)
}
/*
impl Iterator for DijkstraMap {
	//We iterate over x,y coordinates (x first)
	type Item = &mut DijkstraTile;

	fn next(&mut self) -> Option<&mut DijkstraTile> {
		let (imax,jmax) = (self.map[0].len() - 1,self.map.len() - 1);
		let iter_coord = self.iter_coord.clone();
		match iter_coord {
			(i,j) if i < imax => {
				self.iter_coord = (i+1,j);
				Some(&mut self.map[j][i])
			}
			(i,j) if j < jmax => {
				self.iter_coord = (0,j+1);
				Some(&mut self.map[j][i])
			}
			(i,j) if i == imax && j==jmax => {
				self.iter_coord = (i+1,j+1);
				Some(&mut self.map[j][i])
			}
			_ => {
				self.iter_coord = (0,0);
				None
			},
		}
		
	}
}
*/
/*
impl IntoIterator for DijkstraMap {
	type Item = &mut DijkstraTile;
	type IntoIter = DijMapIntoIter;

	fn into_iter(self) -> Self::IntoIter {
		DijMapIntoIter { dmap: self, iter_coord: (0,0) }
	}
}

pub struct DijMapIntoIter {
	dmap: DijkstraMap,
	iter_coord: (usize,usize),
}

impl Iterator for DijMapIntoIter {
	//We iterate over x,y coordinates (x first)
	type Item = &mut DijkstraTile;

	fn next(&mut self) -> Option<&mut DijkstraTile> {
		let (imax,jmax) = (self.dmap.map[0].len() - 1,self.dmap.map.len() - 1);
		let iter_coord = self.iter_coord.clone();
		match iter_coord {
			(i,j) if i < imax => {
				self.iter_coord = (i+1,j);
				Some(self.dmap.map[j][i])
			}
			(i,j) if j < jmax => {
				self.iter_coord = (0,j+1);
				Some(self.dmap.map[j][i])
			}
			(i,j) if i == imax && j==jmax => {
				self.iter_coord = (i+1,j+1);
				Some(self.dmap.map[j][i])
			}
			_ => {
				//self.iter_coord = (0,0);
				None
			},
		}
		
	}
}
*/
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
	//constructor function to add two maps to make a third
	
	pub fn add(map1: &DijkstraMap, map2: &DijkstraMap) -> DijkstraMap {
		let mut new_map = map1.clone();
		for j in 0..new_map.map.len() {
			for i in 0..new_map.map[0].len() {
				match new_map.map[j][i] {
					DijkstraTile::Value(n) => {
						match map2.map[j][i] {
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
	//constructor function to scale a map
	pub fn mult(map: &DijkstraMap, scalar: f64) -> DijkstraMap {
		let mut new_map = map.clone();
		for j in 0..new_map.map.len() {
			for i in 0..new_map.map[0].len() {
				match new_map.map[j][i] {
					DijkstraTile::Value(n) => new_map.map[j][i] = DijkstraTile::Value(n*scalar),
					_ => (),
				};
			};
		};
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