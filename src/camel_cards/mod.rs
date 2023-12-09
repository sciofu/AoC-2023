use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::sync::Mutex;

lazy_static! {
    static ref MY_HASHMAP: Mutex<HashMap<char, u32>> = Mutex::new(HashMap::new());
}

#[derive(Debug)]
struct Hand {
    cards: String,
    bet: u64,
    strength: u64,
}

impl Hand {
    fn compute_strength(&mut self) {
        let mut cards_count: HashMap<char, u64> = HashMap::new();

        let mut j_count = 0;

        for c in self.cards.chars() {
            if c != 'J' {
                let count = cards_count.get(&c);
                match count {
                    Some(value) => cards_count.insert(c, value + 1),
                    None => cards_count.insert(c, 1),
                };
            } else {
                j_count += 1;
            }
        }

        let mut groups: Vec<u64> = [0, 0, 0, 0, 0, 0].to_vec();

        for (key, value) in cards_count {
            groups[value as usize] += 1;
        }

        if groups[5] == 1 {
            self.strength = 7
        } else if groups[4] == 1 {
            if j_count == 1 {
                self.strength = 7
            } else {
                self.strength = 6;
            }
        } else if groups[3] == 1 && groups[2] == 1 {
            if j_count == 2 {
                self.strength = 7
            } else {
                self.strength = 5;
            }
            self.strength = 5;
        } else if groups[3] == 1 {
            let mut bonus = 0;
            if j_count > 0 {
                bonus = j_count + 1;
            }
            self.strength = 4 + bonus;
        } else if groups[2] == 2 {
            if j_count == 1 {
                self.strength = 5;
            } else {
                self.strength = 3;
            }
        } else if groups[2] == 1 {
            if j_count == 1 {
                self.strength = 4
            } else if j_count == 2 {
                self.strength = 6;
            } else if j_count == 3 {
                self.strength = 7;
            } else {
                self.strength = 2;
            }
        } else {
            if j_count >= 4 {
                self.strength = 7;
            } else if j_count == 3 {
                self.strength = 6;
            } else if j_count == 2 {
                self.strength = 4;
            } else if j_count == 1 {
                self.strength = 2;
            } else {
                self.strength = 1;
            }
        }
    }

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Compare the 'id' field
        self.strength
            .cmp(&other.strength)
            .then_with(|| compare_cards(&self.cards, &other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.strength == other.strength && self.cards == other.cards
    }
}

impl Eq for Hand {}

pub fn solve_1() -> io::Result<u64> {
    setup_scores();

    // Open the file
    let file = File::open("src/camel_cards/input")?;
    // Create a buffered reader to read lines efficiently
    let reader = io::BufReader::new(file);
    let lines = reader.lines();

    let mut winnings: u64 = 0;
    let mut hands: Vec<Hand> = Vec::new();

    for line in lines {
        match line {
            Ok(content) => {
                let tokens = content.split_whitespace().collect::<Vec<&str>>();
                let mut hand = Hand {
                    cards: tokens[0].to_string(),
                    bet: tokens[1].parse::<u64>().unwrap_or(0),
                    strength: 0,
                };
                hand.compute_strength();
                hands.push(hand);
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    hands.sort_by(|a, b| a.cmp(&b));

    for (index, hand) in hands.iter().enumerate() {
        let rank = (index + 1) as u64;
        winnings += rank * hand.bet;
    }

    Ok(winnings)
}

fn insert_into_hashmap(key: char, value: u32) {
    // Access the global hashmap and insert a key-value pair
    MY_HASHMAP.lock().unwrap().insert(char::from(key), value);
}

fn get_from_hashmap(key: char) -> Option<u32> {
    // Access the global hashmap and retrieve a value
    MY_HASHMAP.lock().unwrap().get(&key).cloned()
}

fn setup_scores() {
    insert_into_hashmap('A', 14);
    insert_into_hashmap('K', 13);
    insert_into_hashmap('Q', 12);
    insert_into_hashmap('J', 1);
    insert_into_hashmap('T', 10);
    insert_into_hashmap('9', 9);
    insert_into_hashmap('8', 8);
    insert_into_hashmap('7', 7);
    insert_into_hashmap('6', 6);
    insert_into_hashmap('5', 5);
    insert_into_hashmap('4', 4);
    insert_into_hashmap('3', 3);
    insert_into_hashmap('2', 2);
}

fn compare_cards(card1: &String, card2: &String) -> std::cmp::Ordering {
    let mut i = 0;
    while i < 5 {
        let score1 = get_from_hashmap(card1.chars().nth(i).unwrap());
        let score2 = get_from_hashmap(card2.chars().nth(i).unwrap());
        if score1 == score2 {
            i = i + 1;
        } else {
            return score1.cmp(&score2);
        }
    }

    return std::cmp::Ordering::Equal;
}
