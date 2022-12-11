#[derive(Clone)]
struct Monkey {
    pub item_queue: Vec<usize>,
    pub operation: fn(&usize) -> usize,
    pub test_mod: usize,
    pub success: usize,
    pub fail: usize,
}

fn monkey_logic(monkey: &Monkey, relief: bool) -> Vec<(usize, usize)> {
    let mut instrs = Vec::new();

    for item in &monkey.item_queue {
        let mut new = (monkey.operation)(item);
        if relief {
            new /= 3_usize;
        }
        // Chinese Remainder Theorem ftw :D
        // This modulus is the product of all the test moduli from the test case
        new %= 9699690_usize;

        let next = if new % monkey.test_mod == 0 {
            monkey.success
        } else {
            monkey.fail
        };

        instrs.push((next, new));
    }

    instrs
}

fn monkey_business(mut counts: Vec<usize>) -> usize {
    counts.sort_unstable_by(|a, b| b.cmp(a));

    counts[0] * counts[1]
}

fn main() {
    let mut monkeys = [
        // 0
        Monkey {
            item_queue: vec![50, 70, 89, 75, 66, 66],
            operation: |old| old * 5_usize,
            test_mod: 2,
            success: 2,
            fail: 1,
        },
        // 1
        Monkey {
            item_queue: vec![85],
            operation: |old| old.pow(2),
            test_mod: 7,
            success: 3,
            fail: 6,
        },
        // 2
        Monkey {
            item_queue: vec![66, 51, 71, 76, 58, 55, 58, 60],
            operation: |old| old + 1_usize,
            test_mod: 13,
            success: 1,
            fail: 3,
        },
        // 3
        Monkey {
            item_queue: vec![79, 52, 55, 51],
            operation: |old| old + 6_usize,
            test_mod: 3,
            success: 6,
            fail: 4,
        },
        // 4
        Monkey {
            item_queue: vec![69, 92],
            operation: |old| old * 17_usize,
            test_mod: 19,
            success: 7,
            fail: 5,
        },
        // 5
        Monkey {
            item_queue: vec![71, 76, 73, 98, 67, 79, 99],
            operation: |old| old + 8_usize,
            test_mod: 5,
            success: 0,
            fail: 2,
        },
        // 6
        Monkey {
            item_queue: vec![82, 76, 69, 69, 57],
            operation: |old| old + 7_usize,
            test_mod: 11,
            success: 7,
            fail: 4,
        },
        // 7
        Monkey {
            item_queue: vec![65, 79, 86],
            operation: |old| old + 5_usize,
            test_mod: 17,
            success: 5,
            fail: 0,
        },
    ];
    // For part 2
    let clone = monkeys.clone();

    // Part 1
    let mut counts = [0_usize; 8];
    for _round in 1..=20 {
        for turn in 0..8 {
            let instrs = monkey_logic(&monkeys[turn], true);

            // Take all items from this monkey
            let items = std::mem::take(&mut monkeys[turn].item_queue);
            counts[turn] += items.len();

            for (to, item) in instrs {
                monkeys[to].item_queue.push(item);
            }
        }
    }

    println!("{}", monkey_business(Vec::from(counts)),);

    // Part 2
    let mut monkeys = clone;
    let mut counts = [0_usize; 8];
    for _round in 1..=10000 {
        for turn in 0..8 {
            let instrs = monkey_logic(&monkeys[turn], false);

            // Take all items from this monkey
            let items = std::mem::take(&mut monkeys[turn].item_queue);
            counts[turn] += items.len();

            for (to, item) in instrs {
                monkeys[to].item_queue.push(item);
            }
        }
    }

    println!("{}", monkey_business(Vec::from(counts)),);
}
