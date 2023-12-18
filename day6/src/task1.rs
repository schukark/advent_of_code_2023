use std::fs;

fn main() {
    let binding = fs::read_to_string("txts/input.txt")
                            .expect("Error opening file");
    let contents: Vec<_> = binding
                            .lines()
                            .collect();

    let string1 = contents[0];
    let times: Vec<_> = string1
            .trim()
            .split_whitespace()
            .collect();

    let times: Vec<i32> = (&times[1..])
            .iter()
            .map(|x| x.parse().unwrap())
            .collect();

    let string2 = contents[1];
    let distances: Vec<_> = string2
            .trim()
            .split_whitespace()
            .collect();

    let distances: Vec<i32> = (&distances[1..])
            .iter()
            .map(|x| x.parse().unwrap())
            .collect();

    let mut answer: i128 = 1;

    for i in 0..distances.len() {
        let tmp = calculate_times(times[i], distances[i]) as i128;
        answer *= tmp;
    }

    println!("{answer}");
}

fn calculate_times(total: i32, distance: i32) -> i32 {
    let mut result: i32 = 0;

    for t in 0..=total {
        if has_time(total, t, distance) {
            result += 1;
        }
    }

    result
}

fn has_time(total: i32, time: i32, distance: i32) -> bool {
    time * (total - time) > distance
}