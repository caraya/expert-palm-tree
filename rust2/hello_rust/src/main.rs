extern crate hello_lib;

use hello_lib::greeter;

fn main() {
    let hello = greeter::Greeter::new("\nHello");
    hello.greet("from Rust!\n");
    println!("I'm a Rustacean in training");
}
