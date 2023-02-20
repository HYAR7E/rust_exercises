use std::fs::File;
use std::io::{BufRead, BufReader, Read};

use url::Url;

fn main() {
    match File::open("src/data/content.txt") {
        Ok(file) => read_entire_file(&file),
        Err(error) => panic!("Error opening file: {error}"),
    };
    match File::open("src/data/content.txt") {
        Ok(file) => read_file_by_line(file),
        Err(error) => panic!("Error opening file: {error}"),
    };
}

fn read_entire_file(mut file: &File) {
    println!("\n\n### read_entire_file");
    let mut content_buffer = String::new();
    match file.read_to_string(&mut content_buffer) {
        Ok(result) => println!("Result: {result}\n{content_buffer}"),
        Err(error) => panic!("Error reading file: {error}"),
    }
}

fn read_file_by_line(file: File) {
    println!("\n\n### read_file_by_line");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = match line {
            Ok(content) => content,
            Err(error) => panic!("Error: {error}"),
        };
        if line.is_empty() {
            continue;
        }
        if let Some(url) = parse_url(line) {
            println!("{url}");
        }
    }
}

fn parse_url(line: String) -> Option<Url> {
    match Url::parse(&line) {
        Ok(url) => Some(url),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_url() {
        let url_text = String::from("https://docs.rs/gdnative-core/0.7.0/gdnative_core/");
        let url = match Url::parse(&url_text) {
            Ok(url) => url,
            Err(error) => panic!("Error when paring: {error}"),
        };
        assert_eq!(parse_url(url_text), Some(url));
    }

    #[test]
    fn test_parse_url_fail() {
        let url_text = String::from("Hello World!.com");
        assert!(parse_url(url_text).is_none());
    }
}
