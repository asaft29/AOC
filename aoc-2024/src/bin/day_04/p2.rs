use std::{error::Error, fs::read_to_string};

fn check_xmas(i: usize, j: usize, matrix: &Vec<Vec<char>>) -> bool {
    if i == 0 || j == 0 || i + 1 >= matrix.len() || j + 1 >= matrix[0].len() {
        return false;
    }

    if let (Some(row_above), Some(row_below)) = (matrix.get(i - 1), matrix.get(i + 1)) {
        if let (Some(top_left), Some(top_right), Some(bottom_left), Some(bottom_right)) = (
            row_above.get(j - 1),
            row_above.get(j + 1),
            row_below.get(j - 1),
            row_below.get(j + 1),
        ) {
            let is_mas = |arr: [char; 2]| arr == ['M', 'S'] || arr == ['S', 'M'];

            let diag1 = [*top_left, *bottom_right];
            let diag2 = [*top_right, *bottom_left];

            return is_mas(diag1) && is_mas(diag2);
        }
    }
    false
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
    if let Some(valid) = aoc::absoulte_path("day_04.txt") {
        let res = find_diagonal(&valid)?;
        println!("Part 2 - {res}");
    }
    Ok(())
}
