fn detect_compartment(rucksack: &str) -> (&str, &str) {
  let median = rucksack.len() / 2;
  let (first, last) = rucksack.split_at(median);
  (first, last)
}

fn common_char(x: &str, y: &str) -> char {
  let mut result: char = 'a';

  for c in x.chars() {
    if y.contains(c) {
      result = c;
    }
  }

  result
}

fn find_priority(c: char) -> usize {
  let for_lower = |lower_c: char| {
    let mut priority = 0;
    for p in 'a'..='z' {
      priority += 1;
  
      if lower_c == p {
        return priority;
      }
    }

    priority
  };

  let for_upper = |upper_c: char| {
    let mut priority = 27;
    for p in 'A'..='Z' {
      if upper_c == p {
        return priority;
      }
      priority += 1;
    }

    priority
  };

  if c.is_lowercase() {
    for_lower(c)
  } else {
    for_upper(c)
  }
}

pub fn exec(input: String) -> usize {
  let result:usize = input.lines().map(|c| {
    let (l,f) = detect_compartment(c);
    let common_c = common_char(l,f);
    find_priority(common_c)
  }).sum();

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn split_compartment() {
    let (f, l) = detect_compartment("vJrwpWtwJgWrhcsFMMfFFhFp");
    assert_eq!("vJrwpWtwJgWr", f);
    assert_eq!("hcsFMMfFFhFp", l);

    let (f, l) = detect_compartment("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
    assert_eq!("jqHRNqRjqzjGDLGL", f);
    assert_eq!("rsFMfFZSrLrFZsSL", l);

    let (f, l) = detect_compartment("PmmdzqPrVvPwwTWBwg");
    assert_eq!("PmmdzqPrV", f);
    assert_eq!("vPwwTWBwg", l);
  }

  #[test]
  fn find_common_char() {
    let (f, l) = detect_compartment("vJrwpWtwJgWrhcsFMMfFFhFp");
    let c = common_char(f, l);
    assert_eq!('p', c);

    let (f, l) = detect_compartment("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
    let c = common_char(f, l);
    assert_eq!('L', c);

    let (f, l) = detect_compartment("PmmdzqPrVvPwwTWBwg");
    let c = common_char(f, l);
    assert_eq!('P', c);
  }

  #[test]
  fn what_priority() {
    assert_eq!(16, find_priority('p'));
    assert_eq!(38, find_priority('L'));
    assert_eq!(42, find_priority('P'));
  }
}