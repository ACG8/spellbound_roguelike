///This file stores the code for the game object and player controls

//use std::collections::HashMap;
//use gfx_device_gl::Resources;
use piston_window::*;
use dungeon::Map;
use object::Object;
//use sprite::Sprite;


enum Command {
    None,               //For when game is waiting for player to issue instruction
    Move(isize,isize),
}

pub struct Game {
    player: Object,
    command: Command,
    map: Map,
    //graphics: HashMap<&'static str,Texture<Resources>>, //Game loads all graphics on startup then stores them here.
}



impl Game {
    pub fn new(w:&PistonWindow) -> Game {
        Game { 
            player : Object::new(w,"player.png"), 
            command : Command::None,
            map: Map::new(w,40), }
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
        //Use a bool to check whether the player did anything
        let mut player_acted = true;
        //Handle player action
        match self.command {
            Command::Move(i,j) => self.player.mov(&self.map,i,j),
            _ => player_acted = false,
        };

        if player_acted { self.command = Command::None } //Clear the active command

        //Handle monster actions

    }
    pub fn on_draw(&mut self, ren: RenderArgs, e: PistonWindow) {//, glyphs: &mut Glyphs) {
        //use find_folder::Search;
        //use piston_window::*;
        e.draw_2d(|c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            //let view = c
            //let view = c.transform.trans(0.0,0.0);
            //let center = c.transform.trans((ren.width / 2) as f64, (ren.height / 2) as f64);
            let view = c.transform.trans((ren.width / 2) as f64-self.player.x(),(ren.height / 2) as f64-self.player.y());
            //self.player.render(g, center);

            self.map.render(g, view);
            self.player.render(g, view);

            // Text test
            /*
            let transform = c.transform.trans(10.0, 100.0);
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                "Hello world!",
                glyphs,
                &c.draw_state,
                transform, 
                g,
            );*/
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
                    _ => {}
                }
            }
            _ => {}
        }
    }
}