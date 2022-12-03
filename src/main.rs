mod day1;
mod day2;
mod day3;

use crate::day1::{day1part1, day1part2};
use crate::day2::{day2part1, day2part2};
use crate::day3::{day3part1, day3part2};
use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
    let s = read_to_string("input/day1.txt").expect("Unable to open file.");
    println!("Day 1 Part 1: {}", day1part1(&s));
    println!("Day 1 Part 2: {}", day1part2(&s));
    let s = read_to_string("input/day2.txt").expect("Unable to open file.");
    println!("Day 2 Part 1: {}", day2part1(&s));
    println!("Day 2 Part 2: {}", day2part2(&s));
    let s = read_to_string("input/day3.txt").expect("Unable to open file.");
    println!("Day 3 Part 1: {}", day3part1(&s));
    println!("Day 3 Part 2: {}", day3part2(&s));
}
