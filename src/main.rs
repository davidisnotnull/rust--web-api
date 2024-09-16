use std::net::TcpListener;

mod connection_handler;
mod entities;
mod models;
mod password_helper;
mod error_handler;

fn main() -> std::io::Result<()> {
    println!("Starting listener...");
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server listening on port 7878");

    for stream in listener.incoming() {
        let stream = stream?;
        connection_handler::handle_connection(stream);
    }

    Ok(())
}
