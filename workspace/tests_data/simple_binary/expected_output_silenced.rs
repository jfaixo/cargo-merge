pub mod hello_world {

pub fn hello_world() {
    println!("Hello, world!");
}

}

use crate::hello_world::hello_world;

fn main() {
    hello_world();
}

