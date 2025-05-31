use anyhow::Result;
use std::collections::HashSet;

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn count_pos_cycles(mut i: usize, mut j: usize, matrix: &Vec<Vec<char>>) -> bool {
    let mut index = 0;
    let mut seen: HashSet<(usize, usize, usize)> = HashSet::new();

    while i < matrix.len() && j < matrix[0].len() {
        if !seen.insert((i, j, index)) {
            return true;
        }

        let (x, y) = DIRECTIONS[index];
        let new_x = i as isize + x;
        let new_y = j as isize + y;
        if new_x < 0
            || new_y < 0
            || new_x as usize >= matrix.len()
            || new_y as usize >= matrix[0].len()
        {
            break;
        } else {
            if matrix[new_x as usize][new_y as usize] == '#' {
                index = (index + 1) % 4;
                continue;
            }
            i = new_x as usize;
            j = new_y as usize;
        }
    }

    false
}

pub fn find_cycles(path: impl AsRef<str>) -> Result<u32> {
    let mut cycles = 0;
    let mut matrix: Vec<Vec<char>> = aoc::read_input_chars(path.as_ref())?;

    let r = matrix.len();
    let c = matrix[0].len();

    let (start_i, start_j) = {
        let mut pos = (0, 0);
        for i in 0..r {
            for j in 0..c {
                if matrix[i][j] == '^' {
                    pos = (i, j);
                    break;
                }
            }
        }
        pos
    };

    for i in 0..r {
        for j in 0..c {
            if matrix[i][j] == '#' || (i, j) == (start_i, start_j) {
                continue;
            }

            matrix[i][j] = '#';
            if count_pos_cycles(start_i, start_j, &matrix) {
                cycles += 1;
            }
            matrix[i][j] = '.';
        }
    }

    Ok(cycles)
}

pub fn exec() -> Result<()> {
    if let Some(valid) = aoc::absoulte_path("day_06.txt") {
        println!("It takes some time - will try to make it better with async...");
        let res = find_cycles(&valid)?;
        println!("Part 2 - {res}");
    }
    Ok(())
}
