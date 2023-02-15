use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the guessing game (1-100)");
    let secret_number = generate_secret_number();
    loop {
        let input_number = input_a_number();
        // Handle wrong input
        if input_number == 0 {
            continue;
        }
        let result = compare_numbers(secret_number, input_number);
        if result {
            println!("Correct! You win");
            break;
        }
    }
}

fn generate_secret_number() -> u32 {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    return secret_number;
}

fn input_a_number() -> u32 {
    // Create variable to store input
    let mut input_number = String::new();
    println!("Guess the number: ");
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
    // Return input
    input_number
}

fn compare_numbers(secret_number: u32, guessed_number: u32) -> bool {
    let result;
    match guessed_number.cmp(&secret_number) {
        Ordering::Less => {
            println!("Too small");
            result = false;
        }
        Ordering::Greater => {
            println!("Too big");
            result = false;
        }
        Ordering::Equal => {
            result = true;
        }
    }
    return result;
}
