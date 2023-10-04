
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
    
};

use hello::ThreadPool;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let _stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(_stream);
        });
        
    }
    println!("Hello, server done!");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // handle end of connection
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    // understand request_line match syntax.
    let(status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" =>  ("HTTP/1.1 200 OK", "hello.html"),
        "GET /thread HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 Not Found", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let resp = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(resp.as_bytes()).unwrap();
  
    // if request_line == "GET / HTTP/1.1" {
    //     let status_line = "HTTP/1.1 200 OK";
    //     let contents = fs::read_to_string("hello.html").unwrap();
    //     let length = contents.len();
    //     let resp = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    //     stream.write_all(resp.as_bytes()).unwrap();
    //     //println!("HTTP request: {:#?}", http_request);
    // } else {
    //     println!("crap.server with request line: {request_line}");
    //     let status_line = "HTTP/1.1 404 OK";
    //     let contents = fs::read_to_string("404.html").unwrap();
    //     let length = contents.len();
    //     let resp = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    //     stream.write_all(resp.as_bytes()).unwrap();
    //     //  let http_request: Vec<_> = buf_reader
    //     // .lines()
    //     // .map(|line| line.unwrap())
    //     // .take_while(|line| line.len() > 0)
    //     // .collect();
    //     // println!("HTTP response: {:#?}", http_request);
    // }
   

   
}
