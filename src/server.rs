use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        println!("User arrived!");
        let status_line = "HTTP/1.1 200 OK";
        let contents = std::fs::read_to_string("preview/index.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    } else if request_line == "POST /preview HTTP/1.1" {
        println!("User is posting!");
        let mut buffer = [0; 100];
        if let Ok(cnt) = stream.read(&mut buffer) {
            println!("Read: {}", cnt);
            let status_line = "HTTP/1.1 200 OK";
            let contents = "<h1># Foo</h1> <p> Bar baz boo</p>";
            let length = contents.len();

            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
            stream.write_all(response.as_bytes()).unwrap();
        } else {
            println!("err");
            let status_line = "HTTP/1.1 200 OK";
            let response = format!("{status_line}\r\n");
            stream.write_all(response.as_bytes()).unwrap();
        }
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = std::fs::read_to_string("preview/404.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}

pub fn preview_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    println!("Listening at: http://127.0.0.1:8000");

    for stream in listener.incoming() {
        handle_connection(stream?);
    }

    Ok(())
}
