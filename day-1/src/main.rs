
mod part1;
mod part2;


fn main() {
    let input: Vec<i32> = include_str!("../input.txt").lines().flat_map(|x| x.trim().parse::<i32>()).collect();

    println!("Result {}", part1::count_higher_measure(&input).unwrap());
    println!("Result {}", part2::count_higher_measure(&input, 3).unwrap());

}