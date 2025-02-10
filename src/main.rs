mod html;
mod lexer;
mod token;

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use crate::html::create_html;
use crate::lexer::Lexer;

const VERSION: &str = "0.1.0";

fn print_help() {
    println!("Usage: mdb [COMMAND]");
    println!("Commands:");
    println!("    h, help            Prints this message.");
    println!("    v, version         Prints mdb version.");
    println!("    p, preview         Spins up a server on which you can live preview how your markdown will look.");
    println!("    g, generate        Generates html articles from your markdown files");
}

enum Command {
    Invalid,
    Help,
    Version,
    Preview,
    Generate,
}

fn generate_blogs() {
    let path = String::from("example/articles/blog1.md");
    let file_content = std::fs::read_to_string(&path).unwrap();

    let lexer = Lexer::new(file_content);
    let tokens = lexer.tokens();

    for token in tokens.iter() {
        println!("{:?}", token);
    }

    let page = create_html(&tokens);
    println!("{page}");
}

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

fn preview_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    println!("Listening at: http://127.0.0.1:8000");

    for stream in listener.incoming() {
        handle_connection(stream?);
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().into_iter().collect();

    if args.len() != 2 {
        println!("Invalid number of arguments!");
        println!("Use --help");
        return Ok(());
    }

    let command = match args[1].as_str() {
        "h" | "help" => Command::Help,
        "v" | "version" => Command::Version,
        "p" | "preview" => Command::Preview,
        "g" | "generate" => Command::Generate,
        _ => Command::Invalid,
    };

    match command {
        Command::Invalid => println!("Invalid command!\nUse --help"),
        Command::Version => println!("Version: {}", VERSION),
        Command::Help => print_help(),
        Command::Preview => preview_server()?,
        Command::Generate => generate_blogs(),
    }

    Ok(())
}
