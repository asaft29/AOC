use anyhow::Result;
use std::collections::HashSet;
use std::sync::{Mutex, Arc, atomic::{AtomicU32, Ordering}};
use threadpool::ThreadPool;

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


// don't worry if u don't understand this, it is pretty complicated if u just started Rust 
// long story short, i tried to have threads for each row so that the computation is faster

pub fn find_cycles(path: impl AsRef<str>) -> Result<u32> {
    let matrix = Arc::new(aoc::read_input_chars(path.as_ref())?);

    let r = matrix.len();
    let c = matrix[0].len();

    // Find start position
    let (start_i, start_j) = {
        let mut pos = (0, 0);
        'outer: for i in 0..r {
            for j in 0..c {
                if matrix[i][j] == '^' {
                    pos = (i, j);
                    break 'outer;
                }
            }
        }
        pos
    };

    let counter = Arc::new(AtomicU32::new(0));
    let pool = ThreadPool::new(r * c);

    for i in 0..r {
        for j in 0..c {
            let counter = Arc::clone(&counter);
            let matrix_clone = Arc::clone(&matrix);
            pool.execute(move || {
                if matrix_clone[i][j] == '#' || (i, j) == (start_i, start_j) {
                    return;
                }

                let mut local_matrix = matrix_clone.to_vec();
                local_matrix[i] = matrix_clone[i].clone();

                local_matrix[i][j] = '#';

                if count_pos_cycles(start_i, start_j, &local_matrix) {
                    counter.fetch_add(1, Ordering::Relaxed);
                }
            });
        }
    }

    pool.join();

    Ok(counter.load(Ordering::Relaxed))
}



pub fn exec() -> Result<()> {
    if let Some(valid) = aoc::absoulte_path("day_06.txt") {
        println!("It takes 5-6 seconds for the output to generate with threading");
        let res = find_cycles(&valid)?;
        println!("Part 2 - {res}");
    }
    Ok(())
}
