use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

pub fn day6part1(input: &str) -> u32 {
    let mut index = 4;
    for (first, second, third, fourth) in input.chars().tuple_windows() {
        if first != second
            && first != third
            && first != fourth
            && second != third
            && second != fourth
            && third != fourth
        {
            return index;
        }
        index += 1;
    }
    index
}

pub fn day6part2(input: &str) -> u32 {
    let mut previous: VecDeque<char> = VecDeque::new();
    let mut chars = input.chars();
    for _ in 0..14 {
        previous.push_back(chars.next().unwrap());
    }
    let mut index = 14;
    for char in chars {
        let set: HashSet<char> = HashSet::from_iter(previous.iter().cloned());
        if set.len() == 14 {
            return index;
        }
        previous.pop_front();
        previous.push_back(char);
        index += 1;
    }
    index
}

#[cfg(test)]
mod test {
    use crate::day6::{day6part1, day6part2};
    use std::fs::read_to_string;

    #[test]
    fn part1test() {
        let s = read_to_string("test_input/day6.txt").expect("Unable to open file.");
        assert_eq!(day6part1(&s), 7);
    }

    #[test]
    fn part2test() {
        let s = read_to_string("test_input/day6.txt").expect("Unable to open file.");
        assert_eq!(day6part2(&s), 19);
    }
}
