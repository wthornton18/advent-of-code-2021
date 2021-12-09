mod part1;

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

fn read_to_string<P>(p: P) -> Vec<String> 
    where P: AsRef<Path>
{
    let mut buffer = String::new();
    let mut f = File::open(&p).unwrap();
    let _ = f.read_to_string(&mut buffer);

    buffer.lines().map(ToOwned::to_owned).collect()
}

fn main() {
    let lines: Vec<String> = read_to_string(PathBuf::from("./input.txt"));
    println!("{}", part1::simulate_fishes(80, &lines));
    println!("{}", part1::simulate_fishes(256, &lines))
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn test_parse() {
        let input_data = vec!["3,4,3,1,2".to_string()];
        let initial_state = part1::parse_data(&input_data);
        assert_eq!(initial_state, [0, 1, 1, 2, 1, 0, 0, 0, 0])
    }

    #[test]
    fn test_part1() {
        let input_data = vec!["3,4,3,1,2".to_string()];
        let fishes = part1::simulate_fishes(80, &input_data);
        assert_eq!(fishes, 5934)
    }
    #[test]
    fn test_part2() {
        let input_data = vec!["3,4,3,1,2".to_string()];
        let fishes = part1::simulate_fishes(256, &input_data);
        assert_eq!(fishes, 26984457539)
    }  
}