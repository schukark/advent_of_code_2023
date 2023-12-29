use std::fs;

pub fn execute() {
    let binding = fs::read_to_string("txts/sample.txt").expect("Can't open file");
    let contents: Vec<&str> = binding.lines().collect();
    let grid: Vec<Vec<char>> = contents.iter().map(|x| x.chars().collect::<Vec<char>>()).collect();

    let (start_x, start_y) = find_start(&grid);
    let (end_x, end_y) = find_end(&grid);

    let mut visited: Vec<Vec<bool>> = Vec::new();

    for i in 0..grid.len() {
        visited.push(Vec::new());
        for _ in 0..grid[0].len() {
            visited[i].push(false);
        }
    }

    let longest_path = dfs(&grid, &mut visited,  0, (start_x, start_y), (end_x, end_y));
    println!("{longest_path}");
}

fn dfs(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, 
    depth: i32, cur_position: (i32, i32), end_position: (i32, i32)) -> i32 {

    visited[cur_position.0 as usize][cur_position.1 as usize] = true;

    if end_position == cur_position {
        println!("Changed max to {depth}");
        visited[cur_position.0 as usize][cur_position.1 as usize] = false;
        return depth;
    }

    let mut mut_ans = 0;

    for (dx, dy) in vec![(-1, 0), (0, 1), (1, 0), (0, -1)] {
        let (new_x, new_y) = (cur_position.0 + dx, cur_position.1 + dy);

        if new_x < 0 || new_x >= grid.len() as i32 || new_y < 0 || new_y >= grid[0].len() as i32 {
            continue;
        }

        if visited[new_x as usize][new_y as usize] || grid[new_x as usize][new_y as usize] == '#' {
            continue;
        }

        if grid[new_x as usize][new_y as usize] == '.' {
            mut_ans = i32::max(mut_ans, dfs(grid, visited, depth + 1, (new_x, new_y), end_position));
        }
        else if get_arrow((dx, dy)) == grid[new_x as usize][new_y as usize] {
            let (new_x, new_y) = (cur_position.0 + 2 * dx, cur_position.1 + 2 * dy);
            mut_ans = i32::max(dfs(grid, visited,  depth + 1, (new_x, new_y), end_position), mut_ans);
        }
    }

    visited[cur_position.0 as usize][cur_position.1 as usize] = false;
    return mut_ans;
}

fn get_arrow(d: (i32, i32)) -> char {
    match d {
        (1, 0) => 'v',
        (-1, 0) => '^',
        (0, 1) => '>',
        (0, -1) => '<',
        _ => panic!("Wrong symbol"),
    }
}

fn find_start(grid: &Vec<Vec<char>>) -> (i32, i32) {
    for i in 0..grid[0].len() {
        if grid[0][i] == '.' {
            return (0, i as i32);
        }
    }

    (-1, -1)
}

fn find_end(grid: &Vec<Vec<char>>) -> (i32, i32) {
    for i in 0..grid[grid.len() - 1].len() {
        if grid[grid.len() - 1][i] == '.' {
            return (grid.len() as i32 - 1, i as i32);
        }
    }

    (-1, -1)
}