use std::io::{self, Read};

use day2::play;
fn main() -> io::Result<()> {
  let mut buffer = String::new();
  let mut stdin = io::stdin();
  stdin.read_to_string(&mut buffer)?;

  let final_score = play(buffer);
  
  println!("total score player 1: {}", final_score.player1);
  println!("total score player 2: {}", final_score.player2);

  Ok(())
}
