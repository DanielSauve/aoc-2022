use std::collections::HashSet;

fn compute_tail_coordinate(head: (i64, i64), tail: &mut (i64, i64)) {
    let x_diff = (head.0 - tail.0).abs();
    let y_diff = (head.1 - tail.1).abs();
    if x_diff > 1 {
        if y_diff != 0 {
            tail.1 = head.1;
        }
        tail.0 += if head.0 > tail.0 { 1 } else { -1 }
    } else if y_diff > 1 {
        if x_diff != 0 {
            tail.0 = head.0;
        }
        tail.1 += if head.1 > tail.1 { 1 } else { -1 }
    }
}

fn compute_trail_coordinates(coordinates: &mut Vec<(i64, i64)>) {
    for i in 0..coordinates.len() - 1 {
        compute_tail_coordinate(coordinates[i], &mut coordinates[i + 1]);
    }
}

fn calculate_trail_movement(input: &str, trail_length: usize) -> u32 {
    let mut coordinates: Vec<(i64, i64)> = Vec::new();
    for _ in 0..trail_length {
        coordinates.push((0, 0));
    }
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    for line in input.lines() {
        let mut split = line.split(' ');
        let direction = split.next().unwrap();
        let mut amount: u32 = split.next().unwrap().parse().unwrap();
        match direction {
            "R" => {
                while amount > 0 {
                    coordinates[0].0 += 1;
                    compute_trail_coordinates(&mut coordinates);
                    visited.insert(*coordinates.last().unwrap());
                    amount -= 1;
                }
            }
            "L" => {
                while amount > 0 {
                    coordinates[0].0 -= 1;
                    compute_trail_coordinates(&mut coordinates);
                    visited.insert(*coordinates.last().unwrap());
                    amount -= 1;
                }
            }
            "U" => {
                while amount > 0 {
                    coordinates[0].1 += 1;
                    compute_trail_coordinates(&mut coordinates);
                    visited.insert(*coordinates.last().unwrap());
                    amount -= 1;
                }
            }
            "D" => {
                while amount > 0 {
                    coordinates[0].1 -= 1;
                    compute_trail_coordinates(&mut coordinates);
                    visited.insert(*coordinates.last().unwrap());
                    amount -= 1;
                }
            }
            _ => continue,
        }
    }
    visited.len() as u32
}

pub fn day9part1(input: &str) -> u32 {
    calculate_trail_movement(input, 2)
}

pub fn day9part2(input: &str) -> u32 {
    calculate_trail_movement(input, 10)
}

#[cfg(test)]
mod test {
    use crate::day9::{day9part1, day9part2};
    use std::fs::read_to_string;

    #[test]
    fn part1test() {
        let s = read_to_string("test_input/day9.txt").expect("Unable to open file.");
        assert_eq!(day9part1(&s), 13);
    }

    #[test]
    fn part2test() {
        let s = read_to_string("test_input/day9.txt").expect("Unable to open file.");
        assert_eq!(day9part2(&s), 1);
    }
}
