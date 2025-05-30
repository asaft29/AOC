use std::{collections::HashMap, error::Error, fs::read_to_string};

pub fn read_page_order(path: &str) -> Result<String, Box<dyn Error>> {
    let result = read_to_string(path)?
        .lines()
        .take_while(|&line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    Ok(result)
}

pub fn read_page_rules(path: &str) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let result = read_to_string(path)?
        .lines()
        .skip_while(|line| !line.trim().is_empty())
        .skip(1)
        .collect::<Vec<_>>()
        .join("\n");

    let mut vec: Vec<Vec<i32>> = Vec::new();

    for line in result.lines() {
        let temp_vec: Result<Vec<i32>, _> =
            line.split(",").map(|s| s.trim().parse::<i32>()).collect();
        vec.push(temp_vec?);
    }

    Ok(vec)
}

pub fn add_middle_page_number(path: &str) -> Result<i32, Box<dyn Error>> {
    let mut middle_number: i32 = 0;
    let mut hash_map: HashMap<i32, Vec<i32>> = HashMap::new();

    let orders = read_page_order(path)?;

    for line in orders.lines() {
        let v = line.split("|").collect::<Vec<&str>>();
        if let (Some(key), Some(value)) = (v.get(0), v.get(1)) {
            hash_map
                .entry(key.parse::<i32>()?)
                .or_insert_with(Vec::new)
                .push(value.parse::<i32>()?);
        }
    }

    for (_, vec) in hash_map.iter_mut() {
        vec.sort();
    }

    let vec_rules = read_page_rules(path)?;

    'outer: for vec in vec_rules.iter() {
        for i in 0..vec.len() - 1 {
            let current_vec = hash_map.get(&vec[i]);
            if let Some(valid) = current_vec {
                for j in (i + 1)..vec.len() {
                    if let Err(_) = valid.binary_search(&vec[j]) {
                        continue 'outer;
                    }
                }
            }
        }
        let len = vec.len();
        if len % 2 == 1 {
            middle_number += vec[len / 2];
        } else {
            let mid = len / 2;
            middle_number += (vec[mid - 1] + vec[mid]) / 2;
        }
    }

    Ok(middle_number)
}

pub fn execute() -> Result<(), Box<dyn Error>> {
    let res = add_middle_page_number("src/inputs/day_05.txt")?;
    println!("Part 1 - {}", res);
    Ok(())
}
