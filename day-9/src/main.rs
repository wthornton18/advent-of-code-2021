mod part1;
mod part2;

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};


fn read_to_string<P>(p: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let mut buffer = String::new();
    let mut f = File::open(&p).unwrap();
    let _ = f.read_to_string(&mut buffer);

    buffer.lines().map(ToOwned::to_owned).collect()
}

fn main() {
    let lines: Vec<String> = read_to_string(PathBuf::from("./input.txt"));
    println!("{}", part1::Floor::new(&lines).calculate_risk_level());
    println!("{}", part2::Floor::new(&lines).get_basin_sizes());
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = vec!["2199943210".to_string(),
                              "3987894921".to_string(),
                              "9856789892".to_string(),
                              "8767896789".to_string(),
                              "9899965678".to_string(),];
        let floor = part1::Floor::new(&test_input);
        assert_eq!(floor.calculate_risk_level(), 15);
    }

    #[test]
    fn test_part2() {
        let test_input = vec!["2199943210".to_string(),
        "3987894921".to_string(),
        "9856789892".to_string(),
        "8767896789".to_string(),
        "9899965678".to_string(),];
        let floor = part2::Floor::new(&test_input);
        assert_eq!(floor.get_basin_sizes(), 1134)
    }
}