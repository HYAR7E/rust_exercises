fn main() {
    for i in 1..=100 {
        let fizz = fizzbuzz(i);
        println!("{}: {}", i, fizz)
    }
}

fn fizzbuzz(i: i32) -> String {
    match (i % 3, i % 5) {
        (0, 0) => String::from("FizzBuzz"),
        (0, _) => String::from("Fizz"),
        (_, 0) => String::from("Buzz"),
        (_, _) => i.to_string(),
    }
}

#[test]
fn test_fizz() {
    assert_eq!(fizzbuzz(3), "Fizz");
    assert_eq!(fizzbuzz(33), "Fizz");
    assert_eq!(fizzbuzz(63), "Fizz");
}

#[test]
fn test_buzz() {
    assert_eq!(fizzbuzz(5), "Buzz");
    assert_eq!(fizzbuzz(55), "Buzz");
    assert_eq!(fizzbuzz(95), "Buzz");
}

#[test]
fn test_fizzbuzz() {
    assert_eq!(fizzbuzz(15), "FizzBuzz");
    assert_eq!(fizzbuzz(30), "FizzBuzz");
    assert_eq!(fizzbuzz(75), "FizzBuzz");
}
