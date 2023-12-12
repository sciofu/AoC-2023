use std::collections::HashMap;
use std::fs;
use std::io::{self};

#[derive(Debug)]
struct Rule {
    destination: u64,
    source: u64,
    count: u64,
}

impl Rule {
    fn find_destination(&self, source: u64) -> Option<u64> {
        // println!("source: {}", source);
        // println!("self.source: {}", self.source);
        // println!("self.count: {}", self.count);
        // println!("self.destination: {}", self.destination);

        if source >= self.source && source <= self.source + self.count {
            return Some(self.destination + (source - self.source));
        }
        return None;
    }

    fn find_backwards_destionation(&self, destination: u64) -> Option<u64> {
        if destination >= self.destination && destination < self.destination + self.count {
            return Some(self.source + (destination - self.destination));
        }
        return None;
    }
}

fn parse_category(category: String) -> Vec<Rule> {
    let mut rules: Vec<Rule> = Vec::new();

    let lines: Vec<&str> = category.split("\n").collect();
    let rules_lines = lines[1..].to_vec();

    for rule_line in rules_lines {
        let rules_numbers: Vec<u64> = rule_line
            .split(" ")
            .map(|x| x.parse::<u64>().unwrap_or(0))
            .collect();

        rules.push(Rule {
            destination: rules_numbers[0],
            source: rules_numbers[1],
            count: rules_numbers[2],
        })
    }

    return rules;
}

pub fn solve_1() -> io::Result<u64> {
    // Open the file

    let file_path = "src/fertilizer/input";

    // Read the entire content of the file into a string
    let file_content = fs::read_to_string(file_path)?;

    let categories: Vec<&str> = file_content.split("\n\n").collect();

    // println!("{:?}", categories);
    let seeds: Vec<u64> = categories[0]
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap_or(0))
        .collect();

    // println!("seeds: {:?}", seeds);

    let mut rules_list: Vec<Vec<Rule>> = Vec::new();

    for category in categories[1..].to_vec() {
        rules_list.push(parse_category(category.to_string()));
    }

    // println!("rules: {:?}", rules_list);

    let mut lowest_location: u64 = u64::MAX;

    for seed in seeds[1..].to_vec() {
        let mut current_location = seed;
        println!("seed: {}", seed);

        for category in rules_list.iter() {
            for rule in category.iter() {
                if let Some(destination) = rule.find_destination(current_location) {
                    current_location = destination;
                    break;
                }
            }
        }

        if current_location < lowest_location {
            println!("lowest_location seed: {}", seed);
            lowest_location = current_location;
        }
    }

    Ok(lowest_location)
}

pub fn solve_2() -> io::Result<u64> {
    // Open the file

    let file_path = "src/fertilizer/input";

    // Read the entire content of the file into a string
    let file_content = fs::read_to_string(file_path)?;

    let categories: Vec<&str> = file_content.split("\n\n").collect();

    // println!("{:?}", categories);
    let seeds: Vec<u64> = categories[0]
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap_or(0))
        .collect();

    // println!("seeds: {:?}", seeds);

    let seeds_pairs: Vec<(u64, u64)> = seeds[1..]
        .to_vec()
        .chunks(2)
        .filter_map(|chunk| {
            if let [a, b] = chunk {
                Some((*a, *b))
            } else {
                None
            }
        })
        .collect();

    println!("seeds_pairs: {:?}", seeds_pairs);

    let mut rules_list: Vec<Vec<Rule>> = Vec::new();

    for category in categories[1..].to_vec() {
        rules_list.push(parse_category(category.to_string()));
    }

    // println!("rules: {:?}", rules_list);

    let mut lowest_location: u64 = u64::MAX;

    let mut loc = 0;
    loop {
        let mut current_location = loc;

        for category in rules_list.iter().rev() {
            // println!("current_location: {}", current_location);
            for rule in category.iter() {
                if let Some(destination) = rule.find_backwards_destionation(current_location) {
                    current_location = destination;
                    break;
                }
            }
        }

        if is_seed(&seeds_pairs, current_location) {
            // println!("found seed: {}", current_location);
            // println!("lowest_location seed: {}", loc);
            lowest_location = loc;
            break;
        }

        loc += 1;
    }

    Ok(lowest_location)
}

fn is_seed(seeds_pairs: &Vec<(u64, u64)>, seed: u64) -> bool {
    for seeds_pair in seeds_pairs.iter() {
        // println!("seeds_pair: {:?}", seeds_pair);
        if seeds_pair.0 <= seed && seeds_pair.0 + seeds_pair.1 > seed {
            return true;
        }
    }
    return false;
}
