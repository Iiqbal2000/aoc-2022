use std::io::{self, Read};

use day3::exec;

fn main() {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer);

    let total = exec(buffer);
    println!("{total}");
}
