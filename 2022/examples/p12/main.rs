use aoc_2022::line_by_line;
use petgraph::{visit::IntoNodeReferences, Graph};

fn main() {
    let mut start_coord = (0, 0);
    let mut end_coord = (0, 0);

    let heights = line_by_line("./examples/p12/heightmap.txt")
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, x)| match x {
                    'S' => {
                        start_coord = (row, col);
                        0
                    }
                    'E' => {
                        end_coord = (row, col);
                        25
                    }
                    other => other.to_digit(36).unwrap() - 10,
                })
                .collect::<Vec<_>>()
        });

    let mut graph = Graph::new();

    let nodes = heights
        .map(|x| {
            x.into_iter()
                .map(|height| graph.add_node(height))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for row in 0..nodes.len() {
        for col in 0..nodes[row].len() {
            let curr = nodes[row][col];
            let curr_val = *graph.node_weight(curr).unwrap() as isize;

            let neighbours = [
                // Up
                (-1, 0),
                // Down
                (1, 0),
                // Left
                (0, -1),
                // Right
                (0, 1),
            ]
            .into_iter()
            .filter_map(|(r, c)| {
                let r = row as isize + r;
                let c = col as isize + c;

                Some((r.try_into().ok()?, c.try_into().ok()?))
            })
            .filter_map(|(r, c): (usize, usize)| nodes.get(r)?.get(c))
            .filter(|&x| match graph.node_weight(*x) {
                Some(val) => (..=1).contains(&(*val as isize - curr_val)),
                None => false,
            })
            .collect::<Vec<_>>();

            for n in neighbours {
                graph.update_edge(curr, *n, 1);
            }
        }
    }

    let start = nodes[start_coord.0][start_coord.1];
    let end = nodes[end_coord.0][end_coord.1];
    let distances = petgraph::algo::dijkstra(&graph, start, Some(end), |_| 1);

    // Part 1
    println!("{}", distances[&end]);

    // Part 2
    let starts = graph
        .node_references()
        .filter_map(|(node, val)| (*val == 0).then_some(node));

    let distances = starts
        .filter_map(|node| {
            // Definitely inefficient but /shrug it works
            let dists = petgraph::algo::dijkstra(&graph, node, Some(end), |_| 1);
            dists.get(&end).copied()
        })
        .collect::<Vec<_>>();

    println!("{}", distances.iter().min().unwrap());
}
