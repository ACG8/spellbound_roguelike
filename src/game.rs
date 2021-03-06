//!This file stores the code for the game object and player controls

use piston_window::*;
use dungeon::*;
use object::*;

enum Command {
    None,               //For when game is waiting for player to issue instruction
    Move(isize,isize),
    Automove,
}

pub struct Game {
    player: Creature,
    map: Map,
    creatures: Vec<Creature>,
}

use dijkstra_map::DijkstraMap;
impl Game {
    pub fn new(w:&PistonWindow) -> Game {
        Game { 
            player : Creature::new((1,1),w,"player.png",Behavior::Player), 
            map: Map::new(w,42),
            creatures: vec![]
        }
    }

    fn spawn_creature(&mut self, w:&PistonWindow,filename:&str,ai:Behavior) {
        use rand;
        use rand::*;
        // Create a list of odd coordinates (even coordinates may have walls)
        let mut coordinates = vec![];
        for j in 0..self.map.grid.len() { for i in 0..self.map.grid[0].len() { if i%2==1 && j%2==1 { coordinates.push((i,j)) } } }
        // Shuffle the list
        rand::thread_rng().shuffle(&mut coordinates);

        //Now go through the list until you find a place to spawn the creature or run out of coordinates.
        loop {
            match coordinates.pop() {
                Some((i,j)) => if self.is_passable(i,j) {
                    self.creatures.push( Creature::new((i,j), w, filename, ai) );
                    break;
                },
                None => break,
            }
        }
    }

    // Function to retrive coordinates of non-terrain obstacles in the dungeon
    fn get_obstacles(&self) -> Vec<(usize,usize)> {
        let mut output = vec![];
        output.push((self.player.object.i,self.player.object.j));
        for n in 0..self.creatures.len() { output.push((self.creatures[n].object.i,self.creatures[n].object.j))}
        output
    }

    // Function to determine if the tile at the given coordinates is passable
    fn is_passable(&mut self, i:usize,j:usize) -> bool {
        self.map.tile(i,j).is_passable()
        /*match self.get_creature(i,j) {
            Some(_) => false,
            None => self.map.tile(i,j).is_passable(),
        }*/
    }

    fn get_dijkstra_map(&self, goals: Vec<(usize,usize)>) -> DijkstraMap {
        use dijkstra_map::DijkstraTile;
        //Create a map of dijkstra tiles, ignoring the locations of creatures
        let mut map: Vec<Vec<DijkstraTile>> = self.map.grid.iter().map(
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

    pub fn on_load(&mut self)  {
        //Initialize vision
        self.map.update_vision((self.player.object.i, self.player.object.j));        
    }

    pub fn on_draw(&mut self, ren: RenderArgs, e: PistonWindow) {//, glyphs: &mut Glyphs) {
        e.draw_2d(|c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            let view = c.transform.trans((ren.width / 2) as f64-self.player.object.x(),(ren.height / 2) as f64-self.player.object.y());

            // render the map
            self.map.render(g, view);
            //render the player
            self.player.object.render(g, view, &self.map);

            //render other creatures
            for creature in self.creatures.iter() {
                creature.object.render(g,view,&self.map);
            }
        });
    }
    pub fn on_input(&mut self, inp: &Input, w: &PistonWindow) {
        let mut command = Command::None;
        match inp {
            &Input::Press(key) => {
                match key {
                    //Arrow keys
                    Button::Keyboard(Key::Up) => command = Command::Move(0,-1),
                    Button::Keyboard(Key::Down) => command = Command::Move(0,1),
                    Button::Keyboard(Key::Left) => command = Command::Move(-1,0),
                    Button::Keyboard(Key::Right) => command = Command::Move(1,0),
                    //Numpad keys
                    Button::Keyboard(Key::NumPad1) => command = Command::Move(-1,1),
                    Button::Keyboard(Key::NumPad2) => command = Command::Move(0,1),
                    Button::Keyboard(Key::NumPad3) => command = Command::Move(1,1),
                    Button::Keyboard(Key::NumPad4) => command = Command::Move(-1,-0),
                    Button::Keyboard(Key::NumPad5) => command = Command::Move(0,0),
                    Button::Keyboard(Key::NumPad6) => command = Command::Move(1,0),
                    Button::Keyboard(Key::NumPad7) => command = Command::Move(-1,-1),
                    Button::Keyboard(Key::NumPad8) => command = Command::Move(0,-1),
                    Button::Keyboard(Key::NumPad9) => command = Command::Move(1,-1),
                    //Automated movement
                    Button::Keyboard(Key::O) => command = Command::Automove,
                    _ => {}
                }
            }
            _ => {}
        }
        //Use a bool to check whether the player did anything
        let mut player_acted = true;
        //Handle player action
        match command {
            // Attempt to move in given direction
            Command::Move(i,j) => {
                let (i0,j0) = self.player.coordinates();
                let (i1,j1) = ((i0 as isize + i) as usize, (j0 as isize + j) as usize);
                // Stop if there are any obstacles in the way
                let mut exists_obstacle = false;
                for (i2,j2) in self.get_obstacles() { if i2==i1 && j2==j1 { exists_obstacle=true; break; } }
                // If the terrain is passable, move there
                if self.is_passable(i1,j1) && !exists_obstacle { self.player.object.mov(i,j) };
            },
            Command::Automove => {
                // Autoexplore the map by pressing "o"
                let mut goals = vec![]; //Goals will be all the unexplored tiles on the map
                for j in 0..self.map.grid.len() { for i in 0..self.map.grid[0].len() { if !self.map.tile(i,j).is_explored() { goals.push((i,j)) } } }
                let unexploredmap = self.get_dijkstra_map(goals).with_obstacles(self.get_obstacles());
                self.player.object.automove(&unexploredmap);
            },
            _ => player_acted = false,
        };

        if player_acted {
            // First, recompute vision
            self.map.update_vision((self.player.object.i, self.player.object.j));

            // If there are fewer than 8 monsters, spawn a new one
            if self.creatures.len() < 20 { self.spawn_creature(w,"nyancat.png",Behavior::Coward) };

            // Compute a dijkstramap containing the location of the player. We only need to do this once.
            let player_location = self.get_dijkstra_map(vec![(self.player.object.i,self.player.object.j)]);

            // Compute a dijkstramap with all the tiles the player cannot see. Again, we only need to do this once.
            let mut goals = vec![];
            for j in 0..self.map.grid.len() {
                for i in 0..self.map.grid[0].len() {
                    match los(&self.map,self.player.object.i as isize,self.player.object.j as isize,i as isize,j as isize) {
                        Some((x,y)) if x==i && y==j => (),
                        _ => goals.push((i,j)),
                    }
                }
            }
            let unseen_tiles = self.map.get_dijkstra_map(goals);
            // Handle monster actions
            // I had difficulty here because I was iterating on self.creatures, but this was causing an error inside the loop because I had already borrowed creatures as immutable and was trying to borrow it again as mutable. I solved this by iterating over indices and only accessing a creture when absolutely necessary.
            for n in 0..self.creatures.len() {//creature in self.creatures.iter_mut() {
                // Nyancats are cowardly, so they run away from the player and toward unseen tiles
                // We will take the original maps (which do not change) and update them with obstacles.
                // This is MUCH faster than creating a new map from scratch each time.
                let dmap = &(&player_location*(-1.0)+&unseen_tiles*(0.5)).with_obstacles(self.get_obstacles());
                self.creatures[n].object.automove(&dmap);
            }
        }
    }
}