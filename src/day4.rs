fn parse_line(pairing: &str) -> (u32, u32, u32, u32) {
    let (first, second) = pairing.split_once(',').expect("Unable to parse line.");
    let first_assignment = first
        .split_once('-')
        .expect("Unable to parse first assignment.");
    let second_assignment = second
        .split_once('-')
        .expect("Unable to parse second assignment.");
    (
        first_assignment.0.parse().unwrap(),
        first_assignment.1.parse().unwrap(),
        second_assignment.0.parse().unwrap(),
        second_assignment.1.parse().unwrap(),
    )
}

fn check_overlap(
    first_lower: u32,
    first_upper: u32,
    second_lower: u32,
    second_upper: u32,
    second_outer_bound: u32,
    first_outer_bound: u32,
) -> bool {
    (first_lower <= second_lower && first_upper >= second_outer_bound)
        || (second_lower <= first_lower && second_upper >= first_outer_bound)
}

pub fn day4part1(input: &str) -> u32 {
    let lines = input.lines();
    let mut score = 0;
    for pairing in lines {
        let (first_lower, first_upper, second_lower, second_upper) = parse_line(pairing);
        if check_overlap(
            first_lower,
            first_upper,
            second_lower,
            second_upper,
            second_upper,
            first_upper,
        ) {
            score += 1;
        }
    }
    score
}

pub fn day4part2(input: &str) -> u32 {
    let lines = input.lines();
    let mut score = 0;
    for pairing in lines {
        let (first_lower, first_upper, second_lower, second_upper) = parse_line(pairing);
        if check_overlap(
            first_lower,
            first_upper,
            second_lower,
            second_upper,
            second_lower,
            first_lower,
        ) {
            score += 1;
        }
    }
    score
}

#[cfg(test)]
mod test {
    use crate::day4::{day4part1, day4part2};
    use std::fs::read_to_string;

    #[test]
    fn part1test() {
        let s = read_to_string("test_input/day4.txt").expect("Unable to open file.");
        assert_eq!(day4part1(&s), 2);
    }

    #[test]
    fn part2test() {
        let s = read_to_string("test_input/day4.txt").expect("Unable to open file.");
        assert_eq!(day4part2(&s), 4);
    }
}
