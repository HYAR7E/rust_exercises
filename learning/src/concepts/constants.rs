use std::io;

const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

fn main() {
    let mut input_index = String::new();
    // Ask user for input
    println!("Enter the index");
    // Input number
    io::stdin()
        .read_line(&mut input_index)
        .expect("Failed to read line");
    // Convert input to number
    let input_index: usize = input_index.trim().parse().expect("Please type a number!");
    // print the month
    println!("The month is {}", MONTH_NAMES[input_index]);
}
