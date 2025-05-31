use anyhow::Result;

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn count_pos(
    mut i: usize,
    mut j: usize,
    matrix: &mut Vec<Vec<char>>,
    pos: &mut u32,
    visited: &mut Vec<Vec<bool>>,
) {
    let mut index = 0;
    while i < matrix.len() && j < matrix[0].len() {
        if visited[i][j] == false {
            *pos += 1;
            visited[i][j] = true;
        }
        let (x, y) = DIRECTIONS[index % 4];
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
}

pub fn find_positions(path: impl AsRef<str>) -> Result<u32> {
    let mut positions = 0;
    let mut matrix: Vec<Vec<char>> = aoc::read_input_chars(path.as_ref())?;

    let r = matrix.len();
    let c = matrix[0].len();

    let mut visited: Vec<Vec<bool>> = vec![vec![false; c]; r];

    for i in 0..r {
        for j in 0..c {
            if matrix[i][j] == '^' {
                count_pos(i, j, &mut matrix, &mut positions, &mut visited);
                break;
            }
        }
    }

    Ok(positions)
}

pub fn exec() -> Result<()> {
    if let Some(valid) = aoc::absoulte_path("day_06.txt") {
        let res = find_positions(&valid)?;
        println!("Part 1 - {res}");
    }
    Ok(())
}
