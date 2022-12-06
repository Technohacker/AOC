use aoc_2022::line_by_line;

fn main() {
    // Part 1
    let score = line_by_line("./examples/p4/sections.txt")
        .map(|x| {
            let mut ranges = x.split(',').map(|x| {
                let mut iter = x.split('-').map(|x| x.parse::<usize>().unwrap());

                iter.next().unwrap()..iter.next().unwrap()
            });

            let left = ranges.next().unwrap();
            let right = ranges.next().unwrap();

            (left, right)
        })
        .filter(|(l, r)| {
            (l.start <= r.start && r.end <= l.end) || (r.start <= l.start && l.end <= r.end)
        })
        .count();

    println!("{}", score);

    // Part 2
    let score = line_by_line("./examples/p4/sections.txt")
        .map(|x| {
            let mut ranges = x.split(',').map(|x| {
                let mut iter = x.split('-').map(|x| x.parse::<usize>().unwrap());

                iter.next().unwrap()..iter.next().unwrap()
            });

            let left = ranges.next().unwrap();
            let right = ranges.next().unwrap();

            (left, right)
        })
        .filter(|(l, r)| (l.start <= r.end && l.end >= r.start))
        .count();

    println!("{}", score);
}
