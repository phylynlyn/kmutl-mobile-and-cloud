fn main() {
    println!("{}", pyramid_triangle(5));
}

fn pyramid_triangle(height: usize) -> String {
    let mut output = String::new();
    for row in 0..height {
        // Calculate the number of spaces to be printed at the beginning of the row
        let num_spaces = height - row - 1;
        // Calculate the number of stars to be printed in the row
        let num_stars = (row * 2) + 1;
        // Print the spaces
        for _ in 0..num_spaces {
            output.push_str(" ");
        }
        // Print the stars
        for _ in 0..num_stars {
            output.push_str("*");
        }
        // Print a newline at the end of the row
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
        let expected_output = "    *\n   ***\n  *****\n *******\n*********\n";
        assert_eq!(pyramid_triangle(5), expected_output);
    }
}