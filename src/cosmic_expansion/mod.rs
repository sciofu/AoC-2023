use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::vec;

pub fn solve_1() -> io::Result<u64> {
    // Open the file

    let file = File::open("src/cosmic_expansion/input")?;
    // Create a buffered reader to read lines efficiently
    let reader = io::BufReader::new(file);
    let lines = reader.lines();

    let mut res: u64 = 0;
    let mut galaxies_map: Vec<Vec<char>> = Vec::new();

    let mut i = 0;
    let mut start_i = 0;
    let mut start_j = 0;

    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    let mut column_galaxies: HashSet<usize> = HashSet::new();
    let mut row_galaxies: HashSet<usize> = HashSet::new();
    let mut galaxies_count: usize = 0;
    let mut g_map: HashMap<(usize, usize), usize> = HashMap::new();

    for line in lines {
        match line {
            Ok(content) => {
                let row = content.chars().collect::<Vec<char>>();

                for j in 0..row.len() {
                    if row[j] == '#' {
                        column_galaxies.insert(j);
                        row_galaxies.insert(i);
                        g_map.insert((i, j), galaxies_count);
                        galaxies_count += 1;
                    }
                }
                galaxies_map.push(row);
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
        i += 1;
    }

    // first idea used for part I -> actually adding the rows

    // println!("{:?}", column_galaxies);
    // println!("{:?}", row_galaxies);

    // print_mat(&galaxies_map);
    // let mut added = 0;
    // let n = galaxies_map.len();
    // i = 0;

    // while i < n + added {
    //     if !row_galaxies.contains(&i) {
    //         added += 1;

    //         let new_row_galaxies = row_galaxies
    //             .iter()
    //             .map(|&x| if x > i { x + 1 } else { x })
    //             .collect::<Vec<_>>();
    //         row_galaxies = new_row_galaxies.into_iter().collect::<HashSet<_>>();

    //         println!("insert row {}", i);
    //         insert_row(&mut galaxies_map, i + 1);
    //         i += 1;
    //     }
    //     i += 1;
    // }

    // let m = galaxies_map[0].len();
    // added = 0;
    // let mut j = 0;

    // while j < m + added {
    //     if !column_galaxies.contains(&j) {
    //         added += 1;

    //         let new_column_galaxies = column_galaxies
    //             .iter()
    //             .map(|&x| if x > j { x + 1 } else { x })
    //             .collect::<Vec<_>>();
    //         column_galaxies = new_column_galaxies.into_iter().collect::<HashSet<_>>();

    //         println!("insert column {}", j);
    //         insert_column(&mut galaxies_map, j + 1);
    //         j += 1;
    //     }
    //     j += 1;
    // }
    for i in 0..galaxies_map.len() {
        for j in 0..galaxies_map[0].len() {
            if galaxies_map[i][j] == '#' {
                galaxies.push((i, j));
            }
        }
    }

    // print_mat(&galaxies_map);

    let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut distances: Vec<Vec<u64>> = vec![vec![0; galaxies_map[0].len()]; galaxies_map.len()];
    let mut res: u64 = 0;

    for galaxy in galaxies {
        let mut visited: Vec<Vec<bool>> =
            vec![vec![false; galaxies_map[0].len()]; galaxies_map.len()];
        distances = vec![vec![0; galaxies_map[0].len()]; galaxies_map.len()];

        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        queue.push_back(galaxy);
        visited[galaxy.0][galaxy.1] = true;

        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();

            for direction in directions.iter() {
                let new_i = current.0 as i32 + direction.0;
                let new_j = current.1 as i32 + direction.1;

                if is_on_map(new_i, new_j, galaxies_map.len(), galaxies_map[0].len()) {
                    if !visited[new_i as usize][new_j as usize] {
                        visited[new_i as usize][new_j as usize] = true;
                        queue.push_back((new_i as usize, new_j as usize));
                        distances[new_i as usize][new_j as usize] =
                            distances[current.0][current.1] + 1;

                        if !row_galaxies.contains(&(new_i as usize)) {
                            // just expand on certain rows
                            distances[new_i as usize][new_j as usize] += 999_999;
                        }

                        if !column_galaxies.contains(&(new_j as usize)) {
                            // just expand on certain rows
                            distances[new_i as usize][new_j as usize] += 999_999;
                        }

                        if galaxies_map[new_i as usize][new_j as usize] == '#' {
                            res += distances[new_i as usize][new_j as usize];
                        }
                    }
                }
            }
        }
    }

    Ok(res / 2)
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
            print!("{}", mat[i][j]);
        }
        println!("");
    }
}

fn print_mat_u(mat: &Vec<Vec<u64>>) {
    for i in 0..mat.len() {
        for j in 0..mat[0].len() {
            print!("{} ", mat[i][j]);
        }
        println!("");
    }
}

fn insert_row(galaxies_map: &mut Vec<Vec<char>>, row: usize) {
    let new_row = vec!['.'; galaxies_map[0].len()];
    galaxies_map.insert(row, new_row);
}

fn insert_column(galaxies_map: &mut Vec<Vec<char>>, column: usize) {
    for i in 0..galaxies_map.len() {
        galaxies_map[i].insert(column, '.');
    }
}
