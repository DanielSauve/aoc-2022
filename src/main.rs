mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

use crate::day1::{day1part1, day1part2};
use crate::day2::{day2part1, day2part2};
use crate::day3::{day3part1, day3part2};
use crate::day4::{day4part1, day4part2};
use crate::day5::{day5part1, day5part2};
use crate::day6::{day6part1, day6part2};
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
    let s = read_to_string("input/day4.txt").expect("Unable to open file.");
    println!("Day 4 Part 1: {}", day4part1(&s));
    println!("Day 4 Part 2: {}", day4part2(&s));
    let s = read_to_string("input/day5.txt").expect("Unable to open file.");
    println!("Day 5 Part 1: {}", day5part1(&s));
    println!("Day 5 Part 2: {}", day5part2(&s));
    let s = read_to_string("input/day6.txt").expect("Unable to open file.");
    println!("Day 6 Part 1: {}", day6part1(&s));
    println!("Day 6 Part 2: {}", day6part2(&s));
}
