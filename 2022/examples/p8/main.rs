use aoc_2022::line_by_line;

struct Mat {
    val: Vec<Vec<usize>>,
}

impl Mat {
    pub fn get(&self, row: usize, col: usize) -> usize {
        self.val[row][col]
    }

    pub fn row(&self, n: usize) -> impl DoubleEndedIterator<Item = (usize, usize)> + '_ {
        self.val[n].iter().copied().enumerate()
    }

    pub fn col(&self, n: usize) -> impl DoubleEndedIterator<Item = (usize, usize)> + '_ {
        self.val.iter().map(move |x| x[n]).enumerate()
    }

    pub fn items(&self) -> impl DoubleEndedIterator<Item = (usize, usize, usize)> + '_ {
        self.val
            .iter()
            .enumerate()
            .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, item)| (r, c, *item)))
    }

    pub fn up(&self, row: usize, col: usize) -> impl DoubleEndedIterator<Item = usize> + '_ {
        self.col(col)
            .filter_map(move |(i, n)| (i < row).then_some(n))
            .rev()
    }

    pub fn down(&self, row: usize, col: usize) -> impl DoubleEndedIterator<Item = usize> + '_ {
        self.col(col)
            .filter_map(move |(i, n)| (i > row).then_some(n))
    }

    pub fn left(&self, row: usize, col: usize) -> impl Iterator<Item = usize> + '_ {
        self.row(row)
            .filter_map(move |(i, n)| (i < col).then_some(n))
            .rev()
    }

    pub fn right(&self, row: usize, col: usize) -> impl Iterator<Item = usize> + '_ {
        self.row(row)
            .filter_map(move |(i, n)| (i > col).then_some(n))
    }
}

fn visible(mat: &Mat, row: usize, col: usize) -> bool {
    let height = mat.get(row, col);

    let below = |x| x < height;
    mat.up(row, col).all(below)
        || mat.down(row, col).all(below)
        || mat.left(row, col).all(below)
        || mat.right(row, col).all(below)
}

fn scenic_score(mat: &Mat, row: usize, col: usize) -> usize {
    let height = mat.get(row, col);

    let until_too_tall = |last_too_tall: &mut _, x| {
        if *last_too_tall {
            None
        } else {
            if x >= height {
                *last_too_tall = true
            }
            Some(x)
        }
    };
    mat.up(row, col).scan(false, until_too_tall).count()
        * mat.down(row, col).scan(false, until_too_tall).count()
        * mat.left(row, col).scan(false, until_too_tall).count()
        * mat.right(row, col).scan(false, until_too_tall).count()
}

fn main() {
    let mat = Mat {
        val: line_by_line("./examples/p8/trees.txt")
            .map(|x| {
                x.chars()
                    .map(|x| x.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect(),
    };

    // Part 1
    println!(
        "{}",
        mat.items()
            .filter(|(r, c, _)| visible(&mat, *r, *c))
            .count()
    );

    // Part 2
    println!(
        "{}",
        mat.items()
            .map(|(r, c, _)| scenic_score(&mat, r, c))
            .max()
            .unwrap()
    );
}
