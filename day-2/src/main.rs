mod part1;
mod part2;

fn main() {
    let input: Vec<&str> = include_str!("../input.txt").lines().collect();
    println!("{}", part1::calculate_depth_position(&input).unwrap());
    println!("{}", part2::calculate_depth_position(&input).unwrap())
}




