use std::{str::FromStr, fmt::Debug};

use aoc_2022::{line_by_line, GroupingIterator};

#[derive(Clone, PartialEq, Eq)]
enum PacketData {
    Int(usize),
    List(Vec<PacketData>),
}

peg::parser! {
    grammar list_parser() for str {
        rule number() -> PacketData
            = n:$(['0'..='9']+) {? n.parse().map(PacketData::Int).or(Err("usize")) }

        pub(crate) rule list() -> PacketData
            = "[" l:((number() / list()) ** ",") "]" { PacketData::List(l) }
    }
}

impl FromStr for PacketData {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        list_parser::list(s).map_err(|_| ())
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (PacketData::Int(a), PacketData::Int(b)) => a.cmp(b),
            (PacketData::List(a), PacketData::List(b)) => a.cmp(b),
            (PacketData::Int(i), PacketData::List(l)) => vec![PacketData::Int(*i)].cmp(l),
            (PacketData::List(l), PacketData::Int(i)) => l.cmp(&vec![PacketData::Int(*i)]),
        }
    }
}

impl Debug for PacketData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(arg0) => write!(f, "{}", arg0),
            Self::List(arg0) => f.debug_list().entries(arg0).finish(),
        }
    }
}

fn main() {
    let pairs =
        GroupingIterator::new(line_by_line("./examples/p13/signals.txt"), |x| x.is_empty())
            .map(|x| {
                let mut iter = x
                    .into_iter()
                    .take_while(|x| !x.is_empty())
                    .map(|x| x.parse::<PacketData>().unwrap());

                (iter.next().unwrap(), iter.next().unwrap())
            })
            .collect::<Vec<_>>();

    // Part 1
    let match_count: usize = pairs
        .iter()
        .enumerate()
        .map(|(n, (a, b))| if a > b { 0 } else { n + 1 })
        .sum();

    println!("{}", match_count);

    // Part 2
    let divider_1: PacketData = "[[2]]".parse().unwrap();
    let divider_2: PacketData = "[[6]]".parse().unwrap();

    let mut packets = pairs
        .into_iter()
        .flat_map(|(a, b)| [a, b])
        .collect::<Vec<_>>();

    packets.push(divider_1.clone());
    packets.push(divider_2.clone());
    packets.sort();

    let key: usize = packets
        .iter()
        .enumerate()
        .filter_map(|(n, p)| ((p == &divider_1) || (p == &divider_2)).then(|| dbg!(n + 1)))
        .product();

    for p in packets {
        println!("{:?}", p);
    }
    println!("{}", key);
}
