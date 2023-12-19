use std::collections::HashMap;
use std::fs;
use std::num;

pub fn execute() {
    let binding = fs::read_to_string("txts/input.txt").expect("Can't open file");
    let contents: Vec<_> = binding.lines().collect();
    let contents: Vec<String> = contents.iter().map(|x| x.to_string()).collect();

    let hash_map = build_hash_map(&contents);
    let mut answer: i64 = 1;

    for line in &contents[2..] {
        if line.chars().nth(2).unwrap() != 'A' {
            continue;
        }

        let current: i64 = traverse(line[..3].to_string(), &contents[0], &hash_map) as i64;

        answer = lcm(answer, current);
    }

    println!("{answer}");
}

#[derive(Debug)]
struct Pair {
    left: String,
    right: String,
}

fn build_hash_map(string_lines: &Vec<String>) -> HashMap<String, Pair> {
    let mut answer: HashMap<String, Pair> = HashMap::new();

    for line in &string_lines[2..] {
        let spaces: Vec<_> = line.split_whitespace().collect();
        answer.insert(spaces[0].to_string(), Pair {
            left: spaces[2][1..4].to_string(), right: spaces[3][0..3].to_string()
        });
    }
    
    answer
}

fn traverse(initial: String, sequence: &String, moves: &HashMap<String, Pair>) -> usize {
    let mut answer: usize = 0;

    let mut position = initial.clone();

    loop {
        if position.chars().nth(2).unwrap() == 'Z' {
            break;
        }

        let move_index: usize = answer % sequence.len();
        let result = moves.get(&position).unwrap();

        let mut next_move: String = String::from("");

        if sequence.chars().nth(move_index).unwrap() == 'L' {
            next_move = result.left.clone();
        }
        else {
            next_move = result.right.clone();
        }

        position = next_move;
        answer += 1;
    }

    answer
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn gcd(a: i64, b: i64) -> i64  {
    let (mut n, mut m) = (a, b);
    while m != 0 {
        if n > m {
            (n, m) = (m, n);
        }
        m %= n;
    }

    n
}