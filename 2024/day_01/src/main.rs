use std::{collections::BinaryHeap, error::Error, fs};

pub fn read_input(file_name : &str) -> Result<String, Box<dyn Error>>
{
    let input : String = fs::read_to_string(file_name)?;
    Ok(input)
}
pub fn find_dist(file_name : &str, left_heap : &mut BinaryHeap<i32>, right_heap : &mut BinaryHeap<i32>) -> Result<i32, Box<dyn Error>>
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
            left_heap.push(first_id);
            right_heap.push(second_id);
        }
    }
    while let (Some(left), Some(right)) = (left_heap.pop(), right_heap.pop())
    {
        dist += (left - right).abs(); 
    }

    Ok(dist)
}

fn main() -> Result<(), Box<dyn Error>>{
    let mut left_heap: BinaryHeap<i32> = BinaryHeap::new();
    let mut right_heap : BinaryHeap<i32> = BinaryHeap::new();
    let dist = find_dist("input.txt", &mut left_heap, &mut right_heap)?;
    println!("Result of puzzle - {dist}");
    Ok(()) 
}
