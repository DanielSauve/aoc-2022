pub fn day1part1(input: &str) -> i32 {
    let split = input.split('\n');
    let mut current_calories = 0;
    let mut max_calories = 0;
    for elf in split {
        if elf.is_empty() {
            if current_calories > max_calories {
                max_calories = current_calories;
            }
            current_calories = 0;
        } else {
            current_calories += elf.parse::<i32>().unwrap()
        }
    }
    max_calories
}

pub fn day1part2(input: &str) -> i32 {
    let split = input.split('\n');
    let mut current_calories = 0;
    let mut calorie_counts: Vec<i32> = std::vec![];
    for elf in split {
        if elf.is_empty() {
            calorie_counts.push(current_calories);
            current_calories = 0;
        } else {
            current_calories += elf.parse::<i32>().unwrap()
        }
    }
    calorie_counts.sort();
    calorie_counts.reverse();
    calorie_counts[0] + calorie_counts[1] + calorie_counts[2]
}
