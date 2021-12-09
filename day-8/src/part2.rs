use std::collections::HashMap;

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

pub fn calculate_output_value(input: &[String]) -> i32 {
    let (input_strings, output_strings) = parse_input(input);
    let mut running_total = 0;
    for (input_str, output_str) in input_strings.iter().zip(output_strings) {
        let key_map: HashMap<char, char> = decode_input_string(&input_str);
        let number = convert_output_string(&key_map, &output_str);
        running_total += number
    }
    running_total
}


fn decode_input_string(input_str: &[String]) -> HashMap<char, char> {
    let mut key_map: HashMap<char, char> = HashMap::new();
    let (frequency_key_map, unused_chars) = frequency_analysis(&input_str);
    for (k, v) in frequency_key_map.iter() {
        key_map.insert(*k, *v);
    }
    let known_key_map = known_numbers(input_str, &unused_chars);
    for (k, v) in known_key_map.iter() {
        key_map.insert(*k, *v);
    }

    key_map
}

pub fn known_numbers(input_str: &[String], unused_char: &HashMap<char, i32>) -> HashMap<char, char> {
    let mut key_map: HashMap<char, char> = HashMap::new();
    let mut one: &String = &"".to_string();
    let mut seven: &String = &"".to_string();
    let mut four: &String = &"".to_string();

    for input in input_str {
        match input.len() {
            2 => one = input,
            3 => seven = input,
            4 => four = input,
            _ => continue
        }
    }
    let mut used: Vec<char> = Vec::new();
    for c in one.chars() {
        if unused_char.contains_key(&c) {
            key_map.insert(c, 'c');
            used.push(c)
        }
    }
    for c in seven.chars() {
        if unused_char.contains_key(&c) && ! used.contains(&c) {
            key_map.insert(c, 'a');
            used.push(c);
        }
    }
    for c in four.chars() {
        if unused_char.contains_key(&c) && ! used.contains(&c) {
            key_map.insert(c, 'd');
            used.push(c);
        }
    }

    for (k, _) in unused_char.iter() {
        if ! used.contains(&k) {
            key_map.insert(*k, 'g');
        }
    }
    key_map
}

pub fn frequency_analysis(input_str: &[String]) -> (HashMap<char, char>, HashMap<char, i32>) {
    let mut key_map: HashMap<char, char> = HashMap::new();
    let joined = input_str.join("");
    let mut letter_counts: HashMap<char, i32> = HashMap::new();
    for c in joined.chars() {
        *letter_counts.entry(c).or_insert(0) += 1;
    }
    let mut unused_chars: HashMap<char, i32> = HashMap::new();
    for (c, count) in letter_counts.iter() {
        match *count {
            4 => {key_map.insert(*c, 'e');},
            6 => {key_map.insert(*c, 'b');},
            9 => {key_map.insert(*c, 'f');},
            _ => {unused_chars.insert(*c, *count);},
        }   
    }
    (key_map, unused_chars)
}

fn convert_output_string(key_map: &HashMap<char, char>, output_str: &[String]) -> i32 {
    let correct_digits_string = vec![
        "abcefg".to_string(), //0
        "cf".to_string(), //1
        "acdeg".to_string(), //2
        "acdfg".to_string(), //3
        "bcdf".to_string(), //4
        "abdfg".to_string(), //5
        "abdefg".to_string(), //6
        "acf".to_string(), //7
        "abcdefg".to_string(), // 8
        "abcdfg".to_string(), // 9
    ];

    let mut correct_digits: HashMap<String, i32> = HashMap::new();
    for (i, digit) in correct_digits_string.iter().enumerate() {
        correct_digits.insert(digit.clone(), i as i32);
    }

    let mut digits: Vec<i32> = Vec::new();
    for output in output_str.iter() {
        let mut s: Vec<char> = Vec::new();
        for c in output.chars() {
            let actual_char = key_map.get(&c).unwrap();
            s.push(*actual_char)
        }
        s.sort_by(|a, b| a.cmp(b));
        let string_to_map: String = s.iter().collect();
        let digit = correct_digits.get(&string_to_map).unwrap();
        digits.push(*digit);
    }
    digits.iter().rev().enumerate().map(|(i, x)| i32::pow(10, i as u32)  * x).sum::<i32>()
}