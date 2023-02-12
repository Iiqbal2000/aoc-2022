
// 2-8,3-7
pub fn exec(input: String) -> isize {
  let mut result: isize = 0;

  input.lines().for_each(|line| {
    let pair: Vec<&str> = line.split(',').collect();
  
    let item1: Vec<isize> = contruct_vec(pair[0]);
    let item2: Vec<isize> = contruct_vec(pair[1]);
    if is_overlap(item1, item2) {
      result += 1;
    }
  });
  
  result
}

fn is_overlap(item1: Vec<isize>, item2: Vec<isize>) -> bool {
  if item1.len() > item2.len() {
    for val in item2 {
      if !item1.contains(&val) {
        return false;
      }
    }

    true
  } else {
    for val in item1 {
      if !item2.contains(&val) {
        return false;
      }
    }
    
    true
  }
}

fn contruct_vec(input: &str) -> Vec<isize> {
  let range_str: Vec<&str> = input.split('-').collect();
  let lower: isize = range_str[0].parse().unwrap();
  let upper: isize = range_str[1].parse().unwrap();
  let result: Vec<isize> = (lower..=upper).collect();
  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn contruct() {
    assert_eq!(vec![2,3,4,5,6,7,8], contruct_vec("2-8"));
    assert_eq!(vec![3,4,5,6,7], contruct_vec("3-7"));
  }

  #[test]
  fn overlapping() {
    assert!(is_overlap(vec![2,3,4,5,6,7,8], vec![3,4,5,6,7]));
    assert!(is_overlap(vec![6], vec![4,5,6]));
    assert!(!is_overlap(vec![2,3], vec![4,5]));
  }
}