#![type_length_limit="100114985"]
#[macro_use] extern crate lazy_static;

pub mod parser;

use std::env;
use std::fs;

fn main() {
    let working_dir = env::current_dir().unwrap();
    let file_path = working_dir.join("./main.ace");

    let contents = fs::read_to_string(file_path).unwrap();

    println!("{:?}", parser::syntax::parse(contents.as_str()));
}
