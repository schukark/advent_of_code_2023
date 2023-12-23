use std::fs;

pub fn execute() {
    let binding = fs::read_to_string("txts/input.txt").expect("Can't open file");
    let contents: Vec<_> = binding.split(",").collect();

    let mut boxes: Vec<Vec<Pair>> = Vec::with_capacity(256);

    for i in 0..256 {
        boxes.push(Vec::new());
    }

    for expr in contents {
        if expr.matches("=").count() == 1 {
            assign(&mut boxes, expr);
        }
        else {
            remove(&mut boxes, expr);
        }
    }

    let mut ans: usize = 0;

    for (i, container) in boxes.iter().enumerate() {
        for (j, lens) in container.iter().enumerate() {
            ans += (i + 1) * (j + 1) * lens.power as usize;
        }
    }

    println!("{ans}");
}

#[derive(Debug)]
struct Pair {
    name: String,
    power: i32,
}

fn remove(boxes: &mut Vec<Vec<Pair>>, expr: &str) {
    let label = &expr[0..expr.len() - 1];

    let label_hash = hash_string(label);
    let index = boxes[label_hash as usize].iter().position(|x| x.name == label);

    if let Some(tmp) = index {
        for i in tmp..boxes[label_hash as usize].len() - 1 {
            let tmp_value = &boxes[label_hash as usize][i + 1];
            boxes[label_hash as usize][i] = Pair {name: tmp_value.name.clone(), power: tmp_value.power};
        }
        boxes[label_hash as usize].pop();
    }
}

fn assign(boxes: &mut Vec<Vec<Pair>>, expr: &str) {
    let content: Vec<_> = expr.split("=").collect();
    let label = content[0];

    let new_power = content[1].parse().unwrap();

    let label_hash = hash_string(label);
    let index = boxes[label_hash as usize].iter().position(|x| x.name == label);

    if let Some(tmp) = index {
        boxes[label_hash as usize][tmp].power = new_power;
    }
    else {
        boxes[label_hash as usize].push(Pair {name: label.to_string(), power: new_power});
    }
}

fn hash_string(line: &str) -> u16 {
    let mut ans: u16 = 0;

    for char in line.chars() {
        ans += char as u16;
        ans = (ans * 17 as u16) % 256;
    }

    ans
}