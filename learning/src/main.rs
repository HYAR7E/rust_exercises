#[allow(unused)]

use std::{fs::File, io::Error};

struct ObjectPoint {
    x: f64,
    y: f64,
}

struct TuplePoint (f64, f64);

#[derive(Debug)]
enum Directions {
    North,
    East {distance: f64},
    West {distance: f64},
    South {target: String},
}

#[allow(unused_variables)]
fn main() {
    // use Directions::*;
    // North;
    
    let name = dbg!(String::from("Neldo"));

    let coord = ObjectPoint{
        x: 1.2,
        y: 2.0,
    };
    let coord = TuplePoint(1.0, 2.0);
    let coord = TuplePoint{
        0: 1.0,
        1: 2.0
    };
    let direction = Directions::North;
    println!("{:?}", direction);
    let direction = Directions::East { distance: 1.0 };
    println!("{:?}", direction);
    let direction = Directions::South { target: String::from("asdlkj") } ;
    println!("{:?}", direction);

    let north = Directions::North;
    match north {
        Directions::North => {},
        _ => todo!(),
    };

    let possible = Some(15);
    if let Some(i) = possible {
        println!("{i}");
    };
}

fn fizzbuzz(i: u32) -> String{
    match dbg!(i%3, i%5) {
        (0, 0) => String::from("FizzBuzz"),
        (0, _) => String::from("Fizz"),
        (_, 0) => String::from("Buzz"),
        (a, b) if a == b => unreachable!(),
        (_, _) => i.to_string(),
    }
}

fn main2() -> Result<(), Error>{
    let file = File::open("none.file")?;
    Ok(())
}

#[test]
fn validate_fizzbuzz() {
    assert_eq!(fizzbuzz(3), "Fizz");
    assert_eq!(fizzbuzz(5), "Buzz");
    assert_eq!(fizzbuzz(30), "FizzBuzz");
    assert_eq!(fizzbuzz(7), "7");
}
