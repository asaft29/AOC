use regex::Regex;
use std::{error::Error, fs::read_to_string};

fn find_numbers(path: &str) -> Result<u64, Box<dyn Error>> {
    let mut result: u64 = 0;
    let regex = Regex::new(r"mul\((\d+),(\d+)\)")?;
    let inputs = read_to_string(path)?;
    for pair in regex.captures_iter(&inputs) {
        let first = pair[1].parse::<u64>()?;
        let second = pair[2].parse::<u64>()?;
        result += first * second;
    }
    Ok(result)
}

pub fn execute() -> Result<(), Box<dyn Error>> {
    if let Some(valid) = aoc::absoulte_path("day_03.txt") {
        let res = find_numbers(&valid)?;
        println!("Part 1 - {res}");
    }
    Ok(())
}
