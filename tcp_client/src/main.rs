use std::{
    io::{self, Read, Write},
    net::{Shutdown, TcpStream},
};

fn main() -> io::Result<()> {
    let arg = std::env::args().nth(1);

    let message = match arg {
        Some(msg) => dbg!(msg),
        None => String::from("Default Message"),
    };

    let mut stream = match TcpStream::connect("127.0.0.1:7878") {
        Ok(s) => s,
        Err(e) => {
            println!("Can't reach server");
            return Err(e);
        }
    };
    stream.write(message.as_bytes())?;
    stream.shutdown(Shutdown::Write)?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    println!("Response: {response}");
    Ok(())
}
