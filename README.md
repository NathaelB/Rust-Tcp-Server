<h3 align="center">Lightning TCP Server</h3>
<p align="center">A server that goes as fast as Lightning McQueen ⚡</p>

![Flash](https://fr.web.img6.acsta.net/r_1280_720/newsv7/17/07/31/15/51/048599.jpg)

Lightning TCP is an open-source project, created in Rust, which brings together various features to create
a server under the TCP protocol.

## ✨ Features

- **Router** with recovery of the url and association of a handler to the route
- **Multi-Threading** association of a thread during a client connection
- **Error handling** connections, sendings and receptions are managed via pattern-matching


## Demo (client TCP)
```rust
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3333").unwrap();

    loop {
        let mut input = String::new();
        println!("Please indicate the route");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let request = format!("GET {} HTTP/1.1\r\n\r\n", input);
                match stream.write(request.as_bytes()) {
                    Ok(_) => {
                        let mut buffer = [0; 512];
                        let bytes_read = stream.read(&mut buffer).unwrap();
                        let response = String::from_utf8_lossy(&buffer[..bytes_read]);

                        if response.contains("404 Not Found") {
                            println!("The road does not exist!");
                        } else {
                            println!("Received response from server: {}", response);
                        }
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        break
                    }
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
```