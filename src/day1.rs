pub fn day1part1(input: &str) -> i32 {
    let lines = input.lines();
    let mut current_calories = 0;
    let mut max_calories = 0;
    for elf in lines {
        if elf.is_empty() {
            if current_calories > max_calories {
                max_calories = current_calories;
            }
            current_calories = 0;
        } else {
            current_calories += elf.parse::<i32>().unwrap()
        }
    }
    if max_calories > current_calories {
        max_calories
    } else {
        current_calories
    }
}

pub fn day1part2(input: &str) -> i32 {
    let lines = input.lines();
    let mut current_calories = 0;
    let mut calorie_counts: Vec<i32> = std::vec![];
    for elf in lines {
        if elf.is_empty() {
            calorie_counts.push(current_calories);
            current_calories = 0;
        } else {
            current_calories += elf.parse::<i32>().unwrap()
        }
    }
    calorie_counts.push(current_calories);
    calorie_counts.sort();
    calorie_counts.reverse();
    calorie_counts[0] + calorie_counts[1] + calorie_counts[2]
}

#[cfg(test)]
mod test {
    use crate::day1::{day1part1, day1part2};
    use std::fs::read_to_string;

    #[test]
    fn part1test() {
        let s = read_to_string("test_input/day1.txt").expect("Unable to open file.");
        assert_eq!(day1part1(&s), 24000);
    }

    #[test]
    fn part2test() {
        let s = read_to_string("test_input/day1.txt").expect("Unable to open file.");
        assert_eq!(day1part2(&s), 45000);
    }
}
