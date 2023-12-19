use std::fs;

pub fn execute() {
    let binding = fs::read_to_string("txts/input.txt").expect("Can't open file");
    let contents: Vec<&str> = binding.lines().collect();

    let mut sum: i32 = 0;

    for line in contents {
        let array: Vec<_> = line.split_whitespace().collect();
        let array = array.iter().map(|x| x.parse().unwrap()).collect();

        sum +=  next_num(array);
    }

    println!("{sum}");
}

fn next_num(array: Vec<i32>) -> i32 {
    let mut differences: Vec<Vec<i32>> = Vec::new();
    differences.push(array);

    let mut all_zeros = true;

    loop {
        differences.push(Vec::new());
        all_zeros = true;

        
        let prev_index = differences.len() - 2;
        let prev_length = differences[prev_index].len();
        
        for i in 0..prev_length - 1 {
            let tmp = differences[prev_index][i + 1] - differences[prev_index][i];
            differences[prev_index + 1].push(tmp);
            
            if tmp != 0 {
                all_zeros = false;
            }
        }
        
        if all_zeros {
            break;
        }
    }
    
    let len = differences.len();
    differences[len - 1].push(0);

    let mut answer: i32 = 0;
    
    for i in (1..len).rev() {
        let cur_len = differences[i].len();
        let tmp = differences[i][cur_len - 1] + differences[i - 1][cur_len - 1];
        answer = tmp;
        
        differences[i - 1].push(tmp);
    }

    answer
}