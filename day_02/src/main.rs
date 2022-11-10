use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::str::FromStr;

fn main() {
  puzzle_1("input.txt");
  puzzle_2("input.txt");
}


fn puzzle_1(filename: &str) {
  println!("Solving Puzzle 1");

  let fi = File::open(filename).unwrap();
  let reader = BufReader::new(fi);

  let re = Regex::new(r"(\d*)-(\d*)\s(\w):\s(.*)").unwrap();
  let mut count = 0;
  reader.lines()
  .for_each(|line| {
      for cap in re.captures_iter(&line.unwrap()) {

          //Cast variables parsed from Regex
          let min_char = usize::from_str(&cap[1]).unwrap();
          let max_char = usize::from_str(&cap[2]).unwrap();
          let req_char: &str = &cap[3];
          let password: &str = &cap[4];

          let c = password.matches(req_char).count();

          if c >= min_char && c <= max_char {
              count+=1;
          }
      }

  });

  println!("  Total Valid Passwords: {}", count);
}

fn puzzle_2(filename: &str) {
    println!("Solving Puzzle 2");

    let fi = File::open(filename).unwrap();
    let reader = BufReader::new(fi);

    let re = Regex::new(r"(\d*)-(\d*)\s(\w):\s(.*)").unwrap();
    let mut count = 0;
    reader.lines()
    .for_each(|line| {
        for cap in re.captures_iter(&line.unwrap()) {

          //Cast variables parsed from Regex
          let f_pos = usize::from_str(&cap[1]).unwrap()-1;
          let l_pos = usize::from_str(&cap[2]).unwrap()-1;
          let req_char: &char = &cap[3].chars().next().unwrap();
          let password: &str = &cap[4];

          let f_char: char = password.chars().nth(f_pos).unwrap();
          let l_char: char = password.chars().nth(l_pos).unwrap();

          if &f_char == req_char && &l_char != req_char {
              count+=1;
          } else if &f_char != req_char && &l_char == req_char {
              count +=1;
          }
        }
    });

    println!("  Total Valid Passwords: {}", count);
}