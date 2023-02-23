const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn rustlatin_word(sentence: &str) -> String {
    if sentence.is_empty() {
        return String::new();
    }
    let first_letter = match sentence.chars().nth(0) {
        Some(c) => c,
        None => unreachable!(),
    };
    let mut result = String::from(sentence);
    if VOWELS.contains(&first_letter.to_ascii_lowercase()) {
        let mut prefix = String::from("sr");
        if first_letter.is_ascii_uppercase() {
            prefix.replace_range(0..1, &prefix[..1].to_ascii_uppercase());
        }
        result = result.to_ascii_lowercase();
        result.insert_str(0, &prefix);
    } else {
        let mut sufix = Vec::new();
        loop {
            let letter = result.pop().unwrap();
            if letter.is_alphabetic() {
                result.push(letter);
                break;
            }
            sufix.push(letter);
        }
        sufix.reverse();
        result.push_str("rs");
        result.push_str(&String::from_iter(sufix.iter()));
    };
    result
}

pub fn rustlatin(sentence: &str) -> String {
    let words: Vec<&str> = sentence.split_ascii_whitespace().collect();
    let mut result = Vec::new();
    for word in words {
        result.push(rustlatin_word(word));
    }
    result.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_rustlatin_words_prefix() {
        assert!(rustlatin_word("another").starts_with("sr"));
    }

    #[test]
    fn check_rustlatin_words_sufix() {
        assert!(rustlatin_word("hello").ends_with("rs"));
    }

    #[test]
    fn check_rustlatin_words_uppercase() {
        assert!(rustlatin_word("Another").starts_with("Sra"));
        assert_eq!(rustlatin_word("Hello"), "Hellors");
    }

    #[test]
    fn check_rustlatin_words_with_punctuation() {
        assert_eq!(rustlatin_word("Hello!"), "Hellors!");
        assert_eq!(rustlatin_word("Another..."), "Sranother...");
        assert_eq!(rustlatin_word("Neither?"), "Neitherrs?");
    }

    #[test]
    fn check_rustlatin_sentence() {
        let sentence = "Albion Online is a sandbox mmorpg in which you get to write your own story, instead of following a laid out path. Explore a vast open world consisting of different unique biomes.";
        let rustlatin_sentence = String::from(
            "Sralbion Sronline sris sra sandboxrs mmorpgrs srin whichrs yours getrs tors writers yourrs srown storyrs, srinstead srof followingrs sra laidrs srout pathrs. Srexplore sra vastrs sropen worldrs consistingrs srof differentrs srunique biomesrs."
        );
        assert_eq!(rustlatin(sentence), rustlatin_sentence);
    }
}
