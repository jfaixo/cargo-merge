pub mod lib_and_bin {
pub mod hello_world {

pub fn hello_world() {
    println!("Hello, world!");
}

}

}
use lib_and_bin::hello_world::hello_world;

fn main() {
    hello_world();
}

