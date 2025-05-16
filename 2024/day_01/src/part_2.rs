use std::{collections::HashMap, collections::HashSet, error::Error};

use crate::part_1;

pub struct Similar
{
    left : HashSet<i32>,
    right : HashMap<i32, i32>,
}

impl Similar
{
 pub fn new() -> Self
 {
    Self { left : HashSet::new(), right : HashMap::new() }
 }   
}

pub fn find_similar(file_name : &str, similar : &mut Similar) -> Result<i32, Box<dyn Error>>
{
    let mut dist = 0;
    let inputs = part_1::read_input(file_name)?;
    for line in inputs.lines()
    {
        let mut words = line.split_whitespace();
        let first = words.next().and_then(|word| word.parse::<i32>().ok());
        let second = words.next().and_then(|word| word.parse::<i32>().ok());
        if let (Some (left_val), Some(right_val)) = (first, second)
        {
            similar.left.insert(left_val);
            *similar.right.entry(right_val).or_insert(0) += 1;
        }
    }

    for id in similar.left.iter()
    {
        if !similar.right.contains_key(id)
        {
            continue;
        } 
        let count = similar.right.get(id).ok_or("Error when accessing index")?;
        dist += id * count; 
    } 
    Ok(dist)
}

pub fn execute() -> Result<(), Box<dyn Error>>
{
    let mut sim = Similar::new();
    let dist = find_similar("input.txt", &mut sim)?;
    println!("Part 2 - {dist}");
    Ok(())
}