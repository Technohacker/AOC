use std::collections::{HashMap, hash_map::Entry};

use aoc_2022::line_by_line;

type Pos = (usize, usize);

fn main() {
    let mut cave: HashMap<Pos, ()> = HashMap::new();

    let rocks = line_by_line("./examples/p14/cave.txt").flat_map(|x| {
        let mut iter = x.split(" -> ")
            .map(|x| {
                let mut coords = x.split(',');
                (
                    coords.next().and_then(|x| x.parse::<usize>().ok()).unwrap(),
                    coords.next().and_then(|y| y.parse::<usize>().ok()).unwrap(),
                )
            });
        
        let start = iter.next().unwrap();
        iter.scan(start, |last_pos, next| {
            let curr = *last_pos;
            *last_pos = next;

            if curr.0 == next.0 {
                let range = if curr.1 < next.1 {
                    curr.1..=next.1
                } else {
                    next.1..=curr.1
                };
                Some(range.map(|y| (curr.0, y)).collect::<Vec<_>>())
            } else if curr.1 == next.1 {
                let range = if curr.0 < next.0 {
                    curr.0..=next.0
                } else {
                    next.0..=curr.0
                };
                Some(range.map(|x| (x, curr.1)).collect::<Vec<_>>())
            } else {
                unreachable!();
            }
        })
        .flatten()
        .collect::<Vec<_>>()
    });

    for pos in rocks {
        cave.insert(pos, ());
    }

    // Needed for part 2 and abyss checks
    let floor_y = cave.keys().map(|(_, y)| y + 2).max().unwrap();
    println!("{}", floor_y);

    let copy = cave.clone();

    // Part 1
    for n in 0.. {
        let mut sand_pos = (500, 0);
        let mut abyss = true;

        while sand_pos.1 < floor_y {
            let mut next_pos = (sand_pos.0, sand_pos.1 + 1);

            if !cave.contains_key(&next_pos) {
                sand_pos = next_pos;
                continue;
            }

            next_pos.0 = sand_pos.0 - 1;
            if !cave.contains_key(&next_pos) {
                sand_pos = next_pos;
                continue;
            }

            next_pos.0 = sand_pos.0 + 1;
            if !cave.contains_key(&next_pos) {
                sand_pos = next_pos;
                continue;
            }

            match cave.entry(sand_pos) {
                Entry::Occupied(_) => panic!(),
                Entry::Vacant(x) => *x.insert(()),
            }
            abyss = false;
            break;
        }

        if abyss {
            println!("{}", n);
            break;
        }
    }

    // Part 2
    let mut cave = copy;
    let mut sand_count = 0;
    loop {
        let mut sand_pos = (500, 0);
        let mut floor = true;

        while sand_pos.1 < floor_y {
            let mut next_pos = (sand_pos.0, sand_pos.1 + 1);

            if !cave.contains_key(&next_pos) {
                sand_pos = next_pos;
                continue;
            }

            next_pos.0 = sand_pos.0 - 1;
            if !cave.contains_key(&next_pos) {
                sand_pos = next_pos;
                continue;
            }

            next_pos.0 = sand_pos.0 + 1;
            if !cave.contains_key(&next_pos) {
                sand_pos = next_pos;
                continue;
            }

            if sand_pos == (500, 0) {
                // FIXME: Kinda messy
                println!("{}", sand_count + 1);
                return;
            }

            match cave.entry(sand_pos) {
                Entry::Occupied(_) => panic!(),
                Entry::Vacant(x) => *x.insert(()),
            }
            sand_count += 1;
            floor = false;
            break;
        }

        if floor {
            assert_eq!(sand_pos.1, floor_y);
            match cave.entry(sand_pos) {
                Entry::Occupied(_) => panic!(),
                Entry::Vacant(x) => *x.insert(()),
            }
        }
    }

}
