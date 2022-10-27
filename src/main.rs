#![allow(unused)]
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};
use ggez;
use ggez::input::keyboard::{self, KeyCode};

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::{io};
use std::collections::HashMap;
use rand::{self, thread_rng,Rng};
//use serde_derive;
use serde::{Deserialize, Serialize};
//use serde_json::{Result, Value};

/* immitation of an old shell game called with a twist */

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub health: f32,
    pub cash: f32,
    pub score: f32,
    pub weapon_markup_min: f32,
    pub weapon_markup_max: f32,
    pub weapons: Vec<Weapon>,
    pub cities: Vec<City>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weapon {
    pub name: String,
    pub price: f32,
}

impl std::fmt::Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "'{}' for ${}", self.name, self.price)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct City {
    pub name: String,
}

fn import_config_file() -> Result<Root, Box<dyn Error>> {
    let json_file_path = Path::new("src/game_config.json");
    let file = File::open(json_file_path)?;
    let reader = BufReader::new(file);
    let config: Root = serde_json::from_reader(reader)?;

    Ok(config)

}

fn get_config() -> Root {
    /* abstract call to retreive config */
    let config = import_config_file().unwrap();
    //println!("{:#?}", config);

    config
}



fn randomize_u32(min: f32, max: f32) -> f32 {
    let mut rng = thread_rng();
    let markup = rng.gen_range(min..max);
    markup
}


fn get_current_weapons(default_weapons: Vec<Weapon>, markup_min:f32, markup_max:f32) -> Vec<Weapon> {

    //println!("config Weapons: {:#?}", config.weapons);
    println!("\nUpdating weapon prices...");

    let mut current_weapons: Vec<Weapon> = default_weapons;

    for w in current_weapons.iter_mut() {
        let mut markup = randomize_u32(markup_min, markup_max);
        println!("updating price for {} (${} + ${} => new price ${}", w.name, w.price, markup, w.price + markup);
        w.price += markup;
        w.price = f32::trunc(w.price * 100.0) / 100.0;
    }
    /*
    for weapon in current_weapons.iter() {
        println!("{}", weapon);
    }
    println!("current Weapons: {:#?}", current_weapons);
    */
    current_weapons
}

fn display_weapons(label: &str, weapons: Vec<Weapon>) {
    println!("\n{} weapons:", label);
    for w in weapons.iter() {
        println!("{}",w);
    }
}


// Your game state struct (class)
struct MyGame {
    default_config: Root,
    current_weapons: Vec<Weapon>,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        let mut default_config: Root = get_config();

        display_weapons("default", default_config.weapons.clone());
        let current_weapons: Vec<Weapon> = get_current_weapons(
            default_config.weapons.clone(),
            default_config.weapon_markup_min,
            default_config.weapon_markup_max
        );

        display_weapons("current", current_weapons.clone());
        //initialize game state (instance)
        MyGame {
            current_weapons,
            default_config
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...


        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);
        // Draw code here...
        //let (screen_w, screen_h) = graphics::drawable_size(ctx);
        //println!("weapon 0: {}", self.current_weapons[0].name);
        let score_text = graphics::Text::new(format!(
            "{}         {}",
            self.current_weapons[0].name, self.current_weapons[1].name
        ));
        //let screen_w = graphics::drawable_size(ctx).0;
        //let screen_w_half = screen_w * 0.5;
        //graphics::draw(ctx, &score_text)?;

        graphics::present(ctx)
    }
}


fn main() -> GameResult {
    println!("\n##########################\nWelcome to city_wars:)");

    // Make a Context.
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("Rusty Dope Warz", "astrov1")
        .build()
        .expect("aieee, could not create ggez context!");

    graphics::set_window_title(&mut ctx, "Rusty Dope Warz");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);

    println!("\n### end main ###");
    Ok(())
    
}
