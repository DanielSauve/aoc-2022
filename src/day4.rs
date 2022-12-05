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

pub fn day4part1(input: &str) -> u32 {
    let lines = input.lines();
    let mut score = 0;
    for pairing in lines {
        let (first_lower, first_upper, second_lower, second_upper) = parse_line(pairing);
        if (first_lower <= second_lower && first_upper >= second_upper)
            || (second_lower <= first_lower && second_upper >= first_upper)
        {
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
        if (first_lower <= second_lower && first_upper >= second_lower)
            || (second_lower <= first_lower && second_upper >= first_lower)
        {
            score += 1;
        }
    }
    score
}
