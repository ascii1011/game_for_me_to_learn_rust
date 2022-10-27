#![allow(unused)]
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::{io};
use std::collections::HashMap;
use rand::{thread_rng,Rng};
use serde;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub health: i64,
    pub cash: i64,
    pub weapons: Vec<Weapon>,
    pub cities: Vec<City>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
pub struct Weapon {
    pub name: String,
    pub price: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
pub struct City {
    pub name: String,
}

fn import_config_file() -> Root {
    let json_file_path = Path::new(".game_config.json");
    let file = File::open(json_file_path)?;
    let reader = BufReader::new(file);
    let config: Root = serde_json::from_reader(reader)
        .expect("error while reading or parsing file.");

    Ok(config)

}
fn import_config_str() -> Root {
    let data = r#"
    {
        "health": 100,
        "cash": 10,
        "weapons": [
          {"name": "uzi", "price": 200},
          {"name": "mac", "price": 300},
          {"name": "ak", "price": 450}
        ],
        "cities": [
          {"name": "Neddick"},
          {"name": "Dakota"},
          {"name": "lakapoha"}
        ]
      }"#;
    let root: Root = serde_json::from_str(data).unwrap();
    
    root
}

fn load_weapons() {
    #[derive(PartialEq, Eq, Hash)]
    struct WeaponSpecs<'a> {
        name: &'a str,
    }
    struct WeaponAttributes<'a> {
        retail_price: &'a u32,
    
    }
    type WeaponsDigest<'a> = HashMap<WeaponSpecs<'a>, WeaponAttributes<'a>>;

    let mut weapons: WeaponsDigest = HashMap::new();


    let base_price: u32 = 600;
    let mut rng = thread_rng();
    let random_num: u32 = rng.gen_range(1..100);
    let new_price = base_price + random_num;

    //for (name, price, damage) in weapons_config. {
    //    weapons.insert(WeaponSpecs{name: name}, WeaponAttributes{price: &price});
    // }

    //for (spec, attr) in weapons.iter() {
    //    println!("Weapon specs: {}, price: {}", spec.name, attr.price);
    //}
}

fn load_cities() {
    #[derive(PartialEq, Eq, Hash)]
    struct CitySpecs<'a> {
        name: &'a str
    }

    struct CityAttributes<'a> {
        weapons: &'a str,
    }

    type CitiesDigest<'a> = HashMap<CitySpecs<'a>, CityAttributes<'a>>;

    let mut cities: CitiesDigest = HashMap::new();

    cities.insert(CitySpecs{name:city_names[0]}, CityAttributes{weapons:"cat"});

    for (city, items) in cities.iter() {
        println!("City: {}, items: {}", city.name, items.weapons);
    }
}

fn main() {
    println!("Welcome to city_wars hell :)");

    let config = import_config_file().unwrap();
    println!("{:#?}", config);


    
}
