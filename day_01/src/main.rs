use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
  println!("Advent of Code 2020 - Day 01");
  let input = read_input("input.txt");
  println!("Puzzle 1");
  let result = puzzle_1(&input);
  println!("Puzzle 1 Result: {:?}", result);
  println!("Puzzle 2");
  let result = puzzle_2(&input);
  println!("Puzzle 2 Result: {:?}", result);

}

fn read_input(filename: &str) -> Vec<i32> {
    let fi = File::open(filename).unwrap();
    let reader = BufReader::new(fi);

    let input: Vec<i32> = reader
    .lines()
    .map(|line| {
        let l: i32 = line.unwrap().parse::<i32>().unwrap();
        l
    })
    .collect();

    input
}

fn puzzle_1(input: &Vec<i32>) -> i32 {
    let mut i: usize = 0;
    let mut x: usize = 1;
    let sum: i32 = 2020;
    let mut found: bool = false;

    while !found {

        if x < input.len()-1 && input.get(i).unwrap() + input.get(x).unwrap() == sum {
            println!(" Matching Numbers: {:?}, {:?}", input.get(i).unwrap() , input.get(x).unwrap());
            found = true;
        } else if x == input.len() -1 {
            x = 0;
            i += 1;
        } else {
            x += 1;
        }
    }

    input.get(i).unwrap() * input.get(x).unwrap()

}

fn puzzle_2(input: &Vec<i32>) -> i32 {

    let mut i: usize = 0;
    let mut x: usize = 1;
    let mut z: usize = 2;
    let sum: i32 = 2020;
    let mut found: bool = false;

    while !found {

        if z < input.len()-1 && input.get(i).unwrap() + input.get(x).unwrap() + input.get(z).unwrap() == sum {
            println!(" Matching Numbers: {:?}, {:?}, {:?}", input.get(i).unwrap() , input.get(x).unwrap(), input.get(z).unwrap() );
            found = true;
        } else if z == input.len() -1 {
            z = 0;
            x += 1;
        } else if x == input.len() -1 {
            z = 0;
            x = 0;
            i += 1;
        } else {
            z += 1;
        }
    }

    input.get(i).unwrap() * input.get(x).unwrap() * input.get(z).unwrap()

}