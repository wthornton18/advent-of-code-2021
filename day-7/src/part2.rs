use std::collections::HashMap;

pub fn parse_input(lines: &[String]) -> HashMap<i32, i32> {
    let v = lines[0].split(",").map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    let mut crabs: HashMap<i32, i32> = HashMap::new();
    for s in &v {
        *crabs.entry(*s).or_default() += 1;
    }
    crabs
}

pub fn run(lines: &[String]) -> i32 {
    let crabs = parse_input(lines);
    let mut largest = 0;
    for v in crabs.keys() {
        if *v > largest {
            largest = *v;
        }
    }
    let mut least_fuel = cost(&crabs, 0);
    for i in 0..largest {
        let current_fuel = cost(&crabs, i);
        if current_fuel < least_fuel {
            least_fuel = current_fuel;
        }
    }
    least_fuel
}

fn cost(crabs: &HashMap<i32, i32>, pos: i32) -> i32 {
    crabs.iter().map(|(hpos, num)| (i32::abs(pos - hpos), num)).map(|(dist, num)| num * (dist * (dist + 1)/2)).sum()
}