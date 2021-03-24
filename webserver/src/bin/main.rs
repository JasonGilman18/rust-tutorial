use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); //bind to the ip and port, panic if err
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() { //incoming creates an iteration of connections from client
        let stream = stream.unwrap();   //a connection is a request and response (from client and server respectively)
                                        //after processing the connection, the connection is closed at the end of the scope
        pool.execute(|| {
            handle_connection(stream);
        });                             
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap(); //pass ref to mut var buffer because we want to fill the exact memory that was allocated

    let root_get_request = b"GET / HTTP/1.1\r\n";
    let other_get_request = b"GET /other HTTP/1.1\r\n";
    
    let (status_line, filename) = if buffer.starts_with(root_get_request) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    }
    else if buffer.starts_with(other_get_request) {
        ("HTTP/1.1 200 OK\r\n\r\n", "other.html")
    }
    else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "error.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response  = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

/*
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; //allocate space on the stack for the request data

    stream.read(&mut buffer).unwrap(); //read from the stream into the buffer, read returns Result
                                       //unwrap panics the program if Err is returned

    //println!("Request: {}", String::from_utf8_lossy(&buffer[..])); //converts buffer to string and prints

    //let response = "HTTP/1.1 200 OK\r\n"; //create a response to send back to client
                                          //format is http-version statusCode phrase then clrf sequence
                                          //then optionally headers then clrf sequence
                                          //then body

    let root_get_request = b"GET / HTTP/1.1\r\n";
    let other_get_request = b"GET /other HTTP/1.1\r\n";

    let contents: String;
    let response: String;
    if buffer.starts_with(root_get_request) {
        contents = fs::read_to_string("hello.html").unwrap(); //read a file and place contents into a string
        response = format!( //use format to make a string with data insterted into it
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", //add the content-length header to ensure valid HTTP response
            contents.len(),
            contents
        );
    }
    else if buffer.starts_with(other_get_request) {
        contents = fs::read_to_string("other.html").unwrap();
        response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
    }
    else {
        contents = fs::read_to_string("error.html").unwrap();
        response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
    }

    stream.write(response.as_bytes()).unwrap(); //write the response to the connection stream
    stream.flush().unwrap(); //flush will wait and prevent the program from progressing until all bytes of response are written to the connection
}
*/