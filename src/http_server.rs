use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

fn listen() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_request(stream));
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let method = request.split(' ').nth(0).unwrap();

    match method {
        "GET" => {
            let response = "HTTP/1.1 200";
        }
        &_ => todo!(),
    }
}

fn parse_request(request: &[u8]) -> (&str, &str, &str) {
    let request = std::str::from_utf8(request).unwrap();
    let mut parts = request.split_whitespace();
    let method = parts.next().unwrap();
    let path = parts.next().unwrap();
    let http_version = parts.next().unwrap();
    (method, path, http_version)
}
