use std::collections::HashMap;

#[derive(Debug, Default)]
struct Monkey<'a> {
    number: u8,
    test: u32,
    if_true: u8,
    if_false: u8,
    op: &'a str,
    num: &'a str,
    items: Vec<u32>,
}

fn parse_monkey(monkey_str: &str) -> Monkey {
    let mut monkey = Monkey::default();
    let mut lines = monkey_str.lines();
    // Parse the monkey number.
    monkey.number = lines
        .next()
        .expect("No first line")
        .strip_prefix("Monkey ")
        .expect("First line formatted wrong")
        .strip_suffix(':')
        .expect("Missing colon")
        .parse()
        .expect("Error parsing monkey number");

    // Parse the starting items.
    let starting_items = lines
        .next()
        .expect("Missing starting items")
        .strip_prefix("  Starting items: ")
        .expect("Missing line description")
        .split(", ");
    for worry in starting_items {
        monkey
            .items
            .push(worry.parse().expect("Unable to parse starting items."));
    }

    // Parse the operation.
    let operation = lines.next().expect("Missing operation");
    (monkey.op, monkey.num) = operation
        .strip_prefix("  Operation: new = old ")
        .expect("Unable to parse Operation line")
        .split_once(' ')
        .expect("");

    // Parse the test line.
    let test = lines.next().expect("Missing test");
    monkey.test = test
        .chars()
        .last()
        .expect("Not possible")
        .to_string()
        .parse()
        .expect("Unable to parse test");

    // Parse the if true line.
    let if_true = lines.next().expect("Missing if true");
    monkey.if_true = if_true
        .chars()
        .last()
        .expect("Not possible")
        .to_string()
        .parse()
        .expect("Unable to parse monkey to throw to");

    // Parse the if false line.
    let if_false = lines.next().expect("Missing if false");
    monkey.if_false = if_false
        .chars()
        .last()
        .expect("Not possible")
        .to_string()
        .parse()
        .expect("Unable to parse monkey to throw to");
    monkey
}

pub fn day11part1(_input: &str) -> u32 {
    let monkeys = _input.split("\n\n");
    let mut monkey_list: HashMap<u8, Monkey> = HashMap::new();
    for monkey_str in monkeys {
        let monkey = parse_monkey(monkey_str);
        monkey_list.insert(monkey.number, monkey);
    }
    0
}

pub fn day11part2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use crate::day11::day11part1;
    use std::fs::read_to_string;

    #[test]
    fn part1test() {
        let s = read_to_string("test_input/day11.txt").expect("Unable to open file.");
        assert_eq!(day11part1(&s), 10605);
    }
}
