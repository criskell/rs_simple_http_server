use std::{net::TcpStream, io::{BufReader, BufRead}};

pub struct HttpRequest {
    pub method: String,
    pub path: String,
}

pub fn parse_request(connection: &mut TcpStream) -> HttpRequest {
    let metainfo_lines: Vec<_> = BufReader::new(connection)
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let start_line: Vec<_> = metainfo_lines[0].split(" ").collect();

    let method = start_line[0];
    let path = start_line[1];

    HttpRequest { path: path.to_string(), method: method.to_string() }
}