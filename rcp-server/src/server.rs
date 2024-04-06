use std::net::{TcpListener, TcpStream};
use std::process::exit;

const DEFAULT_IFACE: &str = "0.0.0.0";

pub struct Server<'a> {
    iface: &'a str,
    port: u16,
}

impl<'a> Server<'a> {
    pub fn new(port: u16) -> Self {
        Self { iface: DEFAULT_IFACE, port }
    }

    pub fn run(&self) {
        println!("RCP server listening on {}:{}", self.iface, self.port);

        let listener = match TcpListener::bind(format!("{}:{}", self.iface, self.port)) {
            Ok(listener) => listener,
            Err(_) => {
                println!("Error: Failed to bind to port");
                exit(1);
            },
        };

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    println!("{:?}", stream);
                },
                Err(e) => println!("Failed to establish connection: {}", e),
            };
        }
    }
}