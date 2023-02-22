use clap::Parser;

#[derive(Parser, Debug, Default)]
struct Args {
    numbers: Vec<u8>,
}

fn main() {
    let args = Args::parse();
    for arg in args.numbers {
        println!("{}", fizzbuzz(arg))
    }
}

fn fizzbuzz(i: u8) -> String {
    match (i % 3, i % 5) {
        (0, 0) => String::from("FizzBuzz"),
        (0, _) => String::from("Fizz"),
        (_, 0) => String::from("Buzz"),
        (_, _) => i.to_string(),
    }
}
