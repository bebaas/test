// use shift to implement division 
fn u64_divide(dividend: u64, divisor: u64) -> u64 {
    let mut quotient = 0;
    let mut remainder = 0;

    for i in (0..64).rev() {
        remainder = (remainder << 1) | ((dividend >> i) & 1);

        if remainder >= divisor {
            remainder -= divisor;
            quotient |= 1 << i;
        }
    }

    quotient
}

fn main() {
    let dividend = 1234567890;
    let divisor = 987654321;

    let quotient = u64_divide(dividend, divisor);
    println!("Quotient: {}", quotient);
}
