use std::net::{TcpListener, TcpStream};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(&self) {
        println!("RCP server listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {

                },
                Err(e) => println!("Failed to establish connection: {}", e),
            }
        }
    }
}