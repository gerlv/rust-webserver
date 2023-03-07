use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        // A single stream represents an open connection between the client and the server.
        let stream = stream.unwrap();
        println!("Connection established!");
        // When stream goes out of scope and is dropped at the end of the loop, the connection
        // is closed as part of the drop implementation.
    }
}
