use std::io::{self, Read};

use day4::{exec, is_overlap1, is_overlap2};

fn main() {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer);

    let total = exec(&buffer, is_overlap1);
    println!("part 1: {total}");

    let total = exec(&buffer, is_overlap2);
    println!("part 2: {total}");
}
