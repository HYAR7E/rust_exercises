use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1] == String::from("--help") {
        println!("FizzBuzz CLI tool");
        println!("\tnumber: input for the fizzbuzz algorithm");
        println!("\t--help: display this help message");
        return Ok(());
    }
    for arg in args[1..].iter() {
        let input = match arg.parse() {
            Ok(i) => i,
            Err(error) => return Err(Box::new(error)),
        };
        let result = fizzbuzz(input);
        println!("{result}");
    }
    Ok(())
}

fn fizzbuzz(i: u8) -> String {
    match (i % 3, i % 5) {
        (0, 0) => String::from("FizzBuzz"),
        (0, _) => String::from("Fizz"),
        (_, 0) => String::from("Buzz"),
        (_, _) => i.to_string(),
    }
}

#[test]
fn test_fizzbuzz() {
    assert_eq!(fizzbuzz(1), "1");
    assert_eq!(fizzbuzz(3), "Fizz");
    assert_eq!(fizzbuzz(5), "Buzz");
    assert_eq!(fizzbuzz(15), "FizzBuzz");
}
