use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

pub fn solve_1() -> io::Result<u64> {
    // Open the file
    let file = File::open("src/scratchcards/input")?;

    // Create a buffered reader to read lines efficiently
    let reader = io::BufReader::new(file);

    // Iterate over lines in the file
    let mut sum: u64 = 0;
    let mut winning_numbers_set: HashSet<u32> = HashSet::new();

    for line in reader.lines() {
        // Handle each line as needed
        match line {
            Ok(content) => {
                let mut count = 0;

                let card: Vec<&str> = content.split(": ").collect();
                let number_lists: Vec<&str> = card[1].split("|").collect();
                let winning_numbers = string_to_array(number_lists[0]);
                for number in winning_numbers {
                    winning_numbers_set.insert(number);
                }

                let playable_numbers = string_to_array(number_lists[1]);
                for number in playable_numbers {
                    if winning_numbers_set.contains(&number) {
                        count += 1;
                    }
                }

                if count > 0 {
                    sum += 2_u64.pow(count - 1);
                }
                winning_numbers_set.clear();
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    Ok(sum)
}

pub fn solve_2() -> io::Result<u64> {
    // Open the file
    let file = File::open("src/scratchcards/input")?;

    // Create a buffered reader to read lines efficiently
    let reader = io::BufReader::new(file);

    // Iterate over lines in the file
    let mut sum: u64 = 0;
    let mut winning_numbers_set: HashSet<u32> = HashSet::new();

    let mut cards_count: HashMap<u32, u64> = HashMap::new();

    for line in reader.lines() {
        // Handle each line as needed
        match line {
            Ok(content) => {
                let mut count = 0;

                let card: Vec<&str> = content.split(": ").collect();
                let card_number_tokens: Vec<&str> = card[0].split_whitespace().collect();
                let card_number = card_number_tokens[1].parse::<u32>().unwrap();

                let number_lists: Vec<&str> = card[1].split("|").collect();
                let winning_numbers = string_to_array(number_lists[0]);
                for number in winning_numbers {
                    winning_numbers_set.insert(number);
                }

                let playable_numbers = string_to_array(number_lists[1]);

                for number in playable_numbers {
                    if winning_numbers_set.contains(&number) {
                        count += 1;
                    }
                }

                for i in (card_number + 1)..=(card_number + count) {
                    let current_card_multiplier = cards_count.get(&card_number).unwrap_or(&1);
                    let card_count = cards_count.get(&i).unwrap_or(&1);
                    cards_count.insert(i, current_card_multiplier + card_count);
                }

                sum += cards_count.get(&card_number).unwrap_or(&1);

                winning_numbers_set.clear();
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    Ok(sum)
}

fn string_to_array(numbers: &str) -> Vec<u32> {
    let tokens: Vec<&str> = numbers.trim().split_whitespace().collect();
    let numbers: Vec<u32> = tokens.iter().map(|x| x.parse::<u32>().unwrap()).collect();
    return numbers;
}
