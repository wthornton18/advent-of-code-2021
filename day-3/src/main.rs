mod part1;
mod part2;

fn main() {
    let input: Vec<&str> = include_str!("../input.txt").lines().collect();
    println!("{}", part1::calculate_gamma_epsilon(&input).unwrap());
    println!("{}", part2::calculate_oxyen_co2(&input).unwrap());
}
