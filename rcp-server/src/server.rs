use std::net::{TcpListener, TcpStream};
use std::process::exit;
use std::io::Read;
use std::thread;
use std::io::{BufReader, BufRead};

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

        for stream in listener.incoming() {
            thread::spawn(|| {
                let mut current_stream: TcpStream = match stream {
                    Ok(s) => s,
                    Err(_) => {
                        return
                    },
                };

                println!("New connection from {}", current_stream.peer_addr().unwrap());

                let mut packet = BufReader::new(&current_stream);
                let mut target_path_bytes: Vec<u8> = Vec::new();
                let mut target_path: String;
                match packet.read_until(0x03, &mut target_path_bytes) {
                    Ok(bytes) => {
                        target_path = match String::from_utf8(target_path_bytes) {
                            Ok(str) => str,
                            Err(_) => {
                                println!("Error: Target path is not a valid string");
                                return
                            },
                        };
                        println!("Successfully read target path ({} bytes): {}", bytes, target_path);
                    },
                    Err(_) => {
                        return
                    },
                }
                // current_stream.read_to_string(&mut target_path).unwrap(); 
                // println!("Target path for transfer: {}", target_path);
            });
            
        }
    }
}