use std::fs;
use std::collections::{VecDeque, HashSet};
use std::hash::Hash;

pub fn execute() {
    let binding = fs::read_to_string("txts/input.txt").expect("Can't open file");
    let contents: Vec<_> = binding.lines().collect();

    let mut board: Vec<Vec<char>> = Vec::new();
    let mut distances: Vec<Vec<usize>> = Vec::new();

    calc_board(&mut distances, &mut board, contents);

    let (start_x, start_y) = find_start(&board).unwrap();


    calc_dist(&board, &mut distances, start_x, start_y);

    //println!("{:#?}", distances);

    println!("{}", max_non_inf(distances));
}

fn calc_board(distances: &mut Vec<Vec<usize>>, board: &mut Vec<Vec<char>>, contents: Vec<&str>) {
    for line in contents {
        board.push(Vec::new());
        distances.push(Vec::new());

        for symbol in line.chars() {
            if symbol == '\n' {
                break;
            }

            let board_len = board.len();
            board[board_len - 1].push(symbol);
            distances[board_len - 1].push(usize::MAX);
        }
    }
}

fn find_start(board: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for x in 0..board.len() {
        for y in 0..board[x].len() {
            if board[x][y] == 'S' {
                return Some((x, y));
            }
        }
    }

    None
}

fn calc_dist(board: &Vec<Vec<char>>, distances: &mut Vec<Vec<usize>>, start_x: usize, start_y: usize) {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back((start_x, start_y));
    distances[start_x][start_y] = 0;

    let n = distances.len();
    let m = distances[n - 1].len();

    while queue.len() != 0 {
        let (cur_x, cur_y) = queue.pop_front().unwrap();

        //println!("{:#?}", (cur_x, cur_y));

        for direction in Direction::char_directions(board[cur_x as usize][cur_y as usize]) {
            let (dx, dy) = direction.get_vector();

            let (new_x, new_y) = (cur_x as i32 + dx, cur_y as i32 + dy);

            if new_x < 0 || new_x >= n as i32 || new_y < 0 || new_y >= m as i32 {
                continue;
            }

            //println!("{} {:#?} {:#?}", board[new_x as usize][new_y as usize], direction.get_inverse(), Direction::char_directions(board[new_x as usize][new_y as usize]));

            if !Direction::char_directions(board[new_x as usize][new_y as usize])
                .contains(&direction.get_inverse()) {
                continue;
            }

            if distances[new_x as usize][new_y as usize] == usize::MAX || distances[new_x as usize][new_y as usize] > distances[cur_x as usize][cur_y as usize] + 1 {
                distances[new_x as usize][new_y as usize] = distances[cur_x as usize][cur_y as usize] + 1;
                queue.push_back((new_x as usize, new_y as usize));
            }
        }
    }
}

fn max_non_inf(distances: Vec<Vec<usize>>) -> usize {
    let mut ans: usize = 0;

    for row in distances {
        for elem in row {
            if elem != usize::MAX && elem > ans {
                ans = elem;
            }
        }
    }

    ans
}

#[derive(Hash, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn get_vector(&self) -> (i32, i32) {
        match self {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Up => (-1, 0),
        }
    }

    fn get_inverse(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
        }
    }

    fn char_directions(letter: char) -> HashSet<Direction> {
        let mut answer: HashSet<Direction> = HashSet::new();

        match letter {
            '|' => {
                answer.insert(Direction::Up);
                answer.insert(Direction::Down);
            },
            '-' => {
                answer.insert(Direction::Left);
                answer.insert(Direction::Right);
            },
            'F' => {
                answer.insert(Direction::Down);
                answer.insert(Direction::Right);
            },
            'L' => {
                answer.insert(Direction::Up);
                answer.insert(Direction::Right);
            },
            'J' => {
                answer.insert(Direction::Up);
                answer.insert(Direction::Left);
            },
            '7' => {
                answer.insert(Direction::Down);
                answer.insert(Direction::Left);
            },
            'S' => {
                answer.insert(Direction::Up);
                answer.insert(Direction::Left);
                answer.insert(Direction::Down);
                answer.insert(Direction::Right);
            },
            _ => {},

        };

        answer
    }

    fn get_numeric(&self) -> i32 {
        match self {
            Direction::Down => 1,
            Direction::Right => 2,
            Direction::Left => 3,
            Direction::Up => 4,
        }
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Direction) -> bool {
        self.get_numeric() == other.get_numeric()
    }
}

impl Eq for Direction {}