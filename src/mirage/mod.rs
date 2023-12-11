use std::fs::File;
use std::io::{self, BufRead};

pub fn solve_1() -> io::Result<i128> {
    // Open the file

    let file = File::open("src/mirage/input")?;
    // Create a buffered reader to read lines efficiently
    let reader = io::BufReader::new(file);
    let lines = reader.lines();

    let mut sum: i128 = 0;

    for line in lines {
        match line {
            Ok(content) => {
                let tokens = content.split(' ').collect::<Vec<&str>>();
                // println!("{:?}", tokens);
                let initial_values: Vec<i128> = content
                    .split(' ')
                    .map(|x| x.parse::<i128>().unwrap())
                    .rev() // for part 2
                    .collect();

                let res = predict_next(&initial_values);
                // println!("Result: {}", res);
                sum = res.wrapping_add(sum);
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    Ok(sum)
}

fn predict_next(initial: &Vec<i128>) -> i128 {
    let mut values_matrix: Vec<Vec<i128>> = Vec::new();
    values_matrix.push(initial.clone());

    fill_matrix(&mut values_matrix);

    return compute_prediction(&mut values_matrix);
}

fn fill_matrix(values_matrix: &mut Vec<Vec<i128>>) {
    let length = values_matrix.get(0).unwrap().len();

    for i in 1..length {
        let new_row: Vec<i128> = vec![0; length];
        values_matrix.push(new_row);
    }
}

fn compute_prediction(values_matrix: &mut Vec<Vec<i128>>) -> i128 {
    let max_iterations = values_matrix.get(0).unwrap().len();

    let mut sum = values_matrix[0][max_iterations - 1];

    for i in 1..max_iterations {
        let length = max_iterations - i;

        let mut is_final = true;
        for j in 0..length {
            values_matrix[i][j] = values_matrix[i - 1][j + 1].wrapping_sub(values_matrix[i - 1][j]);

            if is_final && values_matrix[i][j] != 0 {
                is_final = false;
            }

            if j == length - 1 {
                // println!("sum {} j {} lenght {}", sum, j, length);
                sum = sum.wrapping_add(values_matrix[i][j]);
            }
        }

        if is_final {
            break;
        }
    }

    // for i in 0..max_iterations {
    //     for j in 0..max_iterations {
    //         print!("{} ", values_matrix[i][j]);
    //     }
    //     println!()
    // }

    return sum;
}
