use crate::mod1::mod2::hello_world::hello_world;

pub mod mod1 {
pub mod mod2 {
pub mod hello_world {

pub fn hello_world() {
    println!("Hello, world!");
}

}

}

}

fn main() {
    hello_world();
}

