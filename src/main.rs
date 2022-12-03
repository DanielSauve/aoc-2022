mod day1;
mod day2;

use crate::day1::{day1part1, day1part2};
use crate::day2::{day2part1, day2part2};
use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
    let s = read_to_string("input/day1.txt").expect("Unable to open file.");
    println!("{}", day1part1(&s));
    println!("{}", day1part2(&s));
    let s = read_to_string("input/day2.txt").expect("Unable to open file.");
    println!("{}", day2part1(&s));
    println!("{}", day2part2(&s));
}
