pub fn day2part1(input: &str) -> i32 {
    let split = input.split('\n');
    let mut score = 0;
    for round in split {
        let (opp, mut my) = round.split_at(1);
        my = my.strip_prefix(' ').unwrap();
        match my {
            "X" => {
                // Rock
                score += 1;
                match opp {
                    "A" => score += 3,
                    "B" => score += 0,
                    "C" => score += 6,
                    _ => continue,
                }
            }
            "Y" => {
                // Paper
                score += 2;
                match opp {
                    "A" => score += 6,
                    "B" => score += 3,
                    "C" => score += 0,
                    _ => continue,
                }
            }
            "Z" => {
                // Scissors
                score += 3;
                match opp {
                    "A" => score += 0,
                    "B" => score += 6,
                    "C" => score += 3,
                    _ => continue,
                }
            }
            _ => continue,
        }
    }
    score
}

pub fn day2part2(input: &str) -> i32 {
    let split = input.split('\n');
    let mut score = 0;
    for round in split {
        let (opp, mut my) = round.split_at(1);
        my = my.strip_prefix(' ').unwrap();
        match my {
            "X" => {
                // Lose
                match opp {
                    "A" => score += 3,
                    "B" => score += 1,
                    "C" => score += 2,
                    _ => continue,
                }
            }
            "Y" => {
                // Draw
                score += 3;
                match opp {
                    "A" => score += 1,
                    "B" => score += 2,
                    "C" => score += 3,
                    _ => continue,
                }
            }
            "Z" => {
                // Win
                score += 6;
                match opp {
                    "A" => score += 2,
                    "B" => score += 3,
                    "C" => score += 1,
                    _ => continue,
                }
            }
            _ => continue,
        }
    }
    score
}
