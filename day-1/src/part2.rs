pub fn count_higher_measure(measurements: &[i32], window: usize) -> Result<i32, &'static str> {
    println!("part 2");
    
    let measure_window: Vec<i32> = measurements
        .windows(window)
        .map(|x| x.iter().sum())
        .collect();

    let count: i32 = measure_window[..]
        .windows(2)
        .map(|x| if x[1] > x[0] {1} else {0})
        .sum();

    return Ok(count)
    
    
}