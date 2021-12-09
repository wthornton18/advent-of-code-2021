mod part1;
mod part2;

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

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
    println!("{}", part1::run(&lines));
    println!("{}", part2::run(&lines));
}
#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let test_input = vec!["16,1,2,0,4,2,7,1,2,14".to_string()];
        let parsed_input = part1::parse_input(&test_input);
        let mut expected_output: HashMap<i32, i32> = HashMap::new();
        expected_output.insert(14, 1);
        expected_output.insert(1 , 2);
        expected_output.insert(0 , 1);
        expected_output.insert(7 , 1);
        expected_output.insert(16, 1);
        expected_output.insert(2,  3);
        expected_output.insert(4, 1);
        assert_eq!(parsed_input, expected_output)
    }

    #[test]
    fn test_part1() {
        let test_input = vec!["16,1,2,0,4,2,7,1,2,14".to_string()];
        let least_fuel = part1::run(&test_input);
        assert_eq!(least_fuel, 37)
    }

    #[test]
    fn test_part2() {
        let test_input = vec!["16,1,2,0,4,2,7,1,2,14".to_string()];
        let least_fuel = part2::run(&test_input);
        assert_eq!(least_fuel, 168)
    }

}