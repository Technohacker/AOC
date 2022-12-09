use std::{collections::HashSet, iter};

use aoc_2022::line_by_line;

type Pos = (isize, isize);

fn lagging_iter(init: Pos, leader: impl Iterator<Item = Pos>) -> impl Iterator<Item = Pos> {
    leader.scan(init, |curr_pos, (hx, hy)| {
        let dist: (isize, isize) = (hx - curr_pos.0, hy - curr_pos.1);

        let (dx, dy) = match (dist.0.abs(), dist.1.abs()) {
            // On tail, do nothing
            (0, 0)
            // 1 away on one axis, do nothing
            | (0, 1)
            | (1, 0)
            // 1 away on each axis, do nothing
            | (1, 1) => (0, 0),
            // 2 away on one axis, 0 on other, move along that axis
            (2, 0) => (dist.0.signum(), 0),
            (0, 2) => (0, dist.1.signum()),
            // 2 away on one axis, 1/2 away on other, move diagonally
            (2, 1) | (1, 2) | (2, 2) => (dist.0.signum(), dist.1.signum()),
            // All others are impossible
            delta => unreachable!("{:?}", delta),
        };

        curr_pos.0 += dx;
        curr_pos.1 += dy;

        Some(*curr_pos)
    })
}

fn main() {
    let head_moves = line_by_line("./examples/p9/steps.txt").flat_map(|x| {
        let mut spl = x.split(' ');
        let (dir, count) = (
            spl.next().unwrap(),
            spl.next().unwrap().parse::<usize>().unwrap(),
        );

        match dir {
            "U" => iter::repeat((0, 1)).take(count),
            "D" => iter::repeat((0, -1)).take(count),
            "L" => iter::repeat((-1, 0)).take(count),
            "R" => iter::repeat((1, 0)).take(count),
            _ => unreachable!(),
        }
    });

    let head_positions: Vec<_> = iter::once((0, 0))
        .chain(head_moves.scan((0, 0), |curr_pos, (dx, dy)| {
            curr_pos.0 += dx;
            curr_pos.1 += dy;

            Some(*curr_pos)
        }))
        .collect();

    // Part 1
    let knot_1_positions = lagging_iter((0, 0), head_positions.iter().copied());
    println!("{}", knot_1_positions.collect::<HashSet<_>>().len());

    // Part 2
    let head_positions: Box<dyn Iterator<Item = Pos>> = Box::new(head_positions.iter().copied());
    let knot_10_positions = (1..=9).fold(head_positions, |curr, _| {
        Box::new(lagging_iter((0, 0), curr))
    });

    println!("{}", knot_10_positions.collect::<HashSet<_>>().len());
}
