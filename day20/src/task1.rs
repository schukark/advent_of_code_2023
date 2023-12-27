use std::fs;
use std::collections::{HashMap, VecDeque};

pub fn execute() {
    let binding = fs::read_to_string("txts/sample.txt").expect("Can't open file");
    let contents: Vec<&str> = binding.lines().collect();
}