#[macro_use] extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate tokio;
extern crate reqwest;
extern crate fuse;

pub mod filesystem;

fn main() {
    println!("Hello, world!");
}
