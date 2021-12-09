pub fn parse_input(input: &[String]) -> (Vec<Vec<String>>, Vec<Vec<String>>) {
    let mut output_strings: Vec<Vec<String>> = Vec::new();
    let mut input_strings: Vec<Vec<String>> = Vec::new();
    for line in input {
        let split_str = line.split(" | ").map(|x| x.to_string()).collect::<Vec<String>>();
        let temp_input = &split_str[0];
        let temp_output = &split_str[1];
        input_strings.push(temp_input.split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>());
        output_strings.push(temp_output.split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>());
    }
    (input_strings, output_strings)
}

pub fn count_unique_digit(input: &[String]) -> i32 {
    let (_, output_strings) = parse_input(&input);
    let unique_vec = vec![2,3,4,7];
    output_strings.iter()
                    .map(|output| 
                          output
                            .iter()
                            .map( |result| if unique_vec.contains(&result.len()) {1} else {0}))
                    .flatten()
                    .sum()
    
}