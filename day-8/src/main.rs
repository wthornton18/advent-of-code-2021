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
    println!("{}", part1::count_unique_digit(&lines));
    println!("{}", part2::calculate_output_value(&lines));
}

#[cfg(test)]

mod tests {
    use super::*;
    use std::collections::HashMap;
    #[test]
    fn test_parse() {
        let test_input = vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string(),
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc".to_string(),
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".to_string(),
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb".to_string(),
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea".to_string(),
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb".to_string(),
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe".to_string(),
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef".to_string(),
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb".to_string(),
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".to_string(),
        ];

        let parsed_output = part1::parse_input(&test_input);
        assert_eq!(parsed_output.0, vec![vec!["be","cfbegad","cbdgef","fgaecd","cgeb","fdcge","agebfd","fecdb","fabcd","edb"].iter().map(|x| x.to_string()).collect::<Vec<String>>(),
                                            vec!["edbfga","begcd","cbg","gc","gcadebf","fbgde","acbgfd","abcde","gfcbed","gfec"].iter().map(|x| x.to_string()).collect::<Vec<String>>(),
                                            vec!["fgaebd","cg","bdaec","gdafb","agbcfd","gdcbef","bgcad","gfac","gcb","cdgabef"].iter().map(|x| x.to_string()).collect::<Vec<String>>(),
                                            vec!["fbegcd","cbd","adcefb","dageb","afcb","bc","aefdc","ecdab","fgdeca","fcdbega"].iter().map(|x| x.to_string()).collect::<Vec<String>>(),
                                            vec!["aecbfdg","fbg","gf","bafeg","dbefa","fcge","gcbea","fcaegb","dgceab","fcbdga"].iter().map(|x| x.to_string()).collect::<Vec<String>>(),
                                            vec!["fgeab", "ca", "afcebg", "bdacfeg", "cfaedg", "gcfdb", "baec", "bfadeg", "bafgc", "acf"].iter().map(|x| x.to_string()).collect::<Vec<String>>(),
                                            vec!["dbcfg","fgd" ,"bdegcaf" ,"fgec" ,"aegbdf", "ecdfab", "fbedc", "dacgb","gdcebf", "gf"].iter().map(|x| x.to_string()).collect::<Vec<String>>(),
                                            vec!["bdfegc", "cbegaf", "gecbf", "dfcage", "bdacg", "ed" ,"bedf", "ced", "adcbefg", "gebcd"].iter().map(|x| x.to_string()).collect::<Vec<String>>(),
                                            vec!["egadfb", "cdbfeg", "cegd", "fecab", "cgb", "gbdefca", "cg", "fgcdab", "egfdb", "bfceg"].iter().map(|x| x.to_string()).collect::<Vec<String>>(),
                                            vec!["gcafb", "gcf", "dcaebfg", "ecagb", "gf", "abcdeg", "gaef", "cafbge", "fdbac", "fegbdc"].iter().map(|x| x.to_string()).collect::<Vec<String>>()]);
        assert_eq!(parsed_output.1, vec![vec!["fdgacbe".to_string(),"cefdb".to_string(),"cefbgd".to_string(),"gcbe".to_string()],
                                         vec!["fcgedb".to_string(),"cgb".to_string(),"dgebacf".to_string(),"gc".to_string()],
                                         vec!["cg".to_string(), "cg".to_string(), "fdcagb".to_string(), "cbg".to_string()],
                                         vec!["efabcd".to_string(), "cedba".to_string(), "gadfec".to_string(), "cb".to_string()],
                                         vec!["gecf".to_string(), "egdcabf".to_string(), "bgf".to_string(), "bfgea".to_string()],
                                         vec!["gebdcfa".to_string(), "ecba".to_string(), "ca".to_string(), "fadegcb".to_string()],
                                         vec!["cefg".to_string(), "dcbef".to_string(), "fcge".to_string(), "gbcadfe".to_string()],
                                         vec!["ed".to_string(), "bcgafe".to_string(), "cdgba".to_string(), "cbgef".to_string()],
                                         vec!["gbdfcae".to_string(), "bgc".to_string(), "cg".to_string(), "cgb".to_string()],
                                         vec!["fgae".to_string(), "cfgab".to_string(), "fg".to_string(), "bagce".to_string()]
                                         ])
        }
       #[test]
       fn test_part1() {
        let test_input = vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string(),
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc".to_string(),
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".to_string(),
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb".to_string(),
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea".to_string(),
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb".to_string(),
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe".to_string(),
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef".to_string(),
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb".to_string(),
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".to_string(),
        ];
        assert_eq!(part1::count_unique_digit(&test_input), 26)
       }

       #[test]
       fn test_frequency_analysis() {
        let test_input = vec![
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf".to_string(),
        ];
        let (input_str, _) = part2::parse_input(&test_input);
        let (key_map, unused_char) = part2::frequency_analysis(&input_str[0]);
        let mut expected_key_map: HashMap<char, char> = HashMap::new();
        let mut expected_unused_char: HashMap<char, i32> = HashMap::new();
        for (k, v) in vec!['e', 'b', 'g'].into_iter().zip(vec!['b', 'f', 'e']) {
            expected_key_map.insert(k, v);
        }
        for (k, v) in vec!['d', 'a', 'c', 'f'].into_iter().zip(vec![8, 8, 7, 7]) {
            expected_unused_char.insert(k, v);
        }
        assert_eq!(key_map, expected_key_map);
        assert_eq!(unused_char, expected_unused_char);
       }

       #[test]
       fn run_test() {
        let test_input = vec![
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf".to_string(),
        ];
        let (input_str, _) = part2::parse_input(&test_input);
        let (_, unused_char) = part2::frequency_analysis(&input_str[0]);
        let known_key_map = part2::known_numbers(&input_str[0], &unused_char);
        let mut expected_key_map: HashMap<char, char> = HashMap::new();
        for (k, v) in vec!['a', 'c', 'f', 'd'].into_iter().zip(vec!['c', 'g', 'd', 'a']) {
            expected_key_map.insert(k, v);
        } 
        assert_eq!(known_key_map, expected_key_map);
       }

       #[test]
       fn test_part2_simple() {let test_input = vec![
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf".to_string(),
       ];
       let number = part2::calculate_output_value(&test_input);
       assert_eq!(number, 5353);
       }

       #[test]
       fn test_part2() {
        let test_input = vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string(),
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc".to_string(),
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".to_string(),
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb".to_string(),
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea".to_string(),
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb".to_string(),
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe".to_string(),
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef".to_string(),
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb".to_string(),
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".to_string(),
        ];
        assert_eq!(part2::calculate_output_value(&test_input), 61229);
       }
    }
