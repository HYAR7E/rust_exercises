use colored::Colorize;

fn main() {
    let mut text = String::from("");
    let colored_text = String::from("Hello World!").red();
    let text_ref = &colored_text;
    let text_str = &colored_text.to_string();
    text.push_str(text_ref);
    text.push_str(text_str);
    println!("{}", text);
}
