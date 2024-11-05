mod request;
mod server;

use server::Server;

fn main() {
    let socket = String::from("127.0.0.1:80");
    let server = Server::new(socket);
    server.run();
}
