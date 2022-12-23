use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut forest: Vec<Vec<i32>> = vec![];
    for line in input.lines() {
        forest.push(vec![]);
        let current = forest.last_mut().unwrap();
        for height in line.chars() {
            current.push(height.to_digit(10).unwrap() as i32);
        }
    }
    forest
}

pub fn day8part1(input: &str) -> usize {
    let forest: Vec<Vec<i32>> = parse_input(input);
    let mut visible_set: HashSet<(usize, usize)> = HashSet::new();
    let mut visible = 0;
    let (mut x, mut y) = (0, 0);
    while y < forest.len() {
        let mut curr_height = -1;
        while x < forest[y].len() {
            if forest[y][x] > curr_height {
                visible += 1;
                visible_set.insert((x, y));
                curr_height = forest[y][x];
            }
            if curr_height == 9 {
                break;
            }
            x += 1;
        }
        y += 1;
        x = 0;
    }
    let (mut x, mut y) = (0, 0);
    while x < forest[y].len() {
        let mut curr_height = -1;
        while y < forest.len() {
            if forest[y][x] > curr_height {
                visible += 1;
                visible_set.insert((x, y));
                curr_height = forest[y][x];
            }
            if curr_height == 9 {
                break;
            }
            y += 1;
        }
        x += 1;
        y = 0;
    }
    let (mut x, mut y) = (forest[0].len() - 1, 0);
    while y < forest.len() {
        let mut curr_height = -1;
        loop {
            if forest[y][x] > curr_height {
                visible += 1;
                visible_set.insert((x, y));
                curr_height = forest[y][x];
            }
            if curr_height == 9 || x == 0 {
                break;
            }
            x -= 1;
        }
        x = forest[y].len() - 1;
        y += 1;
    }
    let (mut x, mut y) = (0, forest.len() - 1);
    while x < forest[y].len() {
        let mut curr_height = -1;
        loop {
            if forest[y][x] > curr_height {
                visible += 1;
                visible_set.insert((x, y));
                curr_height = forest[y][x];
            }
            if curr_height == 9 || y == 0 {
                break;
            }
            y -= 1;
        }
        x += 1;
        y = forest.len() - 1;
    }
    visible_set.len()
}

fn calculate_scenic_score(forest: &[Vec<i32>], tree_x: usize, tree_y: usize) -> u32 {
    let curr_tree_height = forest[tree_y][tree_x];
    let mut x = tree_x + 1;
    let mut right_score = 0;
    while x < forest[tree_y].len() {
        right_score += 1;

        if forest[tree_y][x] >= curr_tree_height {
            break;
        }
        x += 1;
    }
    let mut left_score = 0;
    if tree_x != 0 {
        let mut x = tree_x - 1;
        loop {
            left_score += 1;

            if forest[tree_y][x] >= curr_tree_height || x == 0 {
                break;
            }
            x -= 1;
        }
    }
    let mut y = tree_y + 1;
    let mut below_score = 0;
    while y < forest.len() {
        below_score += 1;

        if forest[y][tree_x] >= curr_tree_height {
            break;
        }
        y += 1;
    }
    let mut above_score = 0;
    if tree_y != 0 {
        let mut y = tree_y - 1;
        loop {
            above_score += 1;

            if forest[y][tree_x] >= curr_tree_height || y == 0 {
                break;
            }
            y -= 1;
        }
    }
    right_score * left_score * above_score * below_score
}

pub fn day8part2(input: &str) -> u32 {
    let forest: Vec<Vec<i32>> = parse_input(input);
    let mut highscore = 0;
    let (mut x, mut y) = (0, 0);
    while y < forest.len() {
        while x < forest.len() {
            let score = calculate_scenic_score(&forest, x, y);
            if score > highscore {
                highscore = score;
            }
            x += 1;
        }
        y += 1;
        x = 0;
    }
    highscore
}

#[cfg(test)]
mod test {
    use crate::day8::{day8part1, day8part2};
    use std::fs::read_to_string;

    #[test]
    fn part1test() {
        let s = read_to_string("test_input/day8.txt").expect("Unable to open file.");
        assert_eq!(day8part1(&s), 21);
    }

    #[test]
    fn part2test() {
        let s = read_to_string("test_input/day8.txt").expect("Unable to open file.");
        assert_eq!(day8part2(&s), 8);
    }
}
