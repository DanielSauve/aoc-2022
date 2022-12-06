use regex::{Match, Regex};
use std::collections::{vec_deque, VecDeque};
pub fn day5part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut score = 0;
    let mut stacks: Vec<VecDeque<&str>> = vec![];
    for x in 0..9 {
        stacks.push(VecDeque::new());
    }
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let mut current = line.get(0..3).unwrap();
        let mut i = 0usize;
        let mut curr_char = 0usize;
        loop {
            let thing = current.get(1..2).unwrap();
            let re = Regex::new(r"[A-Z]").unwrap();
            if re.is_match(thing) {
                stacks[i].push_front(thing);
            }
            i += 1;
            curr_char += 4;
            let next = line.get(curr_char..curr_char + 3);
            if next.is_none() {
                break;
            }
            current = next.unwrap();
        }
    }
    for line in lines {
        let mut split = line.split(' ');
        let count: u32 = split.nth(1).unwrap().parse().unwrap();
        let mut from: usize = split.nth(1).unwrap().parse().unwrap();
        let mut to: usize = split.nth(1).unwrap().parse().unwrap();
        to -= 1;
        from -= 1;
        for _ in 0..count {
            let temp = stacks[from].pop_back().unwrap();
            stacks[to].push_back(temp);
        }
    }
    for mut x in stacks {
        print!("{}", x.pop_back().unwrap());
    }
    println!("");
    score
}

pub fn day5part2(input: &str) -> u32 {
    let lines = input.lines();
    let mut score = 0;
    score
}
