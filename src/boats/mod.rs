use std::fs::File;
use std::io::{self, BufRead};

pub fn solve_1() -> io::Result<u64> {
    // Open the file

    // Open the file
    let file = File::open("src/boats/input")?;

    // Create a buffered reader to read lines efficiently
    let reader = io::BufReader::new(file);
    let lines = reader.lines();

    let mut mult: u64 = 1;
    let mut times: Vec<u64> = Vec::new();
    let mut distances: Vec<u64> = Vec::new();

    // Handle each line as needed
    let mut i = 0;
    for line in lines {
        match line {
            Ok(content) => {
                let numbers: Vec<&str> = content.split(":").collect();

                if i == 0 {
                    times = parse_line(&numbers[1].to_string());
                } else {
                    distances = parse_line(&numbers[1].to_string());
                }
                i += 1;
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    for (i, time) in times.iter().enumerate() {
        let distance = distances[i] as f64;
        let tt: f64 = *time as f64;
        let delta = tt * tt - 4.0 * distance;

        let x1: f64 = (tt - delta.sqrt()) / 2.0;
        let x2: f64 = (tt + delta.sqrt()) / 2.0;

        let start = x1.floor() as u64;
        let end = x2.ceil() as u64;
        let t = *time as u64;
        let d = distance as u64;

        let mut count = 0;

        println!("start: {}", start);
        println!("end: {}", end);
        println!("t: {}", t);
        println!("d: {}", d);

        for elapsed in start..end {
            if elapsed < t && elapsed * (t - elapsed) > d {
                println!("elapsed: {}", elapsed);
                count += 1;
            }
        }

        mult *= count;
    }

    // Read the entire content of the file into a string

    Ok(mult)
}

fn parse_line(line: &String) -> Vec<u64> {
    return line
        .split_whitespace()
        .map(|f| f.parse::<u64>().unwrap())
        .collect();
}

pub fn solve_2() -> io::Result<u64> {
    // Open the file

    // Open the file
    let file = File::open("src/boats/input")?;

    // Create a buffered reader to read lines efficiently
    let reader = io::BufReader::new(file);
    let lines = reader.lines();

    let mut mult: u64 = 1;
    let mut times: Vec<u64> = Vec::new();
    let mut distances: Vec<u64> = Vec::new();

    // Handle each line as needed
    let mut i = 0;
    for line in lines {
        match line {
            Ok(content) => {
                let numbers: Vec<&str> = content.split(":").collect();

                if i == 0 {
                    times = parse_line(&numbers[1].to_string());
                } else {
                    distances = parse_line(&numbers[1].to_string());
                }
                i += 1;
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    let mut string_time = times.iter().map(|d| d.to_string()).collect::<Vec<String>>();
    let mut string_distance = distances
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<String>>();

    let mut time = string_time.join("").parse::<f64>().unwrap();
    let mut distance = string_distance.join("").parse::<f64>().unwrap();

    let delta = time * time - 4.0 * distance;

    let x1: f64 = (time - delta.sqrt()) / 2.0;
    let x2: f64 = (time + delta.sqrt()) / 2.0;

    let start = x1.floor() as u64;
    let end = x2.ceil() as u64;
    let t = time as u64;
    let d = distance as u64;

    let mut count = 0;

    println!("start: {}", start);
    println!("end: {}", end);
    println!("t: {}", t);
    println!("d: {}", d);

    for elapsed in start..end {
        if elapsed < t && elapsed * (t - elapsed) > d {
            println!("elapsed: {}", elapsed);
            count += 1;
        }
    }

    mult *= count;

    // Read the entire content of the file into a string

    Ok(mult)
}
