use std::collections::HashSet;

fn calculate_score(c: &char) -> u32 {
    if c.is_ascii_uppercase() {
        return 27 + *c as u32 - 'A' as u32;
    }
    1 + *c as u32 - 'a' as u32
}

pub fn day3part1(input: &str) -> u32 {
    let lines = input.lines();
    let mut score = 0;
    for rucksack in lines {
        let (left, right) = rucksack.split_at(rucksack.len() / 2);
        let mut chars = HashSet::new();
        left.chars().for_each(|c| {
            chars.insert(c);
        });
        for c in right.chars() {
            if chars.contains(&c) {
                score += calculate_score(&c);
                break;
            }
        }
    }
    score
}

pub fn day3part2(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut score = 0;
    loop {
        let mut first_chars = HashSet::new();
        let mut second_chars = HashSet::new();
        let mut third_chars = HashSet::new();
        let first = lines.next();
        let second = lines.next();
        let third = lines.next();
        if first.is_none() || second.is_none() || third.is_none() {
            break;
        }
        first.unwrap().chars().for_each(|c| {
            first_chars.insert(c);
        });
        second.unwrap().chars().for_each(|c| {
            second_chars.insert(c);
        });
        third.unwrap().chars().for_each(|c| {
            third_chars.insert(c);
        });
        let intersection = first_chars.intersection(&second_chars);
        intersection.for_each(|c| {
            if third_chars.contains(c) {
                score += calculate_score(c)
            }
        });
    }
    score
}

#[cfg(test)]
mod test {
    use crate::day3::{day3part1, day3part2};
    use std::fs::read_to_string;

    #[test]
    fn part1test() {
        let s = read_to_string("test_input/day3.txt").expect("Unable to open file.");
        assert_eq!(day3part1(&s), 157);
    }

    #[test]
    fn part2test() {
        let s = read_to_string("test_input/day3.txt").expect("Unable to open file.");
        assert_eq!(day3part2(&s), 70);
    }
}
