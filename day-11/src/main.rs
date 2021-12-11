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
    println!("{}", part1::Floor::new(&lines).calculate_octupus_flashing(100));
    println!("{}", part2::Floor::new(&lines).calculate_first_synchro_flash());
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = vec!["5483143223".to_string(),
                              "2745854711".to_string(),
                              "5264556173".to_string(),
                              "6141336146".to_string(),
                              "6357385478".to_string(),
                              "4167524645".to_string(),
                              "2176841721".to_string(),
                              "6882881134".to_string(),
                              "4846848554".to_string(),
                              "5283751526".to_string(),];
        assert_eq!(part1::Floor::new(&test_input).calculate_octupus_flashing(100), 1656);
    }        
    
    #[test]
    fn test_part2() {
        let test_input = vec!["5483143223".to_string(),
                              "2745854711".to_string(),
                              "5264556173".to_string(),
                              "6141336146".to_string(),
                              "6357385478".to_string(),
                              "4167524645".to_string(),
                              "2176841721".to_string(),
                              "6882881134".to_string(),
                              "4846848554".to_string(),
                              "5283751526".to_string(),];
        assert_eq!(part2::Floor::new(&test_input).calculate_first_synchro_flash(), 195);

    }
}