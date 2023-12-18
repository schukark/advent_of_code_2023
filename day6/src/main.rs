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
    
    let times = vec![times[1..].join("")];

    let times: Vec<u64> = times
            .iter()
            .map(|x| x.parse().unwrap())
            .collect();

    let string2 = contents[1];
    let distances: Vec<_> = string2
            .trim()
            .split_whitespace()
            .collect();

    let distances = vec![distances[1..].join("")];

    let distances: Vec<u64> = distances
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

fn calculate_times(total: u64, distance: u64) -> u64 {
    let mut result: u64 = 0;

    for t in 0..=total {
        if has_time(total, t, distance) {
            result += 1;
        }
    }

    result
}

fn has_time(total: u64, time: u64, distance: u64) -> bool {
    time * (total - time) > distance
}