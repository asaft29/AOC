use crate::p1::{Bridge, Op};
use anyhow::Result;
use std::str::FromStr;

// I used my ideea with string concat, but this is not the fastest choice
fn apply(op: Op, a: u64, b: u64) -> Option<u64> {
    match op {
        Op::Add => a.checked_add(b),
        Op::Mul => a.checked_mul(b),
        Op::Concat => match format!("{}{}", a, b).parse::<u64>() {
            Ok(res) => Some(res),
            _ => None,
        },
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
            if let Some(val) = apply(op, acc, values[i + 1]) {
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
    println!("Part 2 - {res}");
    Ok(())
}
