use std::fs;

pub fn execute() {
    let binding = fs::read_to_string("txts/input.txt").expect("Can't open file");
    let contents: String = binding;

    let mut ans = 0;

    assert_eq!(hash_string("HASH"), 52);

    for expr in contents.split(",") {
        let tmp = hash_string(expr);
        ans += tmp as usize;
    }

    println!("{ans}");
}

fn hash_string(line: &str) -> u16 {
    let mut ans: u16 = 0;

    for char in line.chars() {
        ans += char as u16;
        ans = (ans * 17 as u16) % 256;
    }

    ans
}