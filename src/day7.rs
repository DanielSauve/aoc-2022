use lazy_static::lazy_static;
use regex::Regex;
use trees::tr;

struct File<'a> {
    name: &'a str,
    size: u32,
}

lazy_static! {
    static ref CD: Regex = Regex::new(r"\$ cd [a-z]+").unwrap();
    static ref LS: Regex = Regex::new(r"\$ ls").unwrap();
    static ref DIR: Regex = Regex::new(r"dir [a-z]+").unwrap();
    static ref FILE: Regex = Regex::new(r"[0-9]+ [a-z]+").unwrap();
    static ref UP: Regex = Regex::new(r"\$ cd ..").unwrap();
}
pub fn day7part1(input: &str) -> u32 {
    let mut root: File = File { name: "/", size: 0 };
    let mut tree = tr(root);
    let mut curr = tree.root();
    for line in input.lines().skip(2) {
        if UP.is_match(line) {
            // curr = curr.parent().unwrap();
        }
        if CD.is_match(line) {
            // println!("{line}");
            break;
        } else if LS.is_match(line) {
            continue;
        } else if DIR.is_match(line) {
            let (_, dir) = line.split_once(" ").unwrap();
            // curr.push_back(tr(File { name: dir, size: 0 }));
        } else if FILE.is_match(line) {
            let (size_str, name) = line.split_once(" ").unwrap();
            let size: u32 = size_str.parse().unwrap();
            // curr.push_back(tr(File { name, size }));
        } else {
        }
    }
    for i in curr.bfs().iter {
        println!("Name: {} Size: {}", i.data.name, i.data.size);
    }
    let mut size = 0;
    size
}

pub fn day7part2(input: &str) -> u32 {
    let mut index = 14;
    index
}
