use std::{
    path::Path,
    net::{TcpListener, TcpStream},
    fs::read_to_string,
    io::prelude::*
};
use crate::http;

pub struct Server {
    pub public_directory: String,
    pub address: String,
    listener: Option<TcpListener>
}

impl Server {
    pub fn new(public_directory: String, address: String) -> Server {
        Server {
            public_directory,
            address,
            listener: None
        }
    }

    pub fn listen(&mut self) {
        println!("Ouvindo conexões em: {}", self.address);

        self.listener = Some(TcpListener::bind(&self.address).unwrap());

        self.receive_connections();
    }

    fn receive_connections(&mut self) {
        let listener = self.listener.as_ref().unwrap();

        for connection_attempt in listener.incoming() {
            let connection = connection_attempt.unwrap();

            self.handle_connection(connection);
        }
    }

    fn handle_connection(&self, mut connection: TcpStream) {
        let request = http::parse_request(&mut connection);

        let path = Path::new(&self.public_directory).join(&request.path[1..]);
        let content = read_to_string(path);

        let (status_line, content_type, payload) = match content {
            Err(err) => {
                println!("{}", err);
                ("HTTP/1.1 404 NOT FOUND", "text/html", "<h1>Não encontrado.</h1>".to_string())
            },
            Ok(content) => ("HTTP/1.1 200 OK", "text/plain", content),
        };

        let length = payload.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: {content_type}\r\n\r\n{payload}");

        connection.write_all(response.as_bytes()).unwrap();
    }
}