use std::io;
use std::io::Write;

fn main() {
    print!("Enter a number: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let number: u64 = input.trim().parse().expect("Failed to parse input");

    println!("{} = {}", number ,prime_factors(number));

}

fn prime_factors(mut number: u64) -> String {
    let mut factors = Vec::new();
    while number > 1 {
        for i in 2..(number as f64).sqrt() as u64 + 1 {
            while number % i == 0 {
                factors.push(i.to_string());
                number /= i;
            }
            if number == 1 {
                break;
            }
        }
        if number > 1 {
            factors.push(number.to_string());
            break;
        }
    }
    format!("{}", factors.join("*"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_factor() {
    // Test input: 8, expected output: "2*2*2"
    let input = 8;
    let expected_output = "2*2*2";
    assert_eq!(prime_factors(input), expected_output);
    
    // Test input: 8, expected output: "2*2*3"
    let input = 12;
    let expected_output = "2*2*3";
    assert_eq!(prime_factors(input), expected_output);

    // Test input: 14, expected output: "2*7"
    let input = 14;
    let expected_output = "2*7";
    assert_eq!(prime_factors(input), expected_output);
    }
}