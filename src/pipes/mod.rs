use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec;

use tokio::net::unix::pipe;

fn get_loop_size(
    pipe_map: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    steps: &mut u64,
    origin: char,
    path: &mut Vec<(usize, usize)>,
) -> bool {
    if pipe_map[i][j] == 'S' {
        return true;
    }

    let directions: Vec<(i32, i32)> = get_possible_directions(pipe_map[i][j], origin);
    // println!("{:?} {} {}", directions, i, j);

    for direction in directions {
        let new_i = i as i32 + direction.0;
        let new_j = j as i32 + direction.1;

        if check_valid_position(pipe_map, new_i as i32, new_j as i32) {
            *steps = *steps + 1;

            path.push((new_i as usize, new_j as usize));

            return get_loop_size(
                pipe_map,
                new_i as usize,
                new_j as usize,
                steps,
                get_origin(direction.0, direction.1),
                path,
            );
        }
    }

    return false;
}

fn check_valid_position(pipe_map: &Vec<Vec<char>>, i: i32, j: i32) -> bool {
    if i < 0 || j < 0 {
        return false;
    }

    if i as usize >= pipe_map.len() || j as usize >= pipe_map[0].len() {
        return false;
    }

    if pipe_map[i as usize][j as usize] == '.' {
        return false;
    }

    return true;
}

fn get_possible_directions(current_pipe: char, origin: char) -> Vec<(i32, i32)> {
    if current_pipe == 'S' {
        return vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    }

    if current_pipe == '|' {
        if origin == 'n' {
            return vec![(1, 0)];
        } else {
            return vec![(-1, 0)];
        }
    }

    if current_pipe == '-' {
        if origin == 'w' {
            return vec![(0, 1)];
        } else {
            return vec![(0, -1)];
        }
    }

    if current_pipe == 'L' {
        if origin == 'n' {
            return vec![(0, 1)];
        } else {
            return vec![(-1, 0)];
        }
    }

    if current_pipe == 'F' {
        if origin == 's' {
            return vec![(0, 1)];
        } else {
            return vec![(1, 0)];
        }
    }

    if current_pipe == '7' {
        if origin == 'w' {
            return vec![(1, 0)];
        } else {
            return vec![(0, -1)];
        }
    }

    if current_pipe == 'J' {
        if origin == 'w' {
            return vec![(-1, 0)];
        } else {
            return vec![(0, -1)];
        }
    }

    return vec![(1, 0)];
}

fn get_origin(i: i32, j: i32) -> char {
    if i == 0 && j == 1 {
        return 'w';
    }

    if i == 0 && j == -1 {
        return 'e';
    }

    if i == 1 && j == 0 {
        return 'n';
    }

    if i == -1 && j == 0 {
        return 's';
    }

    println!("Should not happen");
    return 's';
}

pub fn solve_1() -> io::Result<u64> {
    // Open the file

    let file = File::open("src/pipes/input")?;
    // Create a buffered reader to read lines efficiently
    let reader = io::BufReader::new(file);
    let lines = reader.lines();

    let mut res: u64 = 0;
    let mut pipe_map: Vec<Vec<char>> = Vec::new();

    let mut i = 0;
    let mut start_i = 0;
    let mut start_j = 0;

    for line in lines {
        match line {
            Ok(content) => {
                let row = content.chars().collect::<Vec<char>>();

                for j in 0..row.len() {
                    if row[j] == 'S' {
                        start_i = i;
                        start_j = j;
                    }
                }

                pipe_map.push(row);
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
        i += 1;
    }

    let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut steps = 0;
    let mut max_steps = u64::MIN;
    let mut path: Vec<(usize, usize)> = Vec::new();
    let mut max_path: Vec<(usize, usize)> = Vec::new();

    println!("{} {}", start_i, start_j);
    for direction in directions {
        let new_i = start_i as i32 + direction.0;
        let new_j = start_j as i32 + direction.1;
        // println!("{} {}", new_i, new_j);

        if check_valid_position(&pipe_map, new_i, new_j) {
            steps = 1;
            // println!("origin {}", get_origin(direction.0, direction.1));

            path.clear();
            path.push((new_i as usize, new_j as usize));
            let is_loop = get_loop_size(
                &pipe_map,
                new_i as usize,
                new_j as usize,
                &mut steps,
                get_origin(direction.0, direction.1),
                &mut path,
            );

            if is_loop {
                if steps > max_steps {
                    max_steps = steps;
                    max_path = path.clone();
                }
            }
        }
    }

    // println!("path {:?}", max_path);
    let n = pipe_map.len();
    let m = pipe_map[0].len();

    for i in 0..n {
        let mut is_out = true;

        for j in 0..m {
            let curr = pipe_map[i][j];

            if max_path.contains(&(i, j))
                && (curr == 'F' || curr == '7' || curr == '|' || curr == 'S')
            {
                is_out = !is_out
            }

            if !is_out && !max_path.contains(&(i, j)) {
                res += 1;
                pipe_map[i][j] = 'I';
            }

            if is_out {
                pipe_map[i][j] = '0';
            }
        }
    }

    print_mat(&pipe_map);

    Ok(res)
}

// won't work for part 2 :/
fn fill(fill_map: &mut Vec<Vec<i32>>, i: usize, j: usize, size: &mut u64, is_outside: &mut bool) {
    let directions = vec![
        (-1, 0),
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    let n = fill_map.len();
    let m = fill_map[0].len();

    for direction in directions {
        let new_i = i as i32 + direction.0;
        let new_j = j as i32 + direction.1;

        if is_on_map(new_i, new_j, n, m) {
            if fill_map[new_i as usize][new_j as usize] == 0 {
                fill_map[new_i as usize][new_j as usize] = 1;
                *size += 1;
                fill(fill_map, new_i as usize, new_j as usize, size, is_outside);
            }
        } else {
            *is_outside = true;
        }
    }
}

fn is_on_map(i: i32, j: i32, n: usize, m: usize) -> bool {
    if i < 0 || j < 0 {
        return false;
    }

    if i as usize >= n || j as usize >= m {
        return false;
    }

    return true;
}

fn print_mat(mat: &Vec<Vec<char>>) {
    for i in 0..mat.len() {
        for j in 0..mat[0].len() {
            print!("{} ", mat[i][j]);
        }
        println!("");
    }
}
