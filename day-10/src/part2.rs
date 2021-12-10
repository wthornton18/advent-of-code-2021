pub fn calculate_autocomplete_score(input: &[String]) -> i128 {
    let mut scores = input
                        .iter()
                        .map(|line| score(line))
                        .filter(|line| match line {
                            Some(_) => true,
                            None => false
                        } ).map(|line| line.unwrap()).collect::<Vec<i128>>();
    scores.sort_by(|a , b| a.cmp(b));
    let score_len = scores.len() / 2;
    println!("{}", scores.len());
    println!("{}", scores[score_len-1]);
    scores[score_len]
}

fn score(line: &String) -> Option<i128> {
    let mut expect: Vec<char> = Vec::new();
    let mut score: i128 = 0;
    for c in line.chars() {
        match c {
            '(' => expect.push(')'),
            '{' => expect.push('}'),
            '[' => expect.push(']'),
            '<' => expect.push('>'),
            _ => {
                if c != expect.pop().unwrap() {
                    return None
                }
                }
            }
        }
    println!("{:?}", expect);
    while expect.len() > 0 {
        score *=5;
        score += match expect.pop().unwrap() {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("illegal char")
        }

    }
    Some(score)
}
