use std::fs;

pub fn execute() {
    let binding = fs::read_to_string("txts/input.txt").expect("Can't open file");
    let contents: Vec<_> = binding.lines().collect();

    let mut ans:usize = 0;

    for line in &contents {
        let tmp: Vec<_> = line.split_whitespace().collect();
        let mut new_pair = Pair::new();

        new_pair.line = tmp[0].to_string();
        new_pair.lengths = tmp[1].split(",").map(|x| x.parse().unwrap()).collect();
        
       ans += get_ans_row(&new_pair);
    }

    println!("{ans}");
}

fn get_ans_row(row: &Pair) -> usize {
    let count_q = row.line.matches("?").count();
    let mut ans: usize = 0;

    for i in 0..(1 << count_q) {
        let mut tmp = i;

        let mut copy_row = row.line.clone();

        for j in 0..copy_row.len() {
            if copy_row.chars().nth(j).unwrap() == '?' {
                if tmp & 1 == 0 {
                    copy_row.replace_range(j..j+1, "#");
                }
                else {
                    copy_row.replace_range(j..j+1, ".");
                }
                tmp >>= 1;
            }
        }

        ans += is_matching(&copy_row, &row.lengths) as usize;
    }

    ans
}

fn is_matching(row: &String, lengths: &Vec<usize>) -> bool {
    let row_lengths: Vec<_> = row.split(".").filter(|x| !x.is_empty()).collect();
    let row_lengths: Vec<usize> = row_lengths.iter().map(|x| x.len()).collect();
    row_lengths == *lengths
}

#[derive(Debug)]
struct Pair {
    line: String,
    lengths: Vec<usize>
}

impl Pair {
    fn new() -> Pair {
        Pair {
            line: String::from(""),
            lengths: Vec::new()
        }
    }
}