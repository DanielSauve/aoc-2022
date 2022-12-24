pub fn day10part1(input: &str) -> i64 {
    let mut cycle_to_check = 20;
    let mut register = 1;
    let mut curr_cycle = 0;
    let mut signal_strength = 0;
    for line in input.lines() {
        if line == "noop" {
            curr_cycle += 1;
            if curr_cycle == cycle_to_check {
                println!("{curr_cycle}, {register}, {}", curr_cycle * register);
                cycle_to_check += 40;
                signal_strength += curr_cycle * register;
            }
        } else {
            let amount: i64 = line.split_once(' ').unwrap().1.parse().unwrap();
            curr_cycle += 1;
            if curr_cycle == cycle_to_check {
                println!("{curr_cycle}, {register}, {}", curr_cycle * register);
                cycle_to_check += 40;
                signal_strength += curr_cycle * register;
            }
            curr_cycle += 1;
            if curr_cycle == cycle_to_check {
                println!("{curr_cycle}, {register}, {}", curr_cycle * register);
                cycle_to_check += 40;
                signal_strength += curr_cycle * register;
            }
            register += amount;
        }
    }
    signal_strength
}
pub fn day10part2(input: &str) -> String {
    let mut crt: Vec<Vec<char>> = Vec::new();
    for _ in 0..6 {
        crt.push(Vec::from(['#'; 40]))
    }
    let mut curr_cycle: usize = 0;
    let mut sprite_position: i32 = 0;
    let mut row: usize = 0;
    for line in input.lines() {
        if line == "noop" {
            curr_cycle += 1;
            if (curr_cycle - 1) % 40 == 0 {
                row += 1;
            }
            if (curr_cycle % 40) as i32 - sprite_position <= 0 {
                crt[row][curr_cycle % 40] = '#';
            } else {
                crt[row][curr_cycle % 40] = '.';
            }
        } else {
            let amount: i32 = line.split_once(' ').unwrap().1.parse().unwrap();
            curr_cycle += 1;
            if (curr_cycle - 1) % 40 == 0 {
                row += 1;
            }
            if (curr_cycle % 40) as i32 - sprite_position <= 0 {
                crt[row][curr_cycle % 40] = '#';
            } else {
                crt[row][curr_cycle % 40] = '.';
            }
            curr_cycle += 1;
            if (curr_cycle - 1) % 40 == 0 {
                row += 1;
            }
            if (curr_cycle % 40) as i32 - sprite_position <= 0 {
                crt[row][curr_cycle % 40] = '#';
            } else {
                crt[row][curr_cycle % 40] = '.';
            }
            sprite_position += amount;
        }
    }
    let mut output: String = String::new();
    for vec in crt {
        for x in vec {
            output.push(x);
        }
        output.push('\n');
    }
    output
}

#[cfg(test)]
mod test {
    use crate::day10::{day10part1, day10part2};
    use std::fs::read_to_string;

    #[test]
    fn part1test() {
        let s = read_to_string("test_input/day10.txt").expect("Unable to open file.");
        assert_eq!(day10part1(&s), 13140);
    }

    #[test]
    fn part2test() {
        let s = read_to_string("test_input/day10.txt").expect("Unable to open file.");
        assert_eq!(
            day10part2(&s),
            "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n"
        );
    }
}
