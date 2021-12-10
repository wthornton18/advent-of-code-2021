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
    println!("{}", part1::calculate_corrupted_score(&lines));
    println!("{}", part2::calculate_autocomplete_score(&lines))
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn test_parse() {
        let test_input = vec!["[({(<(())[]>[[{[]{<()<>>".to_string(),
                         "[(()[<>])]({[<{<<[]>>(".to_string(),
                         "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
                         "(((({<>}<{<{<>}{[]{[]{}".to_string(),
                         "[[<[([]))<([[{}[[()]]]".to_string(),
                         "[{[{({}]{}}([{[{{{}}([]".to_string(),
                         "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
                         "[<(<(<(<{}))><([]([]()".to_string(),
                         "<{([([[(<>()){}]>(<<{{".to_string(),
                         "<{([{{}}[<[[[<>{}]]]>[]]".to_string(),];
        assert_eq!(part1::calculate_corrupted_score(&test_input), 26397)
    }

    #[test]
    fn test_part2() {
        let test_input = vec!["[({(<(())[]>[[{[]{<()<>>".to_string(),
                         "[(()[<>])]({[<{<<[]>>(".to_string(),
                         "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
                         "(((({<>}<{<{<>}{[]{[]{}".to_string(),
                         "[[<[([]))<([[{}[[()]]]".to_string(),
                         "[{[{({}]{}}([{[{{{}}([]".to_string(),
                         "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
                         "[<(<(<(<{}))><([]([]()".to_string(),
                         "<{([([[(<>()){}]>(<<{{".to_string(),
                         "<{([{{}}[<[[[<>{}]]]>[]]".to_string(),];
        assert_eq!(part2::calculate_autocomplete_score(&test_input), 288957)
    }
}