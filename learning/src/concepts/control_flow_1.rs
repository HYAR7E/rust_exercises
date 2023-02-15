use std::io;

fn main() {
    println!("Hello World!");
    let index = 1;

    if index == 0 {
        println!("Presentation");
    } else if index == 1 {
        println!("First page")
    } else {
        println!("Content")
    }

    println!("declare_with_condition: {}", declare_with_condition());

    infinite_exit_option();
}

fn declare_with_condition() -> i32 {
    let n = if true { 5 } else { 1 };
    n
}

fn infinite_exit_option() {
    let mut input;
    let mut number: i32;
    let exit_text = 'get_number_text: loop {
        'get_number_input: loop {
            input = String::new();
            println!("Enter number:");
            io::stdin().read_line(&mut input).expect("Wrong input");
            if input.contains("exit") {
                break 'get_number_text input;
            }
            match input.trim().parse() {
                Ok(num) => {
                    number = num;
                    break 'get_number_input;
                }
                Err(_) => {
                    println!("Not a number");
                    continue 'get_number_input;
                }
            }
        }
        let number = number.to_string();
        if number.contains("7") {
            break 'get_number_text number;
        } else {
            println!("It must contain a 7 if you wanna get out");
            continue 'get_number_text;
        }
    };
    println!("Exit, number used: {}", exit_text);
}
