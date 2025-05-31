#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_input() {
        if let Some(valid) = aoc::absoulte_path("day_01.txt") {
            let result = aoc::read_input_numbers::<i32>(valid);
            match result {
                Ok(vec) => {
                    assert_eq!(vec[0], vec![88276, 66757]);
                    assert_eq!(vec[1], vec![66898, 13714]);
                }
                Err(err) => panic!("read_input failed: {:?}", err),
            }
        } else {
            panic!("Failed to resolve input path.");
        }
    }
}
