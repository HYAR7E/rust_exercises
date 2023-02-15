fn main() {
    println!("Learning Ownership!");
    function_scope();
}

#[allow(unused)]
fn example_move() {
    let mut hw = String::from("Hello");
    hw.push_str(" World!");
    println!("{hw}");

    // Move hw to hw_copy
    let mut hw_copy = hw;
    hw_copy.push_str(" This is a copy");
    println!("{hw_copy}");

    // Move hw_copy to hw
    hw = hw_copy;
    println!("{hw}");
}

#[allow(unused)]
fn example_clone() {
    let h = String::from("Hello");
    let mut hw = h.clone();
    hw.push_str(" World!");
    println!("h: {h}");
    println!("hw: {hw}");
}

fn function_scope() {
    let hw = String::from("Hello World!");
    // hw moves to function param
    takes_ownership(hw);
    // hw is invalid here

    let mine = gives_ownership();
    let (length, mine) = calculate_length(mine);
    println!("The length of {mine} is {length}")
}

fn takes_ownership(text: String) {
    // text is hw on a memory level
    println!("Moved value: {text}");
}

fn gives_ownership() -> String {
    String::from("Yours")
}

fn calculate_length(text: String) -> (usize, String) {
    (text.len(), text)
}
