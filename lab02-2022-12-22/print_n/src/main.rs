use std::io;
use std::io::Write;

fn main() {
    print!("Enter a number: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let number: usize = input.trim().parse().expect("Failed to parse input");
    println!("{}",print_n(number));
}

fn print_n(n_height: usize) -> String {
    let mut output = String::new();
    for row in 0..n_height {
        for col in 0..n_height {
            if col == 0 || col == n_height - 1 || col == row {
                output.push_str("X ");
            } else {
                output.push_str("O ");
            }
        }
        output.push_str("\n");
    }
    output
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_n() {
       // Test input: 5, expected output: "X O O O X \nX X O O X \nX O X O X \nX O O X X \nX O O O X \n"
        let expected_output = "X O O O X \nX X O O X \nX O X O X \nX O O X X \nX O O O X \n";
        assert_eq!(print_n(5), expected_output);

        // Test input: 7, expected output: "X O O O O O X \nX X O O O O X \nX O X O O O X \nX O O X O O X \nX O O O X O X \nX O O O O X X \nX O O O O O X \n"
        let expected_output = "X O O O O O X \nX X O O O O X \nX O X O O O X \nX O O X O O X \nX O O O X O X \nX O O O O X X \nX O O O O O X \n";
        assert_eq!(print_n(7), expected_output);
    }
}

