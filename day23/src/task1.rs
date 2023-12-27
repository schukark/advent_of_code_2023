use std::fs;

pub fn execute() {
    let binding = fs::read_to_string("txts/input.txt").expect("Can't open file");
    let contents: Vec<&str> = binding.lines().collect();
    let grid: Vec<Vec<char>> = contents.iter().map(|x| x.chars().collect::<Vec<char>>()).collect();

    let (start_x, start_y) = find_start(&grid);
}

fn find_start(grid: &Vec<Vec<char>>) -> (i32, i32) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                return (i as i32, j as i32);
            }
        }
    }

    (-1, -1)
}
