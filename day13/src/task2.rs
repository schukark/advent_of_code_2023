use std::fs;

pub fn execute() {
    let binding = fs::read_to_string("txts/input.txt").expect("Can't open file");
    let contents: Vec<_> = binding.lines().collect();


    let n = contents.len();
    let m = contents[n - 1].len();

    let mut pictures: Vec<Vec<&str>> = Vec::new();
    pictures.push(Vec::new());

    for line in contents {
        if line == "" {
            pictures.push(Vec::new());
        }
        else {
            let len = pictures.len();
            pictures[len - 1].push(line);
        }
    }

    let mut hor = 0;
    let mut vert = 0;

    for picture in pictures {
        let n_pic = picture.len();
        let m_pic = picture[n_pic - 1].len();

        for i in 0..n_pic - 1 {
            if check_horizontal(&picture, i, n_pic) {
                hor += i + 1;
            }
        }

        for i in 0..m_pic - 1 {
            if check_vertical(&picture, i, m_pic) {
                vert += i + 1;
            }
        }
    }

    println!("{}", 100 * hor + vert);
}

fn check_horizontal(contents: &Vec<&str>, index: usize, row_num: usize) -> bool {
    let (mut index_l, mut index_r) = (index as isize, (index + 1) as isize);
    let mut ans = 0;

    while index_l >= 0 && index_r < row_num as isize {
        for i in 0..contents[0].len() {
            if contents[index_l as usize].chars().nth(i).unwrap() != contents[index_r as usize].chars().nth(i).unwrap() {
                ans += 1;
            }
        } 
        index_l -= 1;
        index_r += 1;
    }

    ans == 1
}

fn check_vertical(contents: &Vec<&str>, index: usize, col_num: usize) -> bool {
    let (mut index_l, mut index_r) = (index as isize, (index + 1) as isize);
    let mut ans = 0;

    while index_l >= 0 && index_r < col_num as isize {
        for i in 0..contents.len() {
            if contents[i].chars().nth(index_l as usize).unwrap() != contents[i].chars().nth(index_r as usize).unwrap() {
                ans += 1;
            }
        }
        index_l -= 1;
        index_r += 1;
    }

    ans == 1
}