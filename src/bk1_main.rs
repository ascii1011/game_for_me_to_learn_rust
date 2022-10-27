#![allow(unused)]
// drug wars


//use core::mem;
use std::{io};
use std::cmp::Ordering;
use std::thread::current;

use std::collections::HashMap;

//random
use rand::{thread_rng,Rng};

// finer grained rand
use rand::{SeedableRng,rngs::StdRng};

//trying for converting
use std::convert::{TryFrom,TryInto};

//used to build boilerplate formatted output for a type
use std::fmt::Display;

// no built in len/count for enums so importing these
// CityNames::iter().count()
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};
//use std::intrinsics;



/* ##################### start city test stuff ############## */
#[derive(Debug, EnumCountMacro, EnumIter)]
enum CityNames {
    CA,
    WA,
    NY,
    NJ,
    CO,
    KS,
    TX,
}
/*
Cape Neddick
Dakota Ridge
Lankin
Plevna
Evans Mills
Ama
Cool Valley
East Shore
Lucien
Hollandale

let total_city_count = CityNames.index;
*/
struct CityAttributes {
    name: CityNames,
    drugs: String,
}

impl std::fmt::Display for CityAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}[stuff:{}]", self.drugs, self.drugs)
    }
}

fn get_city() {


    let current_city= CityAttributes {
        name: CityNames::NY,
        drugs: String::from("some stuff"),
    };
}

/* ##################### end city test stuff ############## */


/* ##################### start random test stuff ############## */
fn get_city_names(rand_num: usize) -> Vec<usize>{
    let mut rng = thread_rng();
    //let mut rng = thread_rng();
    let x: u32 = rng.gen();
    println!("{}", x);
    println!("{:?}", rng.gen::<(f64, bool)>());
    let results: Vec<usize> = rand::seq::index::sample(&mut rng, CityNames::iter().count(), rand_num).into_vec();
    results
}

fn get_random_old(min:u32, max:u32) -> u32 {
    let mut rng = thread_rng();
    let random_num: u32 = rng.gen_range(min..max);
    //let random_num: i16 = rand::thread_rng().gen_range(min..max);
    random_num
}

fn get_random(min:u32, max:u32) -> u32 {
    let u64_seed: u64 = 10;
    let mut rng = StdRng::seed_from_u64(u64_seed);
    let mut random_num: u32 = rng.gen_range(4..6);
    random_num
    
}

fn random_stuff() {

    //let rand_num = get_random(6,CityNames::iter().count());
    //println!("rand amount of cities {}", rand_num);

    //let num_of_cities = rand_num as usize;
    //println!("rand city ids: {:?}", get_city_names(num_of_cities));
}

/* ##################### end random test stuff ############## */


let citynames: u32 = 10;
//let city_names: [&str; 3] = ["Cape Neddick","Dakota Ridge","Lankin"];

//Eq requires that you dervice PartialEq on the type.
#[derive(PartialEq, Eq, Hash)]
struct City<'a> {
    name: &'a str
}

struct CityItems<'a> {
    drugs: &'a str,
    weapons: &'a str,
}


type Cities<'a> = HashMap<City<'a>, CityItems<'a>>;

fn init_cities() {

    let mut cities: Cities = HashMap::new();

    //let c = City {
    //   name: city_name
    //};

}


/*
todos:

init cities, tiers of options, current city

loop
  current = get_current_city or None or Default entry location (jail, or something funny)
  show current city
  //show current drugs
  show other cities


  show nav options
    loop
      buy/sell tier1 items

    loop  
      buy/sell tier2 items
  
    heal
  
    jet to new city
*/

fn main() {
    
    

    println!("Welcome to war dev hell :)");

    println!("names: {:?}", CityNames::iter().count());
}


/* 
#[derive(Debug, PartialEq)]
struct RandomNum(i32);

impl TryFrom<i32> for RandomNum {
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(RandomNum(value))
    }

    type error = ();
}
*/

/* Example of 'generics'
basically this function is saying that we 
want to accept any type (T) that can be converted into u32
TA and TB can be two different types
fn add<TA: Into<f64>, TB: <Into<f64>>(a: TA, b: TB) -> f64 {
    a.into() + b.into()
}
*/