use aoc_2022::{line_by_line, GroupingIterator};

#[derive(Clone, Copy)]
enum Instruction {
    Add(isize),
    Noop,
}

fn main() {
    let instructions = line_by_line("./examples/p10/instructions.txt")
        .map(|x| {
            if x.starts_with("addx") {
                Instruction::Add(x.split(' ').nth(1).unwrap().parse().unwrap())
            } else if x == "noop" {
                Instruction::Noop
            } else {
                unreachable!("{}", x);
            }
        })
        .flat_map(|inst| {
            // Treat addx as being an instruction that takes 1 cycle with a noop to fill the other
            let iter: Box<dyn Iterator<Item = Instruction>> = match inst {
                Instruction::Add(x) => {
                    Box::new([Instruction::Noop, Instruction::Add(x)].into_iter())
                }
                Instruction::Noop => Box::new([Instruction::Noop].into_iter()),
            };

            iter
        });

    let x_values: Vec<_> = instructions
        .scan(1, |curr_x, inst| {
            let last_x = *curr_x;
            match inst {
                Instruction::Add(x) => *curr_x += x,
                Instruction::Noop => {}
            };

            Some(last_x)
        })
        .enumerate()
        .map(|(n, x)| ((n + 1) as isize, x))
        .collect();

    let signal_strengths: Vec<_> = x_values.iter().map(|(n, x)| n * x).collect();

    // Part 1
    println!(
        "{}",
        signal_strengths[19]
            + signal_strengths[59]
            + signal_strengths[99]
            + signal_strengths[139]
            + signal_strengths[179]
            + signal_strengths[219]
    );

    // Part 2
    let crt_pixel = GroupingIterator::new(x_values.iter(), |(n, _)| n % 40 == 0)
        .enumerate()
        .map(|(row_count, row)| {
            row.into_iter()
                .map(move |(n, x)| (*n - (40 * row_count as isize), x))
        })
        .map(|row| {
            row.map(|(n, x)| {
                if (x - 1..=x + 1).contains(&(n - 1)) {
                    "#"
                } else {
                    // Empty string for better readability
                    " "
                }
            })
        });

    for row in crt_pixel {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}
