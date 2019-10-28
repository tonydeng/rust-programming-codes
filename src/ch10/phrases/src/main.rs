extern crate phrases;

use phrases::english::{greetings, farewells};

fn main() {
    println!("Hello in English : {}", greetings::hello());
    println!("Goodbye in English : {}", farewells::goodbye());
}