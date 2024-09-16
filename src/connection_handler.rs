use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use uuid::Uuid;
use crate::entities;

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    if let Ok(bytes_read) = stream.read(&mut buffer) {
        if bytes_read == 0 {
            return;
        }

        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        let mut lines = request.lines();

        if let Some(request_line) = lines.next() {
            let mut parts = request_line.split_whitespace();
            let method = parts.next().unwrap_or("");
            let path = parts.next().unwrap_or("");

            println!("Method: {}, Path: {}", method, path);

            let response = match (method, path) {
                ("GET", "/") => build_response("Reality is that which, when you stop believing in it, doesn't go away."),
                ("GET", "/hello") => build_response("Hello, World!"),
                ("GET", "/user") => {
                    let user = entities::User {
                        id: Uuid::new_v4(),
                        name: "Alice".to_string(),
                        email: "alice@example.com".to_string(),
                    };
                    let body = user_to_json(&user);
                    build_response_with_content_type(&body, "application/json")
                },
                ("GET", "/guid") => {
                    build_response(&(*Uuid::new_v4().to_string()))
                },
                ("POST", "/echo") => {
                    let body = request.split("\r\n\r\n").nth(1).unwrap_or("");
                    let response_body = format!("Received: {}", body);
                    build_response(&response_body)
                },
                _ => build_response_404(),
            };

            if let Err(e) = stream.write(response.as_bytes()) {
                eprintln!("Failed to write to connection: {}", e);
            }
            if let Err(e) = stream.flush() {
                eprintln!("Failed to flush stream: {}", e);
            }
        } else {
            let response = build_response_400();
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}

// Helper functions (make them private since they are only used within this module)
fn build_response(body: &str) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
        body.len(),
        body
    )
}

fn build_response_with_content_type(body: &str, content_type: &str) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",
        body.len(),
        content_type,
        body
    )
}

fn build_response_404() -> String {
    let body = "404 Not Found";
    format!(
        "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
        body.len(),
        body
    )
}

fn build_response_400() -> String {
    let body = "400 Bad Request";
    format!(
        "HTTP/1.1 400 BAD REQUEST\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
        body.len(),
        body
    )
}

fn user_to_json(user: &entities::User) -> String {
    format!(
        "{{\"id\": {}, \"name\": \"{}\", \"email\": \"{}\"}}",
        user.id, user.name, user.email
    )
}
