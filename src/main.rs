use std::env;
use rs_simple_http_server::Server;

fn main() {
    let mut args = env::args();

    args.next();

    let public_directory = args.next().expect("Deve fornecer o diretório público");

    let address = args.next().expect("Deve fornecer o endereço como segundo argumento.");

    let mut server = Server::new(public_directory, address);

    server.listen();
}
