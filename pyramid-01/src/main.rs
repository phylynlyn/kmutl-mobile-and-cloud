fn main() {
    println!("{}",triangle(5));
}

fn triangle(height: usize) -> String {
    let mut output = String::new();
    for row in 0..height {
        for _column in 0..row +1 {
            output.push_str("* ");
        }
        output.push_str("\n");
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle() {
        // Set the expected output for the triangle
        let expected_output = "* \n* * \n* * * \n* * * * \n* * * * * \n";
        assert_eq!(triangle(5), expected_output);
    }
}