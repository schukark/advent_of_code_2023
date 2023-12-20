use std::fs;

pub fn execute() {
    let binding = fs::read_to_string("txts/sample.txt").expect("Can't open file");
    let contents: Vec<_> = binding.lines().collect();

    let mut ans:usize = 0;

    for line in &contents {
        let tmp: Vec<_> = line.split_whitespace().collect();
        let mut new_pair = Pair::new();

        new_pair.line = clone_five_string(tmp[0].to_string());
        new_pair.lengths = clone_five_vectors(tmp[1].split(",").map(|x| x.parse().unwrap()).collect());

        //println!("{:#?}", new_pair);
        

        let tmp = get_ans_row(&new_pair);
        println!("{tmp}");
        ans += tmp;
    }

    println!("{ans}");
}

fn clone_five_string(line: String) -> String {
    let copy = &line.clone();
    line + &String::from("?") + copy + &String::from("?") + copy + &String::from("?") + copy + &String::from("?") + copy
}

fn clone_five_vectors(vec: Vec<usize>) -> Vec<usize> {
    let mut ans = vec.clone();
    for i in 0..4 {
        for j in &vec {
            ans.push(*j);
        }
    }

    ans
}

fn get_ans_row(row: &Pair) -> usize {
    let count_q = row.line.matches("?").count();
    let mut ans: usize = 0;

    for i in 0..(1usize << count_q) {
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