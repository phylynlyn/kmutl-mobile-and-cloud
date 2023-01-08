fn main() {
    for i in 0..20 {
    println!("fibonacci from recursive: {} :: fibonacci from iterative: {}",finonacci_recursive(i), finonacci_iterative(i));
    }
}

fn finonacci_recursive(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => finonacci_recursive(n - 1) + finonacci_recursive(n - 2),
    }
}

fn finonacci_iterative(n: u32) -> u32 {

    if n < 2 { return n;}
    let mut first = 0;
    let mut second = 1;
    
    for _i in 0..n-1 {
        let temp  = second; //   temp = 1
        second = first + second;  //second = 0 + 1
        first = temp;  //first = 1
    }
    second // 1
}

// 5 -> 3 + 2 (5)
// 3 -> 2 + 1 (4)
// 2 -> 1 + 1 (3)
// 1 -> 1 + 0 (2)
// 1
// 0

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finonacci_recursive() {
        assert_eq!(finonacci_recursive(0), 0);
        assert_eq!(finonacci_recursive(1), 1);
        assert_eq!(finonacci_recursive(2), 1);
        assert_eq!(finonacci_recursive(3), 2);
        assert_eq!(finonacci_recursive(4), 3);
        assert_eq!(finonacci_recursive(5), 5);
        assert_eq!(finonacci_recursive(6), 8);
    }

    #[test]
    fn test_finonacci_iterative() {
        assert_eq!(finonacci_iterative(0), 0);
        assert_eq!(finonacci_iterative(1), 1);
        assert_eq!(finonacci_iterative(2), 1);
        assert_eq!(finonacci_iterative(3), 2);
        assert_eq!(finonacci_iterative(4), 3);
        assert_eq!(finonacci_iterative(5), 5);
        assert_eq!(finonacci_iterative(6), 8);
    }
}