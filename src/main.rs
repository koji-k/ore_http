use std::net::{TcpStream, TcpListener};
use std::io::Write;
use std::io::Read;
use std::thread;
//use std::str::FromStr;
extern crate ore_http;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8931").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || { response(stream); });
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    drop(listener);
}

fn response(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    let mut request_header = "";
    loop {
        match stream.read(&mut buf) {
            Err(e) => panic!("Got an error: {}", e),
            Ok(0) => {
                println!("Break now!");
                break;
            }
            Ok(_) => {
                request_header = std::str::from_utf8(&mut buf).unwrap().trim();
                break;
            }
        };
    }
    let mut request_header_lines = request_header.lines();
    let path = match request_header_lines.next() {
        Some(s) => {
            let mut info = s.split_whitespace();
            let _method = info.next();
            let path = info.next();
            let _protocol = info.next();
            match path {
                Some(p) => p,
                None => panic!("...?"),
            }
        }
        None => panic!("...??"),
    };

    let body = &ore_http::my_file::read_file(path);
    let content_length = body.len();

    let http_status = "HTTP/1.1 200 OK \r\n";
    let http_header = &format!(
        "{}{}{}{}",
        "Content-Type: text/html; charset=UTF-8\r\n",
        "Content-Length: ",
        content_length,
        "\r\n"
    );
    let mut msg = String::new();
    msg.push_str(http_status);
    msg.push_str(http_header);
    msg.push_str("Connection: Close\r\n");
    msg.push_str("\r\n");
    msg.push_str(body);
    stream.write(msg.as_bytes()).unwrap();

}
