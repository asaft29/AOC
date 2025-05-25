use crate::part_1;
use std::{collections::HashMap, error::Error};



// this works because input.txt is correct, if not i think Kahn's algorithm / topological sort 
// should have been implemented for this

pub fn add_wrong_middle_page_number(path: &str) -> Result<i32, Box<dyn Error>> {
    let mut middle_number: i32 = 0;
    let mut hash_map: HashMap<i32, Vec<i32>> = HashMap::new();

    let orders = part_1::read_page_order(path)?;
    for line in orders.lines() {
        let v = line.split("|").collect::<Vec<&str>>();
        if let (Some(key), Some(value)) = (v.get(0), v.get(1)) {
            hash_map
                .entry(key.parse::<i32>()?)
                .or_insert_with(Vec::new)
                .push(value.parse::<i32>()?);
        }
    }

    for vec in hash_map.values_mut() {
        vec.sort();
    }

    let mut vec_rules = part_1::read_page_rules(path)?;

    for vec in vec_rules.iter_mut() {
        let mut is_wrong = false;

        for i in 0..vec.len() - 1 {
            if let Some(valid) = hash_map.get(&vec[i]) {
                if valid.binary_search(&vec[i + 1]).is_err() {
                    is_wrong = true;
                    break;
                }
            }
        }

        if is_wrong {
            vec.sort_by(|a, b| {
                if let Some(order) = hash_map.get(a) {
                    if order.contains(b) {
                        return std::cmp::Ordering::Less;
                    }
                } else if let Some(order) = hash_map.get(b) {
                    if order.contains(a) {
                        return std::cmp::Ordering::Greater;
                    }
                }
                std::cmp::Ordering::Equal
            });

            let len = vec.len();
            let median = if len % 2 == 1 {
                vec[len / 2]
            } else {
                (vec[len / 2 - 1] + vec[len / 2]) / 2
            };
            middle_number += median;
        }
    }

    Ok(middle_number)
}

pub fn execute() -> Result<(), Box<dyn Error>> {
    let res = add_wrong_middle_page_number("day_05/src/input.txt")?;
    println!("Part 2 - {}", res);
    Ok(())
}
