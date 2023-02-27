fn get_digits(num: u32) -> Vec<u32> {
    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

pub fn is_narcissistic(num: u32) -> bool {
    let digits = get_digits(num);
    let exponential = digits.len() as u32;
    let sum = digits.iter().map(|d| d.pow(exponential)).sum();
    num == sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_get_digits() {
        assert_eq!(get_digits(407), vec![4, 0, 7]);
        assert_eq!(get_digits(999), vec![9, 9, 9]);
    }

    #[test]
    fn check_fail() {
        assert_eq!(is_narcissistic(111), false);
    }

    #[test]
    fn happy_path() {
        assert_eq!(is_narcissistic(407), true);
    }
}
