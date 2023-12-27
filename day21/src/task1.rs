use std::fs;
use std::collections::BTreeSet;

pub fn execute() {
    let binding = fs::read_to_string("txts/input.txt").expect("Can't open file");
    let contents: Vec<&str> = binding.lines().collect();
    let grid: Vec<Vec<char>> = contents.iter().map(|x| x.chars().collect::<Vec<char>>()).collect();

    let (start_x, start_y) = find_start(&grid);

    let result = bfs(start_x, start_y, &grid, 64);
    // for (x, y) in &result {
    //     println!("{x} {y}");
    // }

    println!("{}", result.len());
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

fn bfs(start_x: i32, start_y: i32, grid: &Vec<Vec<char>>, steps: i32) -> BTreeSet<(i32, i32)> {
    let mut queue1: BTreeSet<(i32, i32)> = BTreeSet::new();
    queue1.insert((start_x, start_y));

    for _ in 0..steps {
        let mut queue2: BTreeSet<(i32, i32)> = BTreeSet::new();

        while !queue1.is_empty() {
            let top = queue1.pop_first().unwrap();
            for (dx, dy) in vec![(1, 0), (-1, 0), (0, -1), (0, 1)] {
                let new_x = top.0 + dx;
                let new_y = top.1 + dy;

                if new_x < 0 || new_x >= grid.len() as i32 || new_y < 0 || new_y >= grid[0].len() as i32 {
                    continue;
                }

                if grid[new_x as usize][new_y as usize] == '#' {
                    continue;
                }

                queue2.insert((new_x, new_y));
            }
        }

        queue1 = queue2;
    }

    queue1
}
