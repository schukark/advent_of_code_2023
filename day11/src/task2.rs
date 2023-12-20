use std::fs;
use std::collections::HashSet;
use std::hash::Hash;

pub fn execute() {
    let binding = fs::read_to_string("txts/input.txt").expect("Can't read file");
    let contents: Vec<_> = binding.lines().collect();

    let mut board: HashSet<Point> = HashSet::new();
    let (m, n, present_rows, present_cols) = build_board(&contents, &mut board);

    let mut empty_rows: Vec<usize> = Vec::new();
    let mut empty_cols: Vec<usize> = Vec::new();

    for i in 0..n {
        if !present_rows.contains(&i) {
            empty_rows.push(i);
        }
    }

    for i in 0..m {
        if !present_cols.contains(&i) {
            empty_cols.push(i);
        }
    }

    empty_rows.sort();
    empty_cols.sort();

    let coef: usize = 1000000 - 1;


    let mut new_board: Vec<Point> = Vec::new();

    let (mut row_index, mut col_index) = (0usize, 0usize);
    for i in 0..n {
        col_index = 0usize;

        if row_index != empty_rows.len() && i > empty_rows[row_index] {
            row_index += 1;
        }

        for j in 0..m {
            if col_index != empty_cols.len() && j > empty_cols[col_index] {
                col_index += 1;
            }

            if !board.contains(&Point {x: i, y: j}) {
                continue;
            }

            new_board.push(Point{x: i + row_index * coef ,y: j + col_index * coef});
        }
    }

    let mut ans: usize = 0usize;

    for i in 0..new_board.len() {
        for j in i + 1..new_board.len() {
            ans += new_board[i].dist(&new_board[j]);
        }
    }

    println!("{ans}");
}

fn build_board(contents: &Vec<&str>, board: &mut HashSet<Point>) -> (usize, usize, HashSet<usize>, HashSet<usize>) {
    let n = contents.len();
    let m = contents[n - 1].len();

    let mut present_rows: HashSet<usize> = HashSet::new();
    let mut present_cols: HashSet<usize> = HashSet::new();

    for (i, line) in contents.iter().enumerate() {
        for (j, symbol) in line.chars().enumerate() {
            if symbol == '#' {
                board.insert(Point {x: i, y: j});
                present_rows.insert(i);
                present_cols.insert(j);
            }
        }
    }

    (m, n, present_rows, present_cols)
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn dist(&self, other: &Point) -> usize {
        isize::abs(self.x as isize - other.x as isize) as usize + isize::abs(self.y as isize - other.y as isize) as usize
    }
}