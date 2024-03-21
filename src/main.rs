use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap()) 
        .take_while(|line| !line.is_empty()) 
        .collect();

    let (status_line, contents) = if http_request.get(0).map_or(false, |line| line.starts_with("GET")) {
        let resource = http_request.get(0).unwrap().split_whitespace().nth(1).unwrap();
        let filename = if resource == "/" {
            "hello.html"
        } else {
            &resource[1..] // remove leading '/'
        };
        match fs::read_to_string(filename) {
            Ok(contents) => ("HTTP/1.1 200 OK", contents),
            Err(_) => ("HTTP/1.1 404 NOT FOUND", fs::read_to_string("notfound.html").unwrap()),
        }
    } else {
        ("HTTP/1.1 400 BAD REQUEST", String::from("Bad request"))
    };

    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}", status_line=status_line, length=length, contents=contents);

    stream.write_all(response.as_bytes()).unwrap();
}
