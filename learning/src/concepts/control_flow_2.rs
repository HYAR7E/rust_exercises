fn main() {
    while_with_condition();
    iterate_array_with_counter();
    iterate_array_values();
    iterate_range();
}

fn while_with_condition() {
    let mut counter = 0;
    while counter < 5 {
        println!("counter: {counter}");
        counter += 1;
    }
    println!("Finished")
}

fn iterate_array_with_counter() {
    let ar = [101, 102, 103, 104, 105];
    let mut index = 0;
    while index < ar.len() {
        println!("index: {}", ar[index]);
        index += 1;
    }
}

fn iterate_array_values() {
    let ar = ["A", "E", "I", "O", "U"];
    for el in ar {
        println!("el: {el}");
    }
}

fn iterate_range() {
    for n in (5..10).rev() {
        println!("n: {n}");
    }
}
