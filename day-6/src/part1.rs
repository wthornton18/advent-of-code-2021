pub fn parse_data(input: &[String]) -> Vec<u64> {
    let v = input[0].split(",").map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
    let mut result: Vec<u64> = vec![0; 9];
    for number in v {
        result[number] += 1;
    }
    result
}

pub fn simulate_fishes(days: i32, input: &[String]) -> u64 {
    let mut fishes = parse_data(&input);
    for _ in 1..days+1 {
        let new_fish = fishes[0];
        fishes[7] += new_fish;
        for i in 1..9 {
            fishes[i-1] = fishes[i];
        }
        fishes[8] = new_fish;
    }
    fishes.iter().sum()
}