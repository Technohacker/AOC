use std::{
    fs::File,
    io::{BufRead, BufReader}, collections::BinaryHeap,
};

use aoc_2022::GroupingIterator;

fn main() {
    let file = File::open("./examples/p1/calorie.txt").unwrap();

    let lines = BufReader::new(file)
        // For each line in the file
        .lines()
        .map(|x| x.unwrap());

    // Group together
    let groups = GroupingIterator::new(lines, |x| x.is_empty());

    // Make a priority queue
    let mut queue = groups
        .map(|x| x.iter()
            .filter_map(|x| x.parse::<u32>().ok())
            .sum::<u32>()
        )
        .collect::<BinaryHeap<_>>();

    // For part 1
    println!("{}", queue.peek().unwrap());

    // For part 2
    let mut sum = 0;
    for _ in 0..3 {
        sum += queue.pop().unwrap();
    }
    println!("{}", sum);
}
