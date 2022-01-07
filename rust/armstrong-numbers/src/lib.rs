pub fn is_armstrong_number(num: u32) -> bool {
    let digit_count = num_of_digits(num);

    let mut sum = 0;
    for val in num.to_string().chars() {
        sum += val.to_digit(10).unwrap().pow(digit_count);
    }

    // sum == num
    if sum == num {
        true
    } else {
        false
    }
}

fn num_of_digits(num: u32) -> u32 {
    num.to_string().len() as u32
}
