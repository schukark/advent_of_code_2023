use std::{fs, cmp, collections::HashMap};

pub fn execute() {
    let binding = fs::read_to_string("txts/input.txt").expect("Can't read file");
    let contents: Vec<_> = binding.lines().collect();

    let mut workflows: HashMap<String, Workflow> = HashMap::new();

    let mut index = 0;

    for line in &contents {
        if *line == "" || *line == "\n" {
            break;
        }

        let workflow = Workflow::new(line.to_string());
        let name = workflow.name.clone();
        workflows.insert(name, workflow);

        index += 1;
    }

    //println!("{:#?}", workflows);

    index += 1;

    let mut answer: i32 = 0;

    for i in index..contents.len() {
        let cur = Part::new(contents[i].to_string());
        let tmp_ans = cur.process(&workflows);
        //println!("{tmp_ans}");
        
        if tmp_ans {
            answer += cur.sum_param();
        }
    }

    println!("{answer}");
}

#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32
}

impl Part {
    fn new(line: String) -> Part {
        let binding = line[1..line.len() - 1].to_string();
        let splitted: Vec<_> = binding.split(",").collect();
        let mut part: Part = Part{x: 0, m: 0, a: 0, s: 0};

        for param in splitted {
            match param.chars().nth(0).unwrap() {
                'x' => part.x = param[2..].parse().unwrap(),
                'm' => part.m = param[2..].parse().unwrap(),
                'a' => part.a = param[2..].parse().unwrap(),
                's' => part.s = param[2..].parse().unwrap(),
                _ => panic!("Wrong part stat"),
            }
        }

        part
    }

    fn get_stat(&self, symbol: &Statistic) -> i32 {
        match symbol {
            Statistic::X => self.x,
            Statistic::M => self.m,
            Statistic::A => self.a,
            Statistic::S => self.s,
            _ => panic!("Wrong symbol in get_stat"),
        }
    }

    fn process(&self, workflows: &HashMap<String, Workflow>) -> bool {
        let mut next = &String::from("in");
        //println!("{:#?}", self);

        loop {
            //println!("{next}");
            'inner: loop {
                for rule in &workflows.get(next).unwrap().rules {
                    let value = self.get_stat(&rule.stat);
                    let result = rule.apply(value);

                    match result {
                        Exit::Accepted => {
                            //println!("Accepted");
                            return true;
                        },
                        Exit::Rejected => {
                            //println!("Rejected");
                            return false;
                        },
                        Exit::Continue => {
                            continue;
                        },
                        Exit::Workflow(workflow) => {
                            //println!("Changed workflow");
                            next = workflow;
                            break 'inner;
                        },
                    };
                }
            }
        }
    }

    fn sum_param(&self) -> i32 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug)]
enum Exit {
    Workflow(String),
    Accepted,
    Rejected,
    Continue,
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn new(line: String) -> Workflow {
        let index = line.find('{').unwrap();
        let name = line[0..index].to_string();

        let mut rules: Vec<Rule> = Vec::new();
        let size = line.len();

        //println!("{:#?}", line[index + 1..size - 1].split(",").collect::<Vec<&str>>());

        for rule in line[index + 1..size - 1].split(",") {
            rules.push(Rule::new(rule.to_string()));
        }

        Workflow {name, rules}
    }
}

// if cmp == cmp::Equal, then pass it no matter what the parameter is
// it is boiler plate code to not implement more enum variants
#[derive(Debug)]
struct Rule {
    stat: Statistic,
    cmp: cmp::Ordering,
    cutoff: i32,
    exit: Exit,
}

#[derive(Debug)]
enum Statistic {
    X, M, A, S,
}

impl Rule {
    fn new(line: String) -> Rule {
        let mut splitted: Vec<_> = Vec::new();
        let mut cmp: cmp::Ordering;

        if line.contains('>') {
            splitted = line.split(">").collect();
            cmp = cmp::Ordering::Greater;
        }
        else if line.contains('<'){
            splitted = line.split("<").collect();
            cmp = cmp::Ordering::Less;
        }
        else {
            if line.len() >= 2 {
                return Rule {stat: Statistic::A, cmp: cmp::Ordering::Equal, cutoff: 0, exit: Exit::Workflow(line)};
            }
            else if line.eq(&String::from("A")) {
                return Rule {stat: Statistic::A, cmp: cmp::Ordering::Equal, cutoff: 0, exit: Exit::Accepted};
            }
            return Rule {stat: Statistic::A, cmp: cmp::Ordering::Equal, cutoff: 0, exit: Exit::Rejected};
        }

        let new_end: Vec<_> = splitted[1].split(":").collect();
        splitted.pop();
        splitted.push(new_end[0]);
        splitted.push(new_end[1]);

        let stat = match splitted[0].chars().nth(0).unwrap() {
            'a' => Statistic::A,
            'x' => Statistic::X,
            'm' => Statistic::M,
            's' => Statistic::S,
            _ => panic!("Wrong symbol"),
        };

        let exit: Exit;
        if splitted[2].to_string().eq(&String::from("A")) {
            exit = Exit::Accepted;
        }
        else if splitted[2].to_string().eq(&String::from("R")) {
            exit = Exit::Rejected;
        }
        else {
            exit = Exit::Workflow(splitted[2].to_string());
        }

        let cutoff: i32 = splitted[1].parse().unwrap();

        Rule {stat, cmp, cutoff, exit}
    }

    fn apply(&self, value: i32) -> &Exit {
        if self.cmp.is_eq() {
            return &self.exit;
        }
        else if self.cutoff.cmp(&value) != self.cmp{
            return &self.exit;
        }
        &Exit::Continue
    }
}