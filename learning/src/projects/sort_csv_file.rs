const INPUT_FILE: &str = "./assets/raw.csv";
const OUTPUT_FILE: &str = "./assets/sorted.csv";

use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader, Write},
    path::Path,
};

struct Item {
    last_name: String,
    first_name: String,
    score: i32,
}

fn main() {
    // Get contents of file
    let mut items = read_file(INPUT_FILE);
    // Sort by last_name, first_name and score
    items.sort_by(|a, b| compare_items(a, b));
    // Write to processed file
    generate_file(items);
}

fn read_file(filename: &str) -> Vec<Item> {
    let mut items: Vec<Item> = Vec::new();
    let path = Path::new(&filename);

    // Open file
    let file = File::open(path).expect("Can't open file");

    // Read contents
    let lines = BufReader::new(file).lines();
    for line in lines {
        if let Ok(line) = line {
            let data: Vec<&str> = line.split(',').collect();
            let item: Item = Item {
                last_name: data[0].to_string(),
                first_name: data[1].to_string(),
                score: data[2].trim().parse().unwrap(),
            };
            items.push(item);
        }
    }
    return items;
}

fn compare_items(a: &Item, b: &Item) -> Ordering {
    if a.last_name.eq(&b.last_name) {
        if a.first_name.eq(&b.first_name) {
            return a.score.cmp(&b.score);
        }
        return a.first_name.cmp(&b.first_name);
    }
    return a.last_name.cmp(&b.last_name);
}

fn generate_file(items: Vec<Item>) {
    let filename = Path::new(OUTPUT_FILE);
    let mut file = File::create(filename).expect("Couldn't create sorted file");

    // Write items
    for item in items {
        let mut line = [item.last_name, item.first_name, item.score.to_string()].join(",");
        line.push_str("\n");
        file.write(line.as_bytes())
            .expect("Couln't write to sorted file");
    }
}
