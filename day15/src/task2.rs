use std::fs;

pub fn execute() {
    let binding = fs::read_to_string("txts/input.txt").expect("Can't open file");
    let contents: Vec<_> = binding.lines().collect();
}