fn main() {
    let mut arr = [1, 2, 0, 4, 3, 0, 5, 0];
    remove_zeros(&mut arr);
    println!("{:?}", arr); // prints [1, 2, 4, 3, 5, 0, 0, 0]

    let mut arr = [1, 2, 0, 0, 0, 3, 6];
    remove_zeros(&mut arr);
    println!("{:?}", arr); // prints [1, 2, 3, 6, 0, 0, 0]
}


fn remove_zeros(arr: &mut [i32]) -> String {
    // Keep track of the number of zeros encountered
    let mut num_zeros = 0;
    for i in 0..arr.len() {
        if arr[i] == 0 {
            num_zeros += 1;
        } else {
            arr[i - num_zeros] = arr[i];
        }
    }
    // Fill the remaining elements with zeros
    for i in (arr.len() - num_zeros)..arr.len() {
        arr[i] = 0;
    }
    arr.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_zero() {
    let mut input = [1, 2, 0, 4, 3, 0, 5, 0];
    let expected_output = "1, 2, 4, 3, 5, 0, 0, 0";
    assert_eq!(remove_zeros(&mut input), expected_output);

    let mut input = [1, 2, 0, 0, 0, 3, 6];
    let expected_output = "1, 2, 3, 6, 0, 0, 0";
    assert_eq!(remove_zeros(&mut input), expected_output);
    }
}