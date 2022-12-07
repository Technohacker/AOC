use std::collections::{HashMap, hash_map::Entry};

use aoc_2022::line_by_line;

fn all_unique(msg: &str) -> bool {
    let mut hist = HashMap::new();

    for c in msg.chars() {
        match hist.entry(c) {
            Entry::Occupied(x) => return false,
            Entry::Vacant(x) => x.insert(1),
        };
    }

    true
}

fn first_unique_window(msg: &str, n: usize) -> Option<usize> {
    for i in n..msg.len() {
        let msg = &msg[(i - n)..i];
        if all_unique(msg) {
            return Some(i);
        }
    }
    None
}
fn main() {
    let data = line_by_line("./examples/p6/databuf.txt").next().unwrap();
    
    // Part 1
    println!("{}", first_unique_window(&data, 4).unwrap());
    // Part 2
    println!("{}", first_unique_window(&data, 14).unwrap());
}
