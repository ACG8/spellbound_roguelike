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
	Value(isize),
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
				output[j][i] = DijkstraTile::Value(distance);
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
		DijkstraMap { map: output }
	}
	//constructor function to add two maps to make a third
	/*
	pub fn add(map1: &Dijkstra_Map, map2: &Dijkstra_Map) -> Dijkstra_Map {

	}
	//constructor function to scale a map
	pub fn mult(map: &Dijkstra_Map, scalar: isize) -> Dijkstra_Map {
	}*/
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