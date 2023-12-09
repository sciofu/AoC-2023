use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub fn solve_1() -> io::Result<u32> {
    // Open the file
    let file = File::open("src/trebuchet/input1")?;

    // Create a buffered reader to read lines efficiently
    let reader = io::BufReader::new(file);

    // Iterate over lines in the file
    let mut sum: u32 = 0;
    for line in reader.lines() {
        // Handle each line as needed
        match line {
            Ok(content) => {
                let first_digit: Option<u32> = content
                    .chars()
                    .find(|c: &char| c.is_digit(10))
                    .and_then(|c| c.to_digit(10));

                let last_digit = content
                    .chars()
                    .rev()
                    .find(|c: &char| c.is_digit(10))
                    .and_then(|c| c.to_digit(10));

                if let (Some(fd), Some(ld)) = (first_digit, last_digit) {
                    let number = fd * 10 + ld;
                    sum += number;
                }
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    Ok(sum)
}

pub fn solve_2() -> io::Result<u32> {
    // Open the file
    let file = File::open("src/trebuchet/input1")?;

    // Create a buffered reader to read lines efficiently
    let reader = io::BufReader::new(file);

    let mut digits: HashMap<&str, u32> = HashMap::new();

    // Insert key-value pairs
    digits.insert("one", 1);
    digits.insert("two", 2);
    digits.insert("three", 3);
    digits.insert("four", 4);
    digits.insert("five", 5);
    digits.insert("six", 6);
    digits.insert("seven", 7);
    digits.insert("eight", 8);
    digits.insert("nine", 9);

    // Iterate over lines in the file
    let mut sum: u32 = 0;
    for line in reader.lines() {
        // Handle each line as needed
        match line {
            Ok(content) => {
                let first_digit = get_first_digit(&content, &digits);

                println!("{} -> {}", content, first_digit);

                let last_digit = get_last_digit(&content, &digits);
                println!("{} -> {}", content, last_digit);

                let number = first_digit * 10 + last_digit;
                println!("{} -> {}", content, number);
                sum += number;
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    Ok(sum)
}

fn get_first_digit(content: &str, digits: &HashMap<&str, u32>) -> u32 {
    let mut pos = content.len();
    let mut first_digit: u32 = 0;

    for (key, value) in digits {
        if let Some(p) = content.find(key) {
            if p < pos {
                pos = p;
                first_digit = *value;
            }
        }
    }

    if let Some(p) = content.chars().position(|c| c.is_numeric()) {
        if p < pos {
            pos = p;
            first_digit = content.chars().nth(p).unwrap().to_digit(10).unwrap();
        }
    }

    return first_digit;
}

fn get_last_digit(content: &str, digits: &HashMap<&str, u32>) -> u32 {
    let mut pos = 0;
    let mut first_digit: u32 = 0;

    for (key, value) in digits {
        if let Some(p) = content.rfind(key) {
            if p > pos {
                pos = p;
                first_digit = *value;
            }
        }
    }

    if let Some(p) = content.chars().rev().position(|c| c.is_numeric()) {
        println!("p:{} pos:{}", p, pos);
        if p < content.len() - pos {
            pos = p;
            first_digit = content.chars().rev().nth(p).unwrap().to_digit(10).unwrap();
        }
    }

    return first_digit;
}
