use aoc_2022::line_by_line;

fn main() {
    // Part 1
    let score = line_by_line("./examples/p2/rps.txt").fold(0, |score, x| {
        let mut spl = x.split(' ');
        let opp = spl.next().unwrap();
        let us = spl.next().unwrap();

        // A, X = Rock
        // B, Y = Paper
        // C, Z = Scissors
        score
            + match (opp, us) {
                ("A", "X") => 1 + 3,
                ("A", "Y") => 2 + 6,
                ("A", "Z") => 3 + 0,
                ("B", "X") => 1 + 0,
                ("B", "Y") => 2 + 3,
                ("B", "Z") => 3 + 6,
                ("C", "X") => 1 + 6,
                ("C", "Y") => 2 + 0,
                ("C", "Z") => 3 + 3,
                _ => unreachable!(),
            }
    });

    println!("{}", score);

    // Part 2
    let score = line_by_line("./examples/p2/rps.txt").fold(0, |score, x| {
        let mut spl = x.split(' ');
        let opp = spl.next().unwrap();
        let us = spl.next().unwrap();

        // A, X = Rock
        // B, Y = Paper
        // C, Z = Scissors
        score
            + match (opp, us) {
                ("A", "X") => 3 + 0,
                ("A", "Y") => 1 + 3,
                ("A", "Z") => 2 + 6,
                ("B", "X") => 1 + 0,
                ("B", "Y") => 2 + 3,
                ("B", "Z") => 3 + 6,
                ("C", "X") => 2 + 0,
                ("C", "Y") => 3 + 3,
                ("C", "Z") => 1 + 6,
                _ => unreachable!(),
            }
    });
    println!("{}", score);
}
