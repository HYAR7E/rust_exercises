use std::io::{prelude::*, BufReader, Result};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> Result<()> {
    let reader = BufReader::new(stream.try_clone()?);
    for line in reader.lines() {
        let line = line?;
        stream.write_all(format!("-> {line}\n").as_bytes())?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    for stream in listener.incoming() {
        match handle_client(stream?) {
            Ok(_) => {}
            Err(e) => eprintln!("Error on stream: {e}"),
        }
    }
    Ok(())
}
