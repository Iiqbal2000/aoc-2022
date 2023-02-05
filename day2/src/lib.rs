use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Rochambeau{
  Rock = 1,
  Paper = 2,
  Scissor = 3,
}

impl Rochambeau {
    fn from_str(input_str: &str) -> Option<Rochambeau> {
      match input_str {
        "A" | "X" => Some(Rochambeau::Rock),
        "B" | "Y" => Some(Rochambeau::Paper),
        "C" | "Z" => Some(Rochambeau::Scissor),
        _ => None,
      }
    }
}

#[allow(clippy::if_same_then_else)]
impl PartialOrd for Rochambeau {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
      if self == &Rochambeau::Rock && other == &Rochambeau::Scissor {
        Some(Ordering::Greater)
      } else if self == &Rochambeau::Scissor && other == &Rochambeau::Paper {
        Some(Ordering::Greater)
      } else if self == &Rochambeau::Paper && other == &Rochambeau::Rock {
        Some(Ordering::Greater)
      } else if self == &Rochambeau::Scissor && other == &Rochambeau::Rock {
        Some(Ordering::Less)
      } else if self == &Rochambeau::Paper && other == &Rochambeau::Scissor {
        Some(Ordering::Less)
      } else if self == &Rochambeau::Rock && other == &Rochambeau::Paper {
        Some(Ordering::Less)
      } else {
        Some(Ordering::Equal)
      }
  }
}

#[derive(Debug,PartialEq)]
pub struct PlayerScore {
  pub player1: u16,
  pub player2: u16,
}

fn parse(input_str: String) -> (Rochambeau, Rochambeau) {
  let input_str: Vec<&str> = input_str.split(' ').collect();
  
  let player1: Option<Rochambeau> = Rochambeau::from_str(input_str[0]);
  if player1.is_none() {
    panic!("undefined type")
  }

  let player2: Option<Rochambeau> = Rochambeau::from_str(input_str[1]);
  if player2.is_none() {
    panic!("undefined type")
  }

  (player1.unwrap(),player2.unwrap())
}

fn calculate_player_score(score: &mut PlayerScore, player1: Rochambeau, player2: Rochambeau) {
  match player1.partial_cmp(&player2) {
    Some(Ordering::Less) => {
      score.player2 += (player2 as u16) + 6;
      score.player1 += player1 as u16
    },
    Some(Ordering::Greater) => {
      score.player1 += (player1 as u16) + 6;
      score.player2 += player2 as u16
    },
    Some(Ordering::Equal) => {
      score.player1 += (player1 as u16) + 3;
      score.player2 += (player2 as u16) + 3
    },
    _ => ()
  }
}

fn ultra_top_secret_strategy(player1: Rochambeau, player2: Rochambeau) -> (Rochambeau, Rochambeau) {
  // draw
  if player2 == Rochambeau::Paper {
    return  (player1, player1);
  }

  // Lose
  if player2 == Rochambeau::Rock && player1 == Rochambeau::Rock {
    return (player1, Rochambeau::Scissor);
  } else if player2 == Rochambeau::Rock && player1 == Rochambeau::Scissor {
    return  (player1, Rochambeau::Paper);
  }

  // win
  if player2 == Rochambeau::Scissor && player1 == Rochambeau::Scissor {
    return (player1, Rochambeau::Rock)
  } else if player2 == Rochambeau::Scissor && player1 == Rochambeau::Rock {
    return (player1, Rochambeau::Paper)
  }

  (player1, player2)
}

pub fn play(str_shapes: String) -> PlayerScore {
  let mut score = PlayerScore{
    player1: 0,
    player2: 0
  };

  str_shapes.lines().for_each(|line| {
    let (pl1,pl2) = parse(line.to_string());
    
    let (pl1,pl2) = ultra_top_secret_strategy(pl1,pl2);

    calculate_player_score(&mut score, pl1, pl2);
  });

  score
}

#[cfg(test)]
mod tests {
  use std::cmp::Ordering;

  use super::*;

  #[test]
  fn parse_and_compare() {
    let content = "A Y";

    let (pl1, pl2) = parse(content.to_string());
    assert_eq!(Some(Ordering::Less), pl1.partial_cmp(&pl2))
  }

  #[test]
  fn parse_and_compare_them() {
    let contents = "\
A Y
B X
C Z
    ";

    let mut perline_iter = contents.lines();

    let round1 = parse(perline_iter.next().unwrap().to_string());
    let round2 = parse(perline_iter.next().unwrap().to_string());
    let round3 = parse(perline_iter.next().unwrap().to_string());

    assert!(round1.0 < round1.1);
    assert!(round2.0 > round2.1);
    assert!(round3.0 == round3.1);
  }

  #[test]
  #[ignore]
  fn play_with_ultra_top_secret_strategy() {
    let contents = "\
A Y
B X
C Z
    ";

    let mut perline_iter = contents.lines();

    let mut round1 = parse(perline_iter.next().unwrap().to_string());
    round1 = ultra_top_secret_strategy(round1.0, round1.1);

    let mut round2 = parse(perline_iter.next().unwrap().to_string());
    round2 = ultra_top_secret_strategy(round2.0, round2.1);

    let mut round3 = parse(perline_iter.next().unwrap().to_string());
    round3 = ultra_top_secret_strategy(round3.0, round3.1);

    assert!(round1.0 == round1.1);
    assert!(round2.0 > round2.1);
    assert!(round3.0 < round3.1);
  }

  #[test]
  fn play_it() {
    let contents = "\
A Y
B X
C Z
    ";

    let mut score = PlayerScore{
      player1: 0,
      player2: 0,
    };
    
    let mut perline_iter = contents.lines();

    let round1 = parse(perline_iter.next().unwrap().to_string());
    let round2 = parse(perline_iter.next().unwrap().to_string());
    let round3 = parse(perline_iter.next().unwrap().to_string());

    calculate_player_score(&mut score, round1.0, round1.1);
    assert_eq!(1, score.player1);
    assert_eq!(8, score.player2);

    calculate_player_score(&mut score, round2.0, round2.1);
    assert_eq!(9, score.player1);
    assert_eq!(9, score.player2);

    calculate_player_score(&mut score, round3.0, round3.1);
    assert_eq!(15, score.player1);
    assert_eq!(15, score.player2);
  }
}