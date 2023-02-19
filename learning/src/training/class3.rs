use std::{error::Error, fmt::Debug};

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    fn distance(&self, other: &Self) -> f64 {
        1.0
    }
}

trait Distance {
    fn distance(&self, other: &Self) -> f64 {
        2.0
    }
}

impl Distance for Rectangle {}

impl From<(u32, u32)> for Rectangle {
    fn from(value: (u32, u32)) -> Self {
        Self {
            width: value.0,
            height: value.1,
        }
    }
}

// This is already implemented when we write the "From tuple into Rectangle" implementation
// impl Into<Rectangle> for (u32, u32) {
//     fn into(self) -> Self {
//         Self {
//             width: self.0,
//             height: self.1,
//         }
//     }
// }

fn print_generic_impl_debug<T: Debug>(item: T) -> T {
    println!("{:?}", item);
    item
}

fn print_impl(item: impl Debug) -> impl Debug {
    println!("{:?}", item);
    item
}

#[allow(unused)]
fn main() {
    let square1 = Rectangle {
        width: 50,
        height: 50,
    };
    let square2 = Rectangle::from((50, 50));
    let mut square3: Rectangle = (50, 50).into();
    println!("{:?}", square1.distance(&square2));
    println!("{:?}", Rectangle::distance(&square1, &square2));
    println!("{:?}", Distance::distance(&square1, &square2));
    println!("into Square: {:?}", square3);

    println!("Square: {:?}", square3);
    println!("Area: {}", square3.area());
    square3.set_width(10);
    println!("Square: {:?}", square3);
    println!("Area: {}", square3.area());
    Rectangle::set_width(&mut square3, 20);
    println!("Square: {:?}", square3);
    println!("Area: {}", square3.area());

    let res = print_generic_impl_debug::<f64>(32.1);
    let res = print_impl(32);

    dyn_operators();
}

/* dyn operators */

#[derive(Debug)]
struct A(u32);

#[derive(Debug)]
struct B(u32);

fn print<T: Debug>(item: T) {
    println!("{:?}", item);
}

fn dyn_operators() -> Result<(), Box<dyn Error>> {
    println!("dyn_operators");
    let a = A(1);
    let b = B(2);

    let items: Vec<&dyn Debug> = vec![&a, &b];
    for item in items {
        print(item);
    }

    Ok(())
}
