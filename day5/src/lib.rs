use nom::{
  IResult,
  sequence::{delimited, terminated, preceded, tuple},
  // see the "streaming/complete" paragraph lower for an explanation of these submodules
  character::complete::{char, alpha0, multispace0, digit0, space0},
  character::is_alphabetic,
  bytes::complete::tag,
  bytes::complete::is_not,
  multi::separated_list0,
  multi::many0,
  branch::alt, Parser, combinator::{value, map}, error::{ParseError, Error},
};
use std::collections::HashMap;

pub fn exec(buf: String) -> &'static str {
  let buf = buf.split("\n\n").collect::<Vec<&str>>();

  let header = buf[0].split('\n').collect::<Vec<&str>>();
  let mut store: Store = Store::from_elems(parse_elements(header.first().unwrap()).unwrap().1);

  // feed the store
  for value_line in header[0..header.len()-1].iter().copied() {
    let elems = parse_elements(value_line).unwrap().1;
    store.insert_from_vec(elems);
  }

  for line in buf[1].lines() {
    store.move_crates(line);
  }

  ""
}

#[derive(Debug)]
pub struct Store<'a>(HashMap<usize, Vec<&'a str>>);

impl<'a> Store<'a> {
  fn from_elems(elems: Vec<&'a str>) -> Store<'a> {
    let mut items: Vec<(usize, Vec<&'a str>)> = vec![];
    
    for k in 1..=elems.len() {
      items.push((k.to_owned(), vec![]))
    }

    let store = HashMap::from_iter(items.into_iter());
    Store(store)
  }

  fn insert_from_vec(&mut self, elems: Vec<&'a str>) {
    for (i, val) in elems.into_iter().enumerate() {
      let k = i+1;
      
      if !val.is_empty() {
        self.0.entry(k).or_insert(Vec::new()).push(val);
      }      
    }
  }

  // move 1 from 2 to 1
  // len: 1
  // src: 2
  // dst: 1
  fn move_crates(&mut self, cmd: &str) {
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

    let elems = self.0.get(&converted_src)
      .expect("fail to get a map value");

    let elems = &mut elems[0..converted_len].to_vec();

    self.0.entry(converted_dst)
      .or_insert(Vec::new())
      .append(elems);
    

    let x = self.0.get_mut(&converted_src)
      .expect("fail to get a map value");

    for _i in 1..=converted_len {
      x.pop();
    }

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

fn parse_keys(input: &str) -> IResult<&str, Vec<&str>> {
  many0(
    alt((
        delimited(
            tag(" "),
            digit0,
            tag("  ")
        ),
        delimited(
            tag(" "),
            digit0,
            tag(" ")
        ),
    ))
    )(input)
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
  fn should_parse_keys() {
    let (_, result) = parse_keys(" 1   2   3 ").unwrap();

    assert_eq!(vec!["1", "2", "3"], result);
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

    assert_eq!("", exec(String::from(payload)));
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