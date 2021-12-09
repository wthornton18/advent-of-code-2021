pub fn calculate_oxyen_co2(input_bits: &Vec<&str>) -> Result<i32, &'static str> {
    let bitwidth = input_bits[0].len();
    let mut bits = input_bits.clone();
    loop {
        for i in 0..bitwidth {
            let count = bit_freq(&bits, bitwidth);
            let bit_size = bits.len();
            let mut new_bits: Vec<&str> = Vec::new();
            if count[i] >= bit_size - count[i] {
                for bit in bits.iter() {
                    if bit.chars().nth(i).unwrap() == '1' {
                        new_bits.push(bit)
                    }
                }
            } else {
                for bit in bits.iter() {
                    if bit.chars().nth(i).unwrap() == '0' {
                        new_bits.push(bit)
                    }
                }
            }
            bits = new_bits;
            if bits.len() < 2 {
                break;
            }
        }
        if bits.len() < 2 {
            break;
        }
    }

    let oxygen_generator = usize::from_str_radix(bits[0], 2).unwrap() as i32;
    let mut bits = input_bits.clone();
    

    loop {
        for i in 0..bitwidth {
            let count = bit_freq(&bits, bitwidth);
            let bit_size = bits.len();
            let mut new_bits: Vec<&str> = Vec::new();
            if count[i] < bit_size - count[i] {
                for bit in bits.iter() {
                    if bit.chars().nth(i).unwrap() == '1' {
                        new_bits.push(bit)
                    }
                }
            } else {
                for bit in bits.iter() {
                    if bit.chars().nth(i).unwrap() == '0' {
                        new_bits.push(bit)
                    }
                }
            }
            bits = new_bits;
            if bits.len() < 2 {
                break;
            }
        }
        if bits.len() < 2 {
            break;
        }
    }

    let co2_generator = usize::from_str_radix(bits[0], 2).unwrap() as i32;
    Ok(oxygen_generator * co2_generator)
}

fn bit_freq(bits: &Vec<&str>, bitwidth: usize) -> Vec<usize> {
    let mut count = vec![0;bitwidth];
    for line in bits.iter().map(|s| s.chars()) {
        for (i, _) in line.enumerate().filter(|(_, val)| *val == '1') {
            count[i] += 1;
        }
    }
    count
}