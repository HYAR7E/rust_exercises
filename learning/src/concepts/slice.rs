fn main() {
    println!("Learning slice");
    let sentence = "Don't let me down";
    let text = String::from("Hello World From My Laptop");
    let target = 3;
    let word1 = get_nth_word(&text, target);
    let word2 = get_nth_word(&sentence, target);
    println!("The {}th word is <{}>", target, word1);
    println!("The {}th word is <{}>", target, word2);

    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let piece = &numbers[3..7];
    println!("3-7: {:?}", piece);
    assert_eq!(piece, [4, 5, 6, 7]);
}

fn get_nth_word(text: &str, target: u32) -> &str {
    let bytes = text.as_bytes();
    let mut counter = 0;
    let mut last_index = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            counter += 1;
            if counter == target {
                return &text[last_index + 1..i];
            }
            last_index = i;
        }
    }
    return &text[..];
}
