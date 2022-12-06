use aoc_2022::line_by_line;

fn main() {
    // Part 1
    let mut stacks = vec![
        vec!['Z', 'J', 'N', 'W', 'P', 'S'],
        vec!['G', 'S', 'T'],
        vec!['V', 'Q', 'R', 'L', 'H'],
        vec!['V', 'S', 'T', 'D'],
        vec!['Q', 'S', 'T', 'D', 'B', 'M', 'J'],
        vec!['M', 'W', 'T', 'J', 'D', 'C', 'Z', 'L'],
        vec!['L', 'P', 'M', 'W', 'G', 'T', 'J'],
        vec!['N', 'G', 'M', 'T', 'B', 'F', 'Q', 'H'],
        vec!['R', 'D', 'G', 'C', 'P', 'B', 'Q', 'W'],
    ];

    let instr_regex = regex::Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    line_by_line("./examples/p5/instructions.txt")
        .for_each(|x| {
            let caps = instr_regex.captures(&x).unwrap();
            let (num, src, dest) = (
                caps[1].parse::<usize>().unwrap(),
                caps[2].parse::<usize>().unwrap(),
                caps[3].parse::<usize>().unwrap(),
            );

            for _ in 0..num {
                let x = stacks[src - 1].pop().unwrap();
                stacks[dest - 1].push(x);
            }
        });

    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();

    // Part 2
    let mut stacks = vec![
        vec!['Z', 'J', 'N', 'W', 'P', 'S'],
        vec!['G', 'S', 'T'],
        vec!['V', 'Q', 'R', 'L', 'H'],
        vec!['V', 'S', 'T', 'D'],
        vec!['Q', 'S', 'T', 'D', 'B', 'M', 'J'],
        vec!['M', 'W', 'T', 'J', 'D', 'C', 'Z', 'L'],
        vec!['L', 'P', 'M', 'W', 'G', 'T', 'J'],
        vec!['N', 'G', 'M', 'T', 'B', 'F', 'Q', 'H'],
        vec!['R', 'D', 'G', 'C', 'P', 'B', 'Q', 'W'],
    ];

    line_by_line("./examples/p5/instructions.txt")
        .for_each(|x| {
            let caps = instr_regex.captures(&x).unwrap();
            let (num, src, dest) = (
                caps[1].parse::<usize>().unwrap(),
                caps[2].parse::<usize>().unwrap(),
                caps[3].parse::<usize>().unwrap(),
            );

            let mut group = vec![];
            for _ in 0..num {
                let x = stacks[src - 1].pop().unwrap();
                group.push(x);
            }
            
            for x in group.iter().rev() {
                stacks[dest - 1].push(*x);
            }
        });

    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();
}
