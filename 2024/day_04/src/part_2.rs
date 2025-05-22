use std::{error::Error, fs::read_to_string};

fn check_xmas(i: usize, j: usize, matrix: &Vec<Vec<char>>) -> bool {
    if i == 0 || i + 1 >= matrix.len() || j == 0 || j + 1 >= matrix[0].len() {
        return false;
    }

    let up_left = matrix[i - 1][j - 1];
    let up_right = matrix[i - 1][j + 1];
    let down_left = matrix[i + 1][j - 1];
    let down_right = matrix[i + 1][j + 1];
    let center = matrix[i][j];

    let diag1 = (up_left, center, down_right);
    let diag2 = (up_right, center, down_left);

    let diag1_valid = (diag1.0 == 'M' && diag1.1 == 'A' && diag1.2 == 'S')
        || (diag1.0 == 'S' && diag1.1 == 'A' && diag1.2 == 'M');

    let diag2_valid = (diag2.0 == 'M' && diag2.1 == 'A' && diag2.2 == 'S')
        || (diag2.0 == 'S' && diag2.1 == 'A' && diag2.2 == 'M');

    diag1_valid && diag2_valid
}

fn find_diagonal(path: &str) -> Result<u32, Box<dyn Error>> {
    let mut count: u32 = 0;
    let inputs = read_to_string(path)?;
    let matrix: Vec<Vec<char>> = inputs.lines().map(|line| line.chars().collect()).collect();

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 'A' && check_xmas(i, j, &matrix) {
                count += 1;
            }
        }
    }
    Ok(count)
}
pub fn execute() -> Result<(), Box<dyn Error>> {
    let res = find_diagonal("input.txt")?;
    println!("Part 2 - {res}");
    Ok(())
}
