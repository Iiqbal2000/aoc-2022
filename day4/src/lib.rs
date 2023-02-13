
// 2-8,3-7
pub fn exec(input: &String, is_overlap: fn(Vec<isize>, Vec<isize>) -> bool) -> isize {
  let mut result: isize = 0;

  input.lines().for_each(|line| {
    let pair: Vec<&str> = line.split(',').collect();
  
    let item1: Vec<isize> = construct_vec(pair[0]);
    let item2: Vec<isize> = construct_vec(pair[1]);
    if is_overlap(item1, item2) {
      result += 1;
    }
  });
  
  result
}

// If all elements within a section are present in 
// the corresponding section of its pair, it is considered overlapping.
pub fn is_overlap1(item1: Vec<isize>, item2: Vec<isize>) -> bool {
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

// If an element within a section are present in 
// the corresponding section of its pair, it is considered overlapping.
pub fn is_overlap2(item1: Vec<isize>, item2: Vec<isize>) -> bool {
  if item1.len() > item2.len() {
    for val in item2 {
      if item1.contains(&val) {
        return true;
      }
    }

    false
  } else {
    for val in item1 {
      if item2.contains(&val) {
        return true;
      }
    }
    
    false
  }
}

fn construct_vec(input: &str) -> Vec<isize> {
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
  fn it_produces_vec_from_range_syntax() {
    assert_eq!(vec![2,3,4,5,6,7,8], construct_vec("2-8"));
    assert_eq!(vec![3,4,5,6,7], construct_vec("3-7"));
  }

  #[test]
  fn it_checks_overlap1() {
    assert!(is_overlap1(vec![2,3,4,5,6,7,8], vec![3,4,5,6,7]));
    assert!(is_overlap1(vec![6], vec![4,5,6]));
    assert!(!is_overlap1(vec![2,3], vec![4,5]));
  }

  #[test]
  fn it_checks_overlap2() {
    assert!(!is_overlap2(construct_vec("2-4"), construct_vec("6-8")));
    assert!(!is_overlap2(construct_vec("2-3"), construct_vec("4-5")));
    
    assert!(is_overlap2(construct_vec("5-7"), construct_vec("7-9")));
    assert!(is_overlap2(construct_vec("2-8"), construct_vec("3-7")));
    assert!(is_overlap2(construct_vec("6-6"), construct_vec("4-6")));
  }
}