mod day1;

use crate::day1::{day1part1, day1part2};
use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
    let s = read_to_string("input/day1.txt");
    match s {
        Ok(s) => {
            println!("{}", day1part1(&s));
            println!("{}", day1part2(&s))
        }
        Err(e) => println!("{}", e),
    }
}
