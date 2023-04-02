use nom::{
  IResult,
  sequence::{delimited, preceded, tuple},
  character::complete::{char, alpha0, digit0, space0},
  bytes::complete::tag,
  multi::separated_list0,
  branch::alt, combinator::{value}, error::{Error},
};
use std::{collections::HashMap, fmt::Write};

pub fn exec(buf: String) -> String {
  let buf = buf.split("\n\n").collect::<Vec<&str>>();

  let crate_stack_raw = buf[0].split('\n').collect::<Vec<&str>>();

  let mut crates_store: Store = Store::new();

  // feed the store
  for value_line in crate_stack_raw.into_iter() {
    let elems = parse_elements(value_line).unwrap().1;
    crates_store.insert_from_vec(elems);
  }

  let cmd_iter = buf[1].lines();
  for line in cmd_iter {
    crates_store.move_crate(line);
  }

  let mut result = String::new();
  
  let number_of_crates = crates_store.0.len();
  for k in 1..=number_of_crates {
    let crates = crates_store.0.get(&k).expect("fail to get crates");
    let crate_val = crates[crates.len()-1]; 
    
    result.write_str(crate_val);
  }

  result
}

#[derive(Debug)]
pub struct Store<'a>(HashMap<usize, Vec<&'a str>>);

impl<'a> Store<'a> {
  fn new() -> Store<'a> {
    let kv: HashMap<usize, Vec<&str>> = HashMap::new();
    Store(kv)
  }

  fn insert_from_vec(&mut self, elems: Vec<&'a str>) {
    for (i, val) in elems.into_iter().enumerate() {
      let k = i+1;
      
      if !val.is_empty() {
        self.0.entry(k).or_insert(Vec::new()).insert(0, val);
      }      
    }
  }

  // move 1 from 2 to 1
  // len: 1
  // src: 2
  // dst: 1
  fn move_crate(&mut self, cmd: &str) {
    let (_, (len, src, dst)) = tuple(
      (
        parse_crate_len, 
        parse_src_crate, 
        parse_dst_crate
      )
    )(cmd).expect("fail to parse len, src, and dst values");    
  
    let converted_len = len
      .parse::<usize>()
      .expect("fail converted len");

    let converted_src = src
      .parse::<usize>()
      .expect("fail to convert src value");

    let converted_dst = dst
      .parse::<usize>()
      .expect("fail to convert dst value");

    let store = &mut self.0;

    let src_elems = store
      .get_mut(&converted_src)
      .expect("fail to get a map value");

    let mut moved_crates = src_elems.split_off(src_elems.len() - converted_len);
    
    moved_crates.reverse();

    store.entry(converted_dst)
      .or_insert(Vec::new())
      .append(&mut moved_crates);

  }
}

fn parse_elements(input: &str) -> IResult<&str, Vec<&str>> {
  let get_alpha_in_bracket = delimited(
    char('['), 
    alpha0,
    char(']')
  );

  let mark_it_is_empty = value("", tag("   "));
  
  let mut parser = separated_list0(
    tag(" "), 
    alt((
      mark_it_is_empty,
      get_alpha_in_bracket,
    )),
  );

  parser(input)
}

fn parse_crate_len(input: &str) -> IResult<&str, &str, Error<&str>> {
  preceded(
    tag("move"),
    delimited(space0, digit0, space0)
  )(input)
}

fn parse_src_crate(input: &str) -> IResult<&str, &str, Error<&str>> {
  preceded(
    tag("from"),
    delimited(space0, digit0, space0)
  )(input)
}

fn parse_dst_crate(input: &str) -> IResult<&str, &str, Error<&str>> {
  preceded(
    tag("to"),
    preceded(space0, digit0)
  )(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_parse_elements() {
    let (_, result) = parse_elements("[Z] [M] [P]").unwrap();
    assert_eq!(vec!["Z", "M", "P"], result);

    let (_, result) = parse_elements("[N] [C]    ").unwrap();
    assert_eq!(vec!["N", "C", ""], result);
  
    let (_, result) = parse_elements("    [D]    ").unwrap();
    assert_eq!(vec!["", "D", ""], result);
  }

  #[test]
  fn should_exec() {
    let payload = "    [D]    
[N] [C]    
[Z] [M] [P]
  1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    assert_eq!("CMZ", exec(String::from(payload)));
  }

  #[test]
  fn should_parse_crate_len() {
    let result = parse_crate_len("move 4 from 2 to 1").unwrap();
    assert_eq!("4", result.1);

    let result = parse_src_crate(result.0).unwrap();
    assert_eq!("2", result.1);

    let result = parse_dst_crate(result.0).unwrap();
    assert_eq!("1", result.1);
  }

}