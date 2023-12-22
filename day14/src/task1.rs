use std::fs;

pub fn execute() {
    let binding = fs::read_to_string("txts/input.txt").expect("Can't open file");
    let contents: Vec<_> = binding.lines().collect();

    println!("{}", construct_ans(contents));
}

fn construct_ans(lines: Vec<&str>) -> i32 {
    let mut stack: Vec<i32> = Vec::new();

    for i in 0..lines[0].len() {
        stack.push(-1);
    }

    let mut ans = 0;

    for (i, line) in lines.iter().enumerate() {
        for (j, symbol) in line.chars().enumerate() {
            if symbol == 'O' {
                let tmp = lines.len() as i32 - stack[j] - 1;
                ans += tmp;
                stack[j] += 1;

                //println!("{i} {j} {tmp}");
            }
            else if symbol == '#' {
                stack[j] = i as i32;
            }
        }
    }

    ans
}