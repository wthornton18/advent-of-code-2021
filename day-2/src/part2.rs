pub fn calculate_depth_position(instructions: &[&str]) -> Result<i32, &'static str>{
    let mut position_vec: Vec<i32> = vec![0, 0];
    let mut aim: i32 = 0;
    for instruction in instructions {
        let words: Vec<&str> = instruction.split(" ").collect();
        let movement = words[0];
        let amount: i32 = words[1].parse().unwrap();
        match movement {
            "forward" => {position_vec[0] += amount;
                          position_vec[1] += amount * aim},
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => continue
        }
    }
    Ok(position_vec[0] * position_vec[1])
}