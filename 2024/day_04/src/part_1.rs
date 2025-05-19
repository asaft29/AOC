use std::{error::Error, fs::read_to_string};

const WORD: [char; 4] = ['X', 'M', 'A', 'S'];

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

fn check_direction(
    start_i: usize,
    start_j: usize,
    matrix: &Vec<Vec<char>>,
    dir: (isize, isize),
) -> bool {
    let r = matrix.len() as isize;
    let c = matrix[0].len() as isize;

    for offset in 0..4 {
        let ni = start_i as isize + dir.0 * offset;
        let nj = start_j as isize + dir.1 * offset;

        if ni < 0 || nj < 0 || ni >= r || nj >= c {
            return false;
        }

        let ch = matrix[ni as usize][nj as usize];

        if ch != WORD[offset as usize] {
            return false;
        }
    }

    true
}

fn find_word(path: &str) -> Result<u32, Box<dyn Error>> {
    let mut count: u32 = 0;
    let inputs = read_to_string(path)?;
    let matrix: Vec<Vec<char>> = inputs.lines().map(|line| line.chars().collect()).collect();

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 'X' {
                for dir in DIRECTIONS.iter() {
                    if check_direction(i, j, &matrix, *dir) {
                        count += 1;
                    }
                }
            }
        }
    }

    Ok(count)
}

pub fn execute() -> Result<(), Box<dyn Error>> {
    let res = find_word("input.txt")?;
    println!("Part 1 - {res}");
    Ok(())
}
