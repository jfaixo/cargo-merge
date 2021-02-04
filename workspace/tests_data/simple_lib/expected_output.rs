pub mod simple_lib {
pub mod hello_world {

pub fn hello_world() {
    println!("Hello, world!");
    eprintln!("yeah");
}

}
pub mod call {
use crate::simple_lib::hello_world::hello_world;

pub fn call() {
    hello_world();
}

}

}
