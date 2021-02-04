pub mod hello_world {

pub fn hello_world() {
    println!("Hello, world!");
    eprintln!("yeah");
}

}

use crate::hello_world::hello_world;

fn main() {
    hello_world();
}

