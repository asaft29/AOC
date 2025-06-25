use anyhow::{anyhow, Result};
use std::{fs::read_to_string, str::FromStr};
pub struct Bridge {
    pub left: Vec<u64>,
    pub right: Vec<Vec<u64>>,
}

#[derive(Copy, Clone)]
pub enum Op {
    Add,
    Mul,
    Concat,
}

fn apply(op: &Op, a: u64, b: u64) -> Option<u64> {
    match op {
        Op::Add => a.checked_add(b),
        Op::Mul => a.checked_mul(b),
        _ => a.checked_add(b),
    }
}

// trying to get used to important traits
impl FromStr for Bridge {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match aoc::absoulte_path(s) {
            None => Err(anyhow!("File not found!")),
            Some(valid) => {
                let inputs = read_to_string(valid)?;
                let (l, r): (Vec<u64>, Vec<Vec<u64>>) = inputs
                    .lines()
                    .map(|line| {
                        let nums: Vec<u64> = line
                            .split(|c| c == ':' || c == ' ')
                            .filter_map(|s| s.trim().parse::<u64>().ok())
                            .collect();

                        (nums[0], nums[1..].to_vec())
                    })
                    .unzip();
                Ok(Bridge { left: l, right: r })
            }
        }
    }
}

fn is_valid(target: u64, values: &[u64]) -> bool {
    let mut stack = vec![(values[0], 0)];

    while let Some((acc, i)) = stack.pop() {
        if i == values.len() - 1 {
            if acc == target {
                return true;
            }
            continue;
        }

        for op in [Op::Add, Op::Mul, Op::Concat] {
            if let Some(val) = apply(&op, acc, values[i + 1]) {
                if val <= target {
                    stack.push((val, i + 1));
                }
            }
        }
    }
    false
}
fn get_result() -> Result<u64> {
    let mut res = 0;
    let bridge = Bridge::from_str("day_07.txt")?;
    let mut i = 0;
    let size = bridge.left.len();
    while i < size {
        if is_valid(bridge.left[i], &bridge.right[i]) {
            res += bridge.left[i];
            i += 1;
        } else {
            i += 1;
        }
    }
    Ok(res)
}

pub fn exec() -> Result<()> {
    let res = get_result()?;
    println!("Part 1 - {res}");
    Ok(())
}
