use std::fs::File;
use std::io::{self, BufRead};

pub fn solve_1() -> io::Result<u32> {
    // Open the file
    let file = File::open("src/cube_conundrum/input1")?;

    // Create a buffered reader to read lines efficiently
    let reader = io::BufReader::new(file);

    // Iterate over lines in the file
    let mut sum: u32 = 0;
    for line in reader.lines() {
        // Handle each line as needed
        match line {
            Ok(content) => {
                let game_split: Vec<&str> = content.split(": ").collect();
                let game_tokens: Vec<&str> = game_split[0].split_whitespace().collect();
                let game_number = game_tokens[1].parse::<u32>().unwrap();

                let rounds: Vec<&str> = game_split[1].split("; ").collect();
                let mut possible = true;

                for round in rounds {
                    let cubes: Vec<&str> = round.split(", ").collect();

                    for cube in cubes {
                        let cube_split: Vec<&str> = cube.split_whitespace().collect();
                        let cube_count: u32 = cube_split[0].parse().unwrap();

                        match cube_split[1] {
                            "red" => {
                                if cube_count > 12 {
                                    possible = false;
                                }
                            }
                            "green" => {
                                if cube_count > 13 {
                                    possible = false;
                                }
                            }
                            "blue" => {
                                if cube_count > 14 {
                                    possible = false;
                                }
                            }
                            _ => {
                                eprintln!("Invalid color: {}", cube_split[1]);
                            }
                        }
                    }
                }

                if possible {
                    sum += game_number;
                }
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    Ok(sum)
}

pub fn solve_2() -> io::Result<u32> {
    // Open the file
    let file = File::open("src/cube_conundrum/input1")?;

    // Create a buffered reader to read lines efficiently
    let reader = io::BufReader::new(file);

    // Iterate over lines in the file
    let mut sum: u32 = 0;
    for line in reader.lines() {
        // Handle each line as needed
        match line {
            Ok(content) => {
                let game_split: Vec<&str> = content.split(": ").collect();
                let game_tokens: Vec<&str> = game_split[0].split_whitespace().collect();
                let game_number = game_tokens[1].parse::<u32>().unwrap();

                let rounds: Vec<&str> = game_split[1].split("; ").collect();
                let mut mr = 0;
                let mut mg = 0;
                let mut mb = 0;

                for round in rounds {
                    let cubes: Vec<&str> = round.split(", ").collect();

                    for cube in cubes {
                        let cube_split: Vec<&str> = cube.split_whitespace().collect();
                        let cube_count: u32 = cube_split[0].parse().unwrap();

                        match cube_split[1] {
                            "red" => {
                                if cube_count > mr {
                                    mr = cube_count
                                }
                            }
                            "green" => {
                                if cube_count > mg {
                                    mg = cube_count;
                                }
                            }
                            "blue" => {
                                if cube_count > mb {
                                    mb = cube_count;
                                }
                            }
                            _ => {
                                eprintln!("Invalid color: {}", cube_split[1]);
                            }
                        }
                    }
                }

                sum += mr * mg * mb;
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    Ok(sum)
}
