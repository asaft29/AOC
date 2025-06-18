use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::{fs::read_to_string, str::FromStr};
struct Bridge {
    left: Vec<u64>,
    right: Vec<Vec<u64>>,
}

enum Op {
    Add,
    Mul,
}

fn apply(op: &Op, a: u64, b: u64) -> u64 {
    match op {
        Op::Add => a + b,
        Op::Mul => a * b,
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
    if values.is_empty() {
        return false;
    }

    let ops = [Op::Add, Op::Mul];
    let n = values.len() - 1;

    for op_sequence in std::iter::repeat(ops.iter())
        .take(n)
        .multi_cartesian_product()
    {
        let mut acc = values[0];
        for (i, op) in op_sequence.into_iter().enumerate() {
            acc = apply(op, acc, values[i + 1]);
        }
        if acc == target {
            return true;
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
