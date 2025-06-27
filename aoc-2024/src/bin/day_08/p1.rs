use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::{
    collections::{BTreeSet, HashMap},
    str::FromStr,
};

#[derive(Debug)]
pub struct Freq {
    grid: Vec<Vec<char>>,
    pub frequency_map: HashMap<char, Vec<(isize, isize)>>,
}

impl FromStr for Freq {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match aoc::absoulte_path(s) {
            Some(valid) => {
                let input = aoc::read_input_chars(valid)?;
                let mut hm: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
                for i in 0..input.len() {
                    for j in 0..input[0].len() {
                        if input[i][j].is_alphanumeric() {
                            hm.entry(input[i][j])
                                .or_insert_with(Vec::new)
                                .push((i as isize, j as isize));
                        }
                    }
                }
                Ok(Freq {
                    grid: input,
                    frequency_map: hm,
                })
            }
            None => Err(anyhow!("The file path is not correct!")),
        }
    }
}

impl Freq {
    pub fn check_dif(&self, i: isize, j: isize) -> bool {
        i >= 0 && i < self.grid.len() as isize && j >= 0 && j < self.grid[0].len() as isize
    }

    fn unique_antenna_spot(&self) -> usize {
        let mut unique_pos = BTreeSet::new();

        for coords in self.frequency_map.values() {
            for pair in coords.iter().combinations(2) {
                let (r1, c1) = pair[0];
                let (r2, c2) = pair[1];

                let dif_i = r2 - r1;
                let dif_j = c2 - c1;

                for (new_r, new_c) in [(r2 + dif_i, c2 + dif_j), (r1 - dif_i, c1 - dif_j)] {
                    if self.check_dif(new_r, new_c) {
                        unique_pos.insert((new_r, new_c));
                    }
                }
            }
        }

        unique_pos.len()
    }
}

fn find_antennas(path: &str) -> Result<usize> {
    let freq = Freq::from_str(path)?;
    Ok(freq.unique_antenna_spot())
}

pub fn execute() -> Result<()> {
    let antennas = find_antennas("day_08/day_08.txt")?;
    println!("Part 1 - {}", antennas);
    Ok(())
}

// made this to verify if my method worked on the simple example
#[test]
fn test_antenna() {
    if let Ok(antenna) = find_antennas("day_08/test_p1.txt") {
        assert_eq!(antenna, 14);
    }
}
