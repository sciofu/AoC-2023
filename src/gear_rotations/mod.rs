use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

pub fn solve_1() -> io::Result<u32> {
    // Open the file
    let file = File::open("src/gear_rotations/input1")?;

    // Create a buffered reader to read lines efficiently
    let reader = io::BufReader::new(file);

    // Iterate over lines in the file
    let mut sum: u32 = 0;
    let mut gear_rows: Vec<String> = Vec::new();
    for line in reader.lines() {
        // Handle each line as needed
        match line {
            Ok(content) => {
                gear_rows.push(content);
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
    //TODO: Change gearId to string of col-row
    for (row_index, elem) in gear_rows.iter().enumerate() {
        let gears = elem.chars().collect::<Vec<char>>();

        let mut column_index = 0;
        let mut current_number = 0;
        let mut touches_symbol = false;

        while column_index < gears.len() {
            let current_char = gears.get(column_index).unwrap();
            if current_char.is_digit(10) {
                current_number = current_number * 10 + current_char.to_digit(10).unwrap();

                check_neighbours(row_index, column_index, &gear_rows, &mut touches_symbol)
            } else {
                if touches_symbol {
                    sum += current_number;
                }

                touches_symbol = false;
                current_number = 0;
            }

            column_index += 1;
        }

        if touches_symbol {
            sum += current_number;
        }
    }

    Ok(sum)
}

pub fn solve_2() -> io::Result<u64> {
    // Open the file
    let file = File::open("src/gear_rotations/input1")?;

    // Create a buffered reader to read lines efficiently
    let reader = io::BufReader::new(file);

    // Iterate over lines in the file
    let mut sum: u64 = 0;
    let mut gear_rows: Vec<String> = Vec::new();
    for line in reader.lines() {
        // Handle each line as needed
        match line {
            Ok(content) => {
                gear_rows.push(content);
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    let mut gears_mult: HashMap<String, u64> = HashMap::new();
    let mut correct_gears: HashMap<String, u32> = HashMap::new();

    for (row_index, elem) in gear_rows.iter().enumerate() {
        let gears = elem.chars().collect::<Vec<char>>();

        let mut column_index = 0;
        let mut current_number = 0;
        let mut touched_gears: HashSet<String> = HashSet::new();

        while column_index < gears.len() {
            let current_char = gears.get(column_index).unwrap();
            if current_char.is_digit(10) {
                current_number = current_number * 10 + current_char.to_digit(10).unwrap();

                check_gear_neighbours(row_index, column_index, &gear_rows, &mut touched_gears)
            } else {
                // println!("{} -> touched gears: {:?}", current_number, touched_gears);
                for gear_id in touched_gears.iter() {
                    let gear_mult = gears_mult.get(gear_id).unwrap_or(&0);
                    if gear_mult == &0 {
                        gears_mult.insert((*gear_id).clone(), current_number as u64);
                        correct_gears.insert((*gear_id).clone(), 1);
                    } else {
                        gears_mult.insert((*gear_id).clone(), gear_mult * (current_number as u64));
                        correct_gears.insert(
                            (*gear_id).clone(),
                            correct_gears.get(gear_id).unwrap_or(&0) + 1,
                        );
                    }
                }
                // println!("{} -> gears mult after: {:?}", current_number, gears_mult);

                touched_gears.clear();
                current_number = 0;
            }

            column_index += 1;
        }

        for gear_id in touched_gears.iter() {
            let gear_mult = gears_mult.get(gear_id).unwrap_or(&0);
            if gear_mult == &0 {
                gears_mult.insert((*gear_id).clone(), current_number as u64);
                correct_gears.insert((*gear_id).clone(), 1);
            } else {
                gears_mult.insert((*gear_id).clone(), gear_mult * (current_number as u64));
                correct_gears.insert(
                    (*gear_id).clone(),
                    correct_gears.get(gear_id).unwrap_or(&0) + 1,
                );
            }
        }
    }

    for (gear_id, appearances) in correct_gears.iter() {
        println!("{} -> {} ", gear_id, appearances);
        if appearances.eq(&2) {
            sum += gears_mult.get(gear_id).unwrap_or(&0);
        }
    }
    Ok(sum)
}

fn check_neighbours(
    row_index: usize,
    column_index: usize,
    gear_rows: &Vec<String>,
    touches_symbol: &mut bool,
) {
    if row_index > 0 {
        let previous_row = gear_rows
            .get(row_index - 1)
            .unwrap()
            .chars()
            .collect::<Vec<char>>();

        check_possible_neighbours(&previous_row, column_index, touches_symbol)
    }

    if row_index < gear_rows.len() - 1 {
        let next_row = gear_rows
            .get(row_index + 1)
            .unwrap()
            .chars()
            .collect::<Vec<char>>();

        check_possible_neighbours(&next_row, column_index, touches_symbol)
    }

    let current_row = gear_rows
        .get(row_index)
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    check_possible_neighbours(&current_row, column_index, touches_symbol)
}

fn is_special_char(c: &char) -> bool {
    return !c.is_digit(10) && !c.eq_ignore_ascii_case(&'.');
}

fn check_possible_neighbours(row: &Vec<char>, column: usize, touches_symbol: &mut bool) {
    if column > 0 {
        let left = row.get(column - 1).unwrap();

        if is_special_char(left) {
            *touches_symbol = true;
            return;
        }
    }

    if column < row.len() - 1 {
        let right = row.get(column + 1).unwrap();

        if is_special_char(right) {
            *touches_symbol = true;
            return;
        }
    }

    let center = row.get(column).unwrap();
    if is_special_char(center) {
        *touches_symbol = true;
        return;
    }
}

fn check_gear_neighbours(
    row_index: usize,
    column_index: usize,
    gear_rows: &Vec<String>,
    touched_gears: &mut HashSet<String>,
) {
    if row_index > 0 {
        let previous_row = gear_rows
            .get(row_index - 1)
            .unwrap()
            .chars()
            .collect::<Vec<char>>();

        check_gear_possible_neighbours(&previous_row, column_index, row_index - 1, touched_gears)
    }

    if row_index < gear_rows.len() - 1 {
        let next_row = gear_rows
            .get(row_index + 1)
            .unwrap()
            .chars()
            .collect::<Vec<char>>();

        check_gear_possible_neighbours(&next_row, column_index, row_index + 1, touched_gears)
    }

    let current_row = gear_rows
        .get(row_index)
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    check_gear_possible_neighbours(&current_row, column_index, row_index, touched_gears)
}

fn check_gear_possible_neighbours(
    row: &Vec<char>,
    column: usize,
    row_id: usize,
    touched_gears: &mut HashSet<String>,
) {
    if column > 0 {
        let left = row.get(column - 1).unwrap();

        if left.eq_ignore_ascii_case(&'*') {
            let gear_id = format!("{}-{}", row_id, column - 1);
            touched_gears.insert(gear_id);
        }
    }

    if column < row.len() - 1 {
        let right = row.get(column + 1).unwrap();

        if right.eq_ignore_ascii_case(&'*') {
            let gear_id = format!("{}-{}", row_id, column + 1);
            touched_gears.insert(gear_id);
        }
    }

    let center = row.get(column).unwrap();
    if center.eq_ignore_ascii_case(&'*') {
        let gear_id = format!("{}-{}", row_id, column);
        touched_gears.insert(gear_id);
    }
}
