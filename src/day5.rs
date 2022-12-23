use regex::Regex;
use std::collections::VecDeque;

fn parse_header<'a>(
    lines: &'a mut core::str::Lines<'a>,
) -> (Vec<VecDeque<&'a str>>, &'a mut core::str::Lines<'a>) {
    let mut stacks: Vec<VecDeque<&str>> = vec![];
    for _ in 0..9 {
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
    (stacks, lines)
}

fn get_output(stacks: Vec<VecDeque<&str>>) -> String {
    let mut output: String = String::new();
    for mut x in stacks {
        output.push_str(x.pop_back().unwrap());
    }
    output
}

pub fn day5part1(input: &str) -> String {
    let mut lines = input.lines();

    let (mut stacks, lines) = parse_header(&mut lines);
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
    get_output(stacks)
}

pub fn day5part2(input: &str) -> String {
    let mut lines = input.lines();

    let (mut stacks, lines) = parse_header(&mut lines);
    for line in lines {
        let mut split = line.split(' ');
        let count: u32 = split.nth(1).unwrap().parse().unwrap();
        let mut from: usize = split.nth(1).unwrap().parse().unwrap();
        let mut to: usize = split.nth(1).unwrap().parse().unwrap();
        to -= 1;
        from -= 1;
        let mut temp_stack: Vec<&str> = Vec::new();
        for _ in 0..count {
            temp_stack.push(stacks[from].pop_back().unwrap());
        }
        for _ in 0..count {
            stacks[to].push_back(temp_stack.pop().unwrap());
        }
    }
    get_output(stacks)
}

// #[cfg(test)]
// mod test {
//     use crate::day5::{day5part1, day5part2};
//     use std::fs::read_to_string;
//
//     #[test]
//     fn part1test() {
//         let s = read_to_string("test_input/day5.txt").expect("Unable to open file.");
//         assert_eq!(day5part1(&s), "CMZ");
//     }
//
//     #[test]
//     fn part2test() {
//         let s = read_to_string("test_input/day5.txt").expect("Unable to open file.");
//         assert_eq!(day5part2(&s), "MCD");
//     }
// }
