use std::io;

fn main() {
    println!("Learning References!");
    borrow_lifetime();
    let mut user_input = get_text();
    let input_len = get_length(&user_input);
    println!("The text has {input_len} characters");
    append_sufix(&mut user_input);
    println!("{user_input}");
}

fn get_text() -> String {
    let mut buffer = String::new();
    println!("Write smth");
    io::stdin().read_line(&mut buffer).expect("Invalid input");
    return buffer.trim().to_string();
}

fn get_length(text: &String) -> usize {
    text.len()
}

fn append_sufix(text: &mut String) {
    text.push_str(".")
}

fn borrow_lifetime() {
    let mut text = String::from("Hello World");
    let r1 = &text;
    let r2 = &text;
    println!("{}-{}", r1, r2);
    let r3 = &mut text;
    r3.push_str("!!!");
    println!("{}", r3);
}
