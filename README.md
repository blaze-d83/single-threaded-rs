# Webservers-RS: A Simple Rust Web Server
## Overview
Webservers-RS is a simple and efficient web server implemented in Rust. This project is designed to demonstrate the basic principles of web server functionality, leveraging Rust's performance and safety features. Whether you're learning Rust or looking for a lightweight web server, Webservers-RS is an excellent starting point.

## Features
**Lightweight**: Minimal dependencies and optimized for performance.
**Simple Configuration**: Easy to set up and customize.
**Static File Serving**: Serve static files from a directory.
**Concurrency**: Handle multiple requests concurrently using Rust's async features.

## Project Structure
csharp
Copy code
webservers-rs/
├── src/
│   ├── main.rs
├── static/
│   ├── index.html
│   ├── styles.css
│   └── script.js
├── Cargo.toml
└── README.md

**src/**: Contains the source code for the server.
**static/**: Directory for static files to be served.
**Cargo.toml**: Configuration file for Cargo, the Rust package manager.

## Installation
Clone the repository:
git clone https://github.com/blaze-d83/webservers-rs.git

cd webservers-rs

**Install Rust**: If you don't have Rust installed, you can install it from rust-lang.org.

**Build the project**:
cargo build --release
Usage

## Run the server:
cargo run

## Access the server:
Open your web browser and go to http://127.0.0.1:7878 to see the served static files.

## License
This project is licensed under the MIT License - see the LICENSE file for details.
