use std::io::prelude::*;
use std::io::{self, BufReader};
use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) -> Result<(), io::Error> {
    let mut writer = stream.try_clone()?;
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        let line = line?;
        println!("Received: {}", line);
        let line = format!(">> {}\n", line);
        writer.write_all(line.as_bytes())?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    for stream in listener.incoming() {
        handle_client(stream?)?;
    }
    Ok(())
}
