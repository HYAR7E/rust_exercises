#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    score: u64,
}

#[derive(Debug)]
struct Coord(i32, i32, i32);

#[derive(Debug)]
struct Hero(String, i32, char);

fn main() {
    println!("Learning Structs");
    let me = User {
        username: String::from("Neldo"),
        email: String::from("neldo@gmail.com"),
        active: true,
        score: 0,
    };
    let he = User { score: 17, ..me };
    // println!("me: {:#?}", me); // Error
    println!("he: {:#?}", he);

    let origin = Coord(0, 0, 0);
    dbg!(&origin);
    let target = Coord(77, -23, 16);
    dbg!(&target);
    let distance = calc_distance(origin, target);
    dbg!(&distance);

    let mut batman = Hero(String::from("Batman"), 25, 'M');
    batman.1 = 22;
    println!("{:?}", batman);
    let Hero(name, age, genre) = batman;
    dbg!(name, age, genre);
}

fn calc_distance(coord1: Coord, coord2: Coord) -> Coord {
    Coord(
        coord2.0 - coord1.0,
        coord2.1 - coord1.1,
        coord2.2 - coord1.2,
    )
}
