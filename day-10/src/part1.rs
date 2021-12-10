pub fn calculate_corrupted_score(input: &[String]) -> i32 {
    input.iter().map(|line| score(&line)).sum()
}

fn score(line: &String) -> i32 {
    let mut expect: Vec<char> = Vec::new();
    let mut score: i32 = 0;
    for c in line.chars() {
        match c {
            '(' => expect.push(')'),
            '{' => expect.push('}'),
            '[' => expect.push(']'),
            '<' => expect.push('>'),
            _ => {
                if c != expect.pop().unwrap() {
                    match c {
                        ')' => return 3,
                        ']' => return 57,
                        '}' => return 1197,
                        '>' => return 25137,
                        _ => panic!("illegal char {}", c)
                    }
                }
            }
        }
    }
    0
}