use std::io;

fn main() {
    println!("Fizzbuzz script!");
    let input = input_a_number();
    let chain = fizzbuzz(input);
    println!("chain: {chain}");
}

fn input_a_number() -> u32 {
    // Create variable to store input
    let mut input_number = String::new();
    println!("Enter a number: ");
    io::stdin()
        .read_line(&mut input_number)
        .expect("Failed to read line");
    // Convert string to number
    let input_number: u32 = match input_number.trim().parse() {
        Ok(num) => {
            if num < 1 || num > 100 {
                return 0;
            }
            return num;
        }
        Err(_) => 0,
    };
    input_number
}

fn fizzbuzz(i: u32) -> String {
    let mut result = String::new();
    if i % 3 == 0 {
        result.push_str("Fizz");
    }
    if i % 5 == 0 {
        result.push_str("Buzz");
    };
    if result != "" { result } else { i.to_string() }
}

#[test]
fn validate_fizzbuzz() {
    assert_eq!(fizzbuzz(3), "Fizz");
    assert_eq!(fizzbuzz(5), "Buzz");
    assert_eq!(fizzbuzz(30), "FizzBuzz");
    assert_eq!(fizzbuzz(7), "7");
}
