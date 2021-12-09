pub fn count_higher_measure(measurements: &[i32]) -> Result<i32, &'static str>{
    let count: i32 = measurements.windows(2).map(|x| if x[1] > x[0] {1} else {0}).sum();   
    return Ok(count)
}