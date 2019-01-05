mod sound;

mod performance_group {
    pub use crate::sound::instrument::woodwind;

    pub fn clarinet_trio() {
        woodwind::clarinet();
        woodwind::clarinet();
        woodwind::clarinet();
    }
}

mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

mod menu {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use crate::sound::instrument::woodwind;
use std::collections::HashMap;

use std::fmt::Result;
use std::io::Result as IoResult;

use rand::Rng;

fn main() {
    crate::sound::instrument::woodwind::clarinet();
    sound::instrument::woodwind::clarinet();
    woodwind::clarinet();

    performance_group::clarinet_trio();
    performance_group::woodwind::clarinet();

    let mut v = plant::Vegetable::new("squash");

    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);

    // println!("The ID is {}", v.id);

    let order1 = menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;

    let mut map = HashMap::new();
    map.insert(1, 2);

    let secret_number = rand::thread_rng().gen_range(1, 101);
}
