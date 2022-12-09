use std::collections::HashSet;

pub fn day8part1(input: &str) -> usize {
    let mut forest: Vec<Vec<i32>> = vec![];
    for line in input.lines() {
        forest.push(vec![]);
        let current = forest.last_mut().unwrap();
        for height in line.chars() {
            current.push(height.to_digit(10).unwrap() as i32);
        }
    }
    let mut visible_set: HashSet<(usize, usize)> = HashSet::new();
    let mut visible = 0;
    let (mut x, mut y) = (0usize, 0usize);
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
    let (mut x, mut y) = (0usize, 0usize);
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

pub fn day8part2(input: &str) -> u32 {
    0
}
