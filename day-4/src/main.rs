mod part1;
mod part2;

use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::Read;

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
    println!("{}", part1::calculate_winner(&lines).unwrap());
    println!("{}", part2::calculate_loser(&lines).unwrap());
}
#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let input_data = vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1".to_string(),
            "".to_string(),
            "22 13 17 11 0".to_string(),
            "8  2 23  4 24".to_string(),
            "21  9 14 16 7".to_string(),
            "6 10  3 18  5".to_string(),
            "1 12 20 15 19".to_string(),
            "".to_string(),
            "3 15  0  2 22".to_string(),
            "9 18 13 17  5".to_string(),
            "19  8 7 25 23".to_string(),
            "20 11 10 24 4".to_string(),
            "14 21 16 12 6".to_string(),
            "".to_string(),
            "14 21 17 24  4".to_string(),
            "10 16 15  9 19".to_string(),
            "18  8 23 26 20".to_string(),
            "22 11 13  6  5".to_string(),
            " 2  0 12  3  7".to_string(),
        ];
        
        let (numbers, boards) = part1::parse_boards(&input_data);
        assert_eq!(numbers, vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1]);
        assert_eq!(boards[0].get_state(), &[[22, 13, 17, 11, 0], [8,  2, 23,  4, 24], [21,  9, 14, 16, 7], [6, 10,  3, 18,  5], [1, 12, 20, 15, 19]]);
        assert_eq!(boards[1].get_state(), &[[3, 15,  0,  2, 22], [9, 18, 13, 17,  5], [19,  8, 7, 25, 23], [20, 11, 10, 24, 4], [14, 21, 16, 12, 6]])
    }

    #[test]
    fn test_part1() {
        let input_data = vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1".to_string(),
            "".to_string(),
            "22 13 17 11 0".to_string(),
            "8  2 23  4 24".to_string(),
            "21  9 14 16 7".to_string(),
            "6 10  3 18  5".to_string(),
            "1 12 20 15 19".to_string(),
            "".to_string(),
            "3 15  0  2 22".to_string(),
            "9 18 13 17  5".to_string(),
            "19  8 7 25 23".to_string(),
            "20 11 10 24 4".to_string(),
            "14 21 16 12 6".to_string(),
            "".to_string(),
            "14 21 17 24  4".to_string(),
            "10 16 15  9 19".to_string(),
            "18  8 23 26 20".to_string(),
            "22 11 13  6  5".to_string(),
            " 2  0 12  3  7".to_string(),
        ];
        let result = part1::calculate_winner(&input_data).unwrap();
        assert_eq!(result, 4512)
    

    }

    #[test]
    fn test_part2() {
        let input_data = vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1".to_string(),
            "".to_string(),
            "22 13 17 11 0".to_string(),
            "8  2 23  4 24".to_string(),
            "21  9 14 16 7".to_string(),
            "6 10  3 18  5".to_string(),
            "1 12 20 15 19".to_string(),
            "".to_string(),
            "3 15  0  2 22".to_string(),
            "9 18 13 17  5".to_string(),
            "19  8 7 25 23".to_string(),
            "20 11 10 24 4".to_string(),
            "14 21 16 12 6".to_string(),
            "".to_string(),
            "14 21 17 24  4".to_string(),
            "10 16 15  9 19".to_string(),
            "18  8 23 26 20".to_string(),
            "22 11 13  6  5".to_string(),
            " 2  0 12  3  7".to_string(),
        ];
        let result = part2::calculate_loser(&input_data).unwrap();
        assert_eq!(result, 1924)
    }
}