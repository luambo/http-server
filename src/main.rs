use std::{
    fs,
    io::{prelude::*, BufRead, BufReader},
    net::{TcpListener,TcpStream},
};
fn main() {
    let listener:TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //listening for incoming streams and handling HTTP request data;
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream:TcpStream) -> () {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines()
        .next()
        .unwrap().unwrap();

    if request_line == "GET / HTTP/1.1"{ 
        stream.write_all(content_redirect("index.html", "HTTP/1.1 200 OK").as_bytes()).unwrap(); // add error handling here in the future;
    }else {
        stream.write_all(content_redirect("else.html", "HTTP/1.1 404 NOT FOUND").as_bytes()).unwrap();
    }
} 
// GET requests have no body.

fn content_redirect(html:&str, status:&str) -> String{
    let content = fs::read_to_string(html).unwrap();
    let length = content.len();
    format!("{status}\r\nContent-Length:{length}\r\n\r\n{content}")
}