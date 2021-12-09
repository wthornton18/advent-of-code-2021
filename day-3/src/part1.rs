
pub fn calculate_gamma_epsilon(bits: &[&str]) -> Result<i32, &'static str> {
    let bitwidth = bits[0].len();
    let mut count = vec![0; bitwidth];
    let bit_size = bits.len() /2;
    for bit in bits.iter().map(|s| s.chars()) {
        for (i, _) in bit.enumerate().filter(|(_, val)| *val == '1') {
            count[i] += 1
        }
    }

    let mut gamma = 0;
    for value in &count {
        gamma <<= 1;
        gamma |= (*value > bit_size) as i32;
    }  
    let epsilon = ((1 << bitwidth) - 1) ^ gamma as i32;
    Ok(epsilon * gamma)
    
}