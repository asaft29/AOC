use regex::Regex;
use std::{error::Error, fs::read_to_string};

fn do_function(sentence: &str, valid: &mut bool, regex: &Regex) -> Result<u64, Box<dyn Error>> {
    let mut temp_res: u64 = 0;
    if *valid == true {
        for pair in regex.captures_iter(sentence) {
            let first: u64 = pair[1].parse::<u64>()?;
            let second: u64 = pair[2].parse::<u64>()?;
            temp_res += first * second;
        }
    } else {
        *valid = true;
    }
    Ok(temp_res)
}

fn find_numbers_instructions(path: &str) -> Result<u64, Box<dyn Error>> {
    let mut result: u64 = 0;

    let regex = Regex::new(r"mul\((\d+),(\d+)\)")?;
    let do_dont_regex = Regex::new(r"\bdo(?:n't)?\(\)")?;

    let inputs = read_to_string(path)?;
    let mut start = 0;
    let mut calc = true;

    for word in do_dont_regex.find_iter(&inputs) {
        let sentence = &inputs[start..word.start()];
        let keyword = word.as_str();
        let partial_res = do_function(sentence, &mut calc, &regex)?;
        if keyword == "don't()" {
            calc = false;
        }
        result += partial_res;
        start = word.end();
    }

    if start < inputs.len() {
        let sentence = &inputs[start..];
        let partial_res = do_function(sentence, &mut calc, &regex)?;
        result += partial_res;
    }

    Ok(result)
}

pub fn execute() -> Result<(), Box<dyn Error>> {
    let res = find_numbers_instructions("day_03/src/input.txt")?;
    println!("Part 2 - {res}");
    Ok(())
}
