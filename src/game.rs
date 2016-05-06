///This file stores the code for the game object and player controls

//use std::collections::HashMap;
//use gfx_device_gl::Resources;
use piston_window::*;
use dungeon::*;
use object::*;
//use sprite::Sprite;


enum Command {
    None,               //For when game is waiting for player to issue instruction
    Move(isize,isize),
    Automove, //testing dijkstra map
}

pub struct Game {
    player: Creature,
    command: Command,
    map: Map,
    monsters: Vec<Creature>,
    //graphics: HashMap<&'static str,Texture<Resources>>, //Game loads all graphics on startup then stores them here.
}



impl Game {
    pub fn new(w:&PistonWindow) -> Game {
        Game { 
            player : Creature::new("Player",(1,1),w,"player.png",20,Behavior::Player), 
            command : Command::None,
            map: Map::new(w,42),
            monsters: vec![Creature::new("Nyancat",(3,3),w,"nyancat.png",20,Behavior::Coward)]
        }
    }

    pub fn on_load(&mut self, w: &PistonWindow)  {

        //Load all the graphics
        //self.graphics.insert("player",load_sprite(w, "player.png"));
        //self.graphics.insert("wall",load_sprite(w, "wall.png"));
        
        //self.player.set_sprite(load_sprite(w,"player.png"));
        /*match self.graphics.get("player") {
            Some(sprite) => self.player.set_sprite(sprite.copy()),
            _ => unreachable!(),
        }*/
        
    }
    pub fn on_update(&mut self, upd: UpdateArgs) { //This function is called each turn
        use dijkstra_map::DijkstraMap;
        //Use a bool to check whether the player did anything
        let mut player_acted = true;
        //Handle player action
        match self.command {
            Command::Move(i,j) => self.player.object.mov(&self.map,i,j),
            Command::Automove => {
                let dmap = self.map.get_dijkstra_map(vec![(1,1)]);
                self.player.object.automove(&dmap)
            }, //for testing dijkstra
            _ => player_acted = false,
        };

        if player_acted { 
            self.command = Command::None;
            //local fear map
            let dmapshort = self.map.get_dijkstra_map(vec![(self.player.object.i,self.player.object.j)])*(-2.0);
            //make map of unseen squares
            let mut goals = vec![];
            for j in 0..self.map.grid.len() {
                for i in 0..self.map.grid[0].len() {
                    match los(&self.map,self.player.object.i as isize,self.player.object.j as isize,i as isize,j as isize) {
                        Some((x,y)) if x==i && y==j => (),
                        _ => goals.push((i,j)),
                    }
                }
            }
            //make map of locations 10 tiles away from player
            let dmapfov= self.map.get_dijkstra_map(goals)*0.5;

            let mut goals = vec![];
            for j in 0..self.map.grid.len() {
                for i in 0..self.map.grid[0].len() {
                    if (i as isize - self.player.object.i as isize).abs() >= 10 && (j as isize - self.player.object.j as isize).abs() >= 6 {goals.push((i,j))}
                }
            }
            let dmaplong = self.map.get_dijkstra_map(goals)*0.5;

            let dmap = dmapshort+dmaplong+dmapfov;

            //Handle monster actions
            for monster in self.monsters.iter_mut() {
            //for testing, create a dijkstra map with the player at the center
            monster.object.automove(&dmap);

        } //Clear the active command

        
        }

    }
    pub fn on_draw(&mut self, ren: RenderArgs, e: PistonWindow) {//, glyphs: &mut Glyphs) {
        //use find_folder::Search;
        //use piston_window::*;
        e.draw_2d(|c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            //let view = c
            //let view = c.transform.trans(0.0,0.0);
            //let center = c.transform.trans((ren.width / 2) as f64, (ren.height / 2) as f64);
            let view = c.transform.trans((ren.width / 2) as f64-self.player.object.x(),(ren.height / 2) as f64-self.player.object.y());
            //self.player.render(g, center);

            self.map.render(self.player.object.i, self.player.object.j, g, view);
            self.player.object.render(g, view);

            //render monsters
            for monster in self.monsters.iter() {
                monster.object.render(g,view);
            }
        });
    }
    pub fn on_input(&mut self, inp: Input) {
        match inp {
            Input::Press(key) => {
                match key {
                    //Arrow keys
                    Button::Keyboard(Key::Up) => self.command = Command::Move(0,-1),
                    Button::Keyboard(Key::Down) => self.command = Command::Move(0,1),
                    Button::Keyboard(Key::Left) => self.command = Command::Move(-1,0),
                    Button::Keyboard(Key::Right) => self.command = Command::Move(1,0),
                    //Numpad keys
                    Button::Keyboard(Key::NumPad1) => self.command = Command::Move(-1,1),
                    Button::Keyboard(Key::NumPad2) => self.command = Command::Move(0,1),
                    Button::Keyboard(Key::NumPad3) => self.command = Command::Move(1,1),
                    Button::Keyboard(Key::NumPad4) => self.command = Command::Move(-1,-0),
                    Button::Keyboard(Key::NumPad5) => self.command = Command::Move(0,0),
                    Button::Keyboard(Key::NumPad6) => self.command = Command::Move(1,0),
                    Button::Keyboard(Key::NumPad7) => self.command = Command::Move(-1,-1),
                    Button::Keyboard(Key::NumPad8) => self.command = Command::Move(0,-1),
                    Button::Keyboard(Key::NumPad9) => self.command = Command::Move(1,-1),
                    //Automated movement
                    Button::Keyboard(Key::O) => self.command = Command::Automove,
                    _ => {}
                }
            }
            _ => {}
        }
    }
}