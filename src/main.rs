use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // Bind the TCP Listener to the port address
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Listen for incoming connections
    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        // Call handle_connection to process the request
        handle_connection(_stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Store incoming request in a buffer
    let mut buffer = [0; 1024];

    // Read the incoming request from the client into the buffer
    stream.read(&mut buffer).unwrap();

    // Define byte array representing GET request for the root path
    let get_root = b"GET / HTTP/1.1\r\n";
    let get_css  = b"GET /styles.css HTTP/1.1\r\n";
    let get_js   = b"GET /scripts.js HTTP/1.1\r\n";

    // Determine status line and filename based on the request
    let (status_line, filename) = if buffer.starts_with(get_root) {
        ("HTTP/1.1 200 OK", "static/index.html")
    } else if buffer.starts_with(get_css) {
        ("HTTP/1.1 200 OK", "static/styles.css")
    } else if buffer.starts_with(get_js) {
        ("HTTP/1.1 200 OK", "static/scripts.js")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "static/404.html")
    };

    // Attempt to read the contents of the file
    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        // If the file cannot be read send an error
        Err(_) => {
            let response = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
            return;
        }
    };

    // Construct appropriate HTTP response with status line, content length and contents
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    // Write response to the TCP stream and flush it
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
