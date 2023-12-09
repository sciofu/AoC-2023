use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

use num_integer::lcm;

fn calculate_lcm(values: &Vec<u64>) -> u64 {
    values.iter().fold(1, |acc, &x| lcm(acc, x))
}

#[derive(Debug)]
struct Node {
    value: String,
    left: String,
    right: String,
}

pub fn solve_1() -> io::Result<u64> {
    // Open the file

    let file = File::open("src/haunted_wastelands/input")?;
    // Create a buffered reader to read lines efficiently
    let reader = io::BufReader::new(file);
    let lines = reader.lines();

    let mut steps: u64 = 0;
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut moves: Vec<char> = Vec::new();
    let mut starting_nodes: Vec<String> = Vec::new();

    for line in lines {
        match line {
            Ok(content) => {
                if content.contains(" = ") {
                    let tokens = content.split(" = ").collect::<Vec<&str>>();
                    let current_node = tokens[0].to_string().clone();

                    let children = tokens[1].to_string();
                    let string_children = children.split(", ").collect::<Vec<&str>>();
                    let mut left_str = string_children[0].to_string();
                    let mut right_str = string_children[1].to_string();

                    left_str.remove(0);
                    right_str.remove(right_str.len() - 1);

                    let node = Node {
                        value: current_node.clone(),
                        left: left_str,
                        right: right_str,
                    };

                    nodes.insert(current_node.clone(), node);

                    if current_node.ends_with('A') {
                        starting_nodes.push(current_node.clone());
                    }
                } else {
                    if content.len() > 0 {
                        moves = content.chars().collect::<Vec<char>>();
                    }
                }
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    let mut current_node = "AAA".to_string();
    let mut i: usize = 0;

    while current_node != "ZZZ" {
        let next_node = nodes.get(&current_node).unwrap();
        if moves[i] == 'L' {
            current_node = next_node.left.clone();
        } else {
            current_node = next_node.right.clone();
        }
        i += 1;
        i %= moves.len();
        steps += 1;
    }

    Ok(steps)
}

pub fn solve_2() -> io::Result<u64> {
    // Open the file

    let file = File::open("src/haunted_wastelands/input")?;
    // Create a buffered reader to read lines efficiently
    let reader = io::BufReader::new(file);
    let lines = reader.lines();

    let mut steps: u64 = 0;
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut moves: Vec<char> = Vec::new();
    let mut starting_nodes: Vec<String> = Vec::new();

    for line in lines {
        match line {
            Ok(content) => {
                if content.contains(" = ") {
                    let tokens = content.split(" = ").collect::<Vec<&str>>();
                    let current_node = tokens[0].to_string().clone();

                    let children = tokens[1].to_string();
                    let string_children = children.split(", ").collect::<Vec<&str>>();
                    let mut left_str = string_children[0].to_string();
                    let mut right_str = string_children[1].to_string();

                    left_str.remove(0);
                    right_str.remove(right_str.len() - 1);

                    let node = Node {
                        value: current_node.clone(),
                        left: left_str,
                        right: right_str,
                    };

                    nodes.insert(current_node.clone(), node);

                    if current_node.ends_with('A') {
                        starting_nodes.push(current_node.clone());
                    }
                } else {
                    if content.len() > 0 {
                        moves = content.chars().collect::<Vec<char>>();
                    }
                }
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    let mut nodes_on_z = 0;
    let mut i = 0;
    let mut current_nodes = starting_nodes.clone();
    let mut steps_map: HashMap<String, u64> = HashMap::new();

    println!("{:?}", current_nodes);

    for node in current_nodes {
        let mut current_node = node.clone();
        let mut i: usize = 0;
        steps = 0;

        while !current_node.ends_with('Z') {
            let next_node = nodes.get(&current_node).unwrap();
            if moves[i] == 'L' {
                current_node = next_node.left.clone();
            } else {
                current_node = next_node.right.clone();
            }
            i += 1;
            i %= moves.len();
            steps += 1;
        }

        steps_map.insert(node.clone(), steps);
    }

    let steps_values = steps_map
        .values()
        .map(|val| val.clone())
        .collect::<Vec<u64>>();

    let result = calculate_lcm(&steps_values);

    Ok(result)
}
