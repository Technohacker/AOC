mod grouping_iterator;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub use grouping_iterator::GroupingIterator;

pub fn line_by_line(path: impl AsRef<str>) -> impl Iterator<Item = String> {
    let file = File::open(path.as_ref()).unwrap();

    BufReader::new(file)
        // For each line in the file
        .lines()
        .map(|x| x.unwrap())
}
