use std::{collections::BTreeSet, str::FromStr};
use crate::p1::Freq;
use anyhow::Result;
use itertools::Itertools;

impl Freq {
    fn get_antinodes(&self) -> usize {
        let mut unique_pos = BTreeSet::new();
        for coords in self.frequency_map.values() {
            for pair in coords.iter().combinations(2) {
                let (r1, c1) = pair[0];
                let (r2, c2) = pair[1];

                let dif_i = r2 - r1;
                let dif_j = c2 - c1;

                let directions = [(dif_i, dif_j), (-dif_i, -dif_j)];

                unique_pos.insert((*r1, *c1));
                unique_pos.insert((*r2, *c2));

                for dir in directions.iter() {
                    let mut original_state = (*r2, *c2);
                    loop {
                        original_state.0 += dir.0;
                        original_state.1 += dir.1;

                        if !self.check_dif(original_state.0, original_state.1) {
                            break;
                        }

                        unique_pos.insert((original_state.0, original_state.1));
                    }
                }
            }
        }
        unique_pos.len()
    }
}

fn find_antinodes(path: &str) -> Result<usize> {
    match Freq::from_str(path) {
        Ok(freq) => Ok(freq.get_antinodes()),
        Err(e) => return Err(e),
    }
}
pub fn execute() -> Result<()> {
    let antinodes = find_antinodes("day_08/day_08.txt")?;
    println!("Part 2 - {}", antinodes);
    Ok(())
}
#[test]
fn test_antinodes() {
    if let Ok(antinodes) = find_antinodes("day_08/test_p2.txt") {
        assert_eq!(antinodes, 9);
    }
}
