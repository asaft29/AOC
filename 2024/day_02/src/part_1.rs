use std::{cmp::Ordering, error::Error, fs::read_to_string};


fn check_ascending(vec : &Vec<i32>) -> bool
{
    let mut iter = vec.iter();

    if let Some(mut value) = iter.next()
    {
        for item in iter
        {
            if item - value <= 3 && item - value >= 1
            {
                value = item;
            }
            else {
                return false;
            }
        }
    }
   true
}

fn check_descending(vec : &Vec<i32>) -> bool
{
    let mut iter = vec.iter();

    if let Some(mut value) = iter.next()
    {
        for item in iter 
        {
            if value - item <= 3 && value - item >= 1
            {
                value = item;
            }
            else {
                return false;
            }
        }
    }
   true
}

fn count_reports(file : &str) -> Result<i32, Box<dyn Error>>
{
    let mut safe = 0;
    let inputs = read_to_string(file)?;
    for line in inputs.lines()
    {
        let vec = line.split_whitespace().filter_map(|val| val.parse::<i32>().ok()).collect::<Vec<i32>>();
        let mut iter = vec.iter();
        if let (Some(first), Some(second)) = (iter.next(), iter.next())
        {
            match first.cmp(&second)
            {
                Ordering::Less => { if check_ascending(&vec) { safe += 1 } },
                Ordering::Greater => { if check_descending(&vec) { safe += 1} },
                Ordering::Equal => (),
            }
        }
    }
    Ok(safe)
}

pub fn execute() -> Result<(), Box<dyn Error>>
{
    let reports = count_reports("input.txt")?;
    println!("Part 1 - {reports}");
    Ok(())
}