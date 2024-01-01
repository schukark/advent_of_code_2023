use std::fs;
use std::collections::{HashMap, VecDeque, HashSet};

pub fn execute() {
    let binding = fs::read_to_string("txts/input.txt").expect("Can't open file");
    let contents = binding.lines().collect::<Vec<_>>();

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut edges: Vec<(String, String)> = Vec::new();

    for line in contents {
        let name_and_dest = line.split(": ").collect::<Vec<_>>();
        let destinations = name_and_dest[1].split(" ").map(|x| x.to_string()).collect::<Vec<_>>();
        let name = name_and_dest[0].to_string();

        for destination in &destinations {
            if name_and_dest[0].cmp(destination) == std::cmp::Ordering::Less {
                edges.push((name_and_dest[0].to_string(), destination.clone()));
            }
            else {
                edges.push((destination.clone(), name_and_dest[0].to_string()));
            }

            if let Some(tmp) = graph.get_mut(destination) {
                tmp.push(name_and_dest[0].to_string());
            }
            else {
                graph.insert(destination.clone(), vec![name_and_dest[0].to_string()]);
            }

            if let Some(tmp) = graph.get_mut(&name) {
                tmp.push(destination.clone());
            }
            else {
                graph.insert(name.clone(), vec![destination.clone()]);
            }
        }
    }

    println!("{:#?} {:#?}", graph.len(), edges.len());

    let edges_len = edges.len();

    for i in 0..edges_len {
        println!("Outer iteration {i} / {}", edges_len);
        for j in i+1..edges_len {
            println!("Inner iteration {j} / {}", edges_len);
            for k in j+1..edges_len {
                dfs(&graph, edges[0].0.clone(), vec![edges[i].clone(), edges[j].clone(), edges[k].clone()])
            }
        }
    }
}

fn dfs(graph: &HashMap<String, Vec<String>>, pos: String, forbidden: Vec<(String, String)>) {
    let mut visited = HashSet::<String>::new();

    let mut answer = Vec::<i32>::new();

    //println!("----------");
    
    for (key, _) in graph {
        if visited.contains(key) {
            continue;
        }

        if answer.len() > 2 {
            return;
        }
        
        //println!("Haven't been in {key}");
        let mut counter = 0;
        let mut stack = VecDeque::<&str>::new();
        
        stack.push_back(key);
        visited.insert(key.to_string());
        while !stack.is_empty() {
            let top = stack.pop_front().unwrap();
            counter += 1;

            //println!("{:#?}", top);

            for destination in graph.get(top).unwrap() {
                if visited.contains(destination) {
                    continue;
                }

                //println!("Going from {top} to {destination}");

                if check(&top.to_string(), destination, &forbidden) || check(destination, &top.to_string(), &forbidden) {
                    continue;
                }

                stack.push_back(destination);
                visited.insert(destination.to_string());
            }
        }

        answer.push(counter);
    }

    //println!("{:#?}", visited);

    if answer.len() == 2 {
        println!("Success");
        println!("{}", answer[0] * answer[1]);
        std::process::exit(0);
    }
    
}

fn check(from: &String, to: &String, forbidden: &Vec<(String, String)>) -> bool {
    for edge in forbidden {
        if edge.0 == *from && edge.1 == *to {
            return true;
        }
    }

    false
}
