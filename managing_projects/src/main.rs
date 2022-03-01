use rand::Rng;
use std::collections::HashMap;
// import multiple things from same library...
// use std::{cmp::Ordering, io};
use std::io::{self, Write};
// use glob operator to import all
use std::collections::*;

fn main() {
    println!("Hello, world!");
    let secret_number: u8 = rand::thread_rng().gen_range(1..101);
    println!("secret_number: {}", secret_number);
}
