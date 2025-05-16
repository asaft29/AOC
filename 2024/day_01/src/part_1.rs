
use std::{collections::BinaryHeap, error::Error, fs};
pub struct IDs
{
    left : BinaryHeap<i32>,
    right : BinaryHeap<i32>,
}

impl IDs {
    pub fn new() -> Self
    {
        Self { left : BinaryHeap::new(), right : BinaryHeap::new() }
    }
}

pub fn read_input(file_name : &str) -> Result<String, Box<dyn Error>>
{
    let input : String = fs::read_to_string(file_name)?;
    Ok(input)
}
pub fn find_dist(file_name : &str, values : &mut IDs) -> Result<i32, Box<dyn Error>>
{
    let mut dist : i32 = 0;
    let inputs = read_input(file_name)?;
    
    for line in inputs.lines()
    {
        let mut words = line.split_whitespace();
        let first : Option<i32> = words.next().and_then(|word| word.parse().ok());
        let second : Option<i32> = words.next().and_then(|word| word.parse().ok());

        if let (Some(first_id), Some(second_id)) = (first, second)
        {
            values.left.push(first_id);
            values.right.push(second_id);
        }
    }
    while let (Some(left), Some(right)) = (values.left.pop(), values.right.pop())
    {
        dist += (left - right).abs(); 
    }

    Ok(dist)
}

pub fn execute() -> Result<(), Box<dyn Error>>
{
    let mut results = IDs::new();
    let dist = find_dist("input.txt", &mut results)?;
    println!("Part 1 - {dist}");
    Ok(()) 
}


