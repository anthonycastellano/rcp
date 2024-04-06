mod server;

use server::Server;

const PORT: u16 = 5050;

fn main() {
    let server: Server = Server::new(PORT);
    server.run();
}
