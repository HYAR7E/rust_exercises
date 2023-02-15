use std::io;

// Fibonacci number
fn main() {
    // Get user input for fibonacci number position
    let position: i32 = 'get_input: loop {
        println!("Enter the position you looking for");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {
                println!("# Invalid input");
                continue 'get_input;
            }
        }
        // Transform from string input to number
        match input.trim().parse() {
            Ok(num) => {
                break 'get_input num;
            }
            Err(_) => {
                println!("# Not a valid number");
                continue 'get_input;
            }
        }
    };
    let mut prev_number = 1;
    let mut current_number = 1;
    for _ in 2..(position) {
        (current_number, prev_number) = (current_number + prev_number, current_number);
    }
    println!("{}° fibonacci number is {}", position, current_number);
}
// 1 2 3 4 5 6  7  8  9
// 1 1 2 3 5 8 13 21 34
