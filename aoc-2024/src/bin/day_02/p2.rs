use std::{error::Error, fs::read_to_string};

fn is_ascending(vec: &[i32]) -> bool {
    vec.windows(2).all(|w| {
        let diff = w[1] - w[0];
        diff >= 1 && diff <= 3
    })
}

fn is_descending(vec: &[i32]) -> bool {
    vec.windows(2).all(|w| {
        let diff = w[0] - w[1];
        diff >= 1 && diff <= 3
    })
}

fn is_safe(vec: &[i32]) -> bool {
    if vec.len() < 2 {
        return true;
    }
    if vec[0] < vec[1] {
        is_ascending(vec)
    } else if vec[0] > vec[1] {
        is_descending(vec)
    } else {
        false
    }
}

fn count_reports(file: &str) -> Result<i32, Box<dyn Error>> {
    let mut safe = 0;
    let inputs = read_to_string(file)?;

    for line in inputs.lines() {
        let vec: Vec<i32> = line
            .split_whitespace()
            .filter_map(|v| v.parse().ok())
            .collect();

        if is_safe(&vec) {
            safe += 1;
            continue;
        }

        for i in 0..vec.len() {
            let mut modified = vec.clone();
            modified.remove(i);
            if is_safe(&modified) {
                safe += 1;
                break;
            }
        }
    }
    Ok(safe)
}

pub fn execute() -> Result<(), Box<dyn Error>> {
    let reports = count_reports("src/inputs/day_02.txt")?;
    println!("Part 2 - {reports}");
    Ok(())
}
