use std::collections::HashSet;

use aoc_2022::{line_by_line, GroupingIterator};

fn main() {
    // Part 1
    let score = line_by_line("./examples/p3/rucksacks.txt").fold(0, |score, x| {
        let half = x.len() / 2;
        let left = &x[..half].chars().collect::<HashSet<_>>();
        let right = &x[half..].chars().collect::<HashSet<_>>();

        let comm = *left.intersection(right).next().unwrap();

        score
            + if comm.is_ascii_lowercase() {
                comm as usize - 'a' as usize + 1
            } else {
                comm as usize - 'A' as usize + 27
            }
    });

    println!("{}", score);

    // Part 2
    let enumerated_lines = line_by_line("./examples/p3/rucksacks.txt")
        .enumerate()
        .map(|(n, x)| (n + 1, x));

    let grouped = GroupingIterator::new(enumerated_lines, |(n, _)| n % 3 == 0);
    let score = grouped.fold(0, |score, group| {
        let comm = dbg!(group)
            .iter()
            .map(|(_, x)| x.chars().collect::<HashSet<_>>())
            .reduce(|acc, x| acc.intersection(&x).copied().collect())
            .and_then(|x| x.iter().copied().next())
            .unwrap();

        score
            + if comm.is_ascii_lowercase() {
                comm as usize - 'a' as usize + 1
            } else {
                comm as usize - 'A' as usize + 27
            }
    });

    println!("{}", score);
}
