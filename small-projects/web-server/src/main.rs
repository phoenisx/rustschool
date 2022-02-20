#![allow(unused)]
use std::fs;
use std::{env, rc::Rc, thread};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let selected_option = args[1].parse::<u8>().expect("Pass a number as argument");
    match selected_option {
        1 => single_threaded_server(),
        2 => single_threaded_server_2(),
        3 => single_threaded_server_3(),
        _ => {}
    }
}

fn single_threaded_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        /**
         * Streams can throw errors: For example, many operating systems
         *      have a limit to the number of simultaneous open connections they
         *      can support; new connection attempts beyond that number will produce
         *      an error until some of the open connections are closed.
         */
        let stream = stream.unwrap();

        /**
         * Seeing multiple messages:
         *  - The reason might be that the browser is making a request
         *    for the page as well as a request for other resources, like the favicon.ico icon
         *    that appears in the browser tab.
         *  - It could also be that the browser is trying to connect to the server multiple
         *    times because the server isn’t responding with any data.
         */
        println!("Connection established!");
    }
}

fn single_threaded_server_2() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    }
}

/**
 * https://doc.rust-lang.org/book/ch20-01-single-threaded.html#writing-a-response
 */
fn single_threaded_server_3() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        handle_connection(stream);
    }
}
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let contents = fs::read_to_string("index.html").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\nContent-Length: {}\n\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        // Respond with 404 for any other type of request or GET Routes.
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

