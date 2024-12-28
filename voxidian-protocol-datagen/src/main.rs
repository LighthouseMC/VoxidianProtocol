extern crate voxidian_protocol;

use std::collections::HashMap;

mod generators;

fn main() {
    println!("Hello, world!");
}

trait DataGenerator {
    type Output;

    fn generate(file_contents: &str) -> HashMap<String, Self::Output>;
}