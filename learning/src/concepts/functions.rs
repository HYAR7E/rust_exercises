fn main() {
    println!("Functions!");
    let p = {
        4;
        2;
        777
    };
    println!("p: {p}");
    let n = seven();
    println!("n: {n}");
    let next_number = next_number(99);
    // This is a sentence (doesn't return anything)
    println!("next_number: {next_number}");
}

fn seven() -> i32 {
    7
}

fn next_number(n: i32) -> i32 {
    // This is a expression (returns a value)
    n + 1
}
