use std::net::{TcpListener, TcpStream};
use std::process::exit;
use std::io::{Read, Write};
use std::thread::{self, current};
use std::io::{BufReader, BufRead};

const DEFAULT_IFACE: &str = "0.0.0.0";
const ACK_SIZE: usize = 2;
const ACK_FLAG_BYTE: u8 = 0x69;
const ACK_VALID_BYTE: u8 = 0x01;
const ACK_INVALID_BYTE: u8 = 0x00;
const NEWLINE_BYTE: u8 = 0x03;

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

                // get target path
                let mut packet: BufReader<&TcpStream> = BufReader::new(&current_stream);
                let mut target_path_bytes: Vec<u8> = Vec::new();
                let mut target_path: String;
                match packet.read_until(NEWLINE_BYTE, &mut target_path_bytes) {
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
                    Err(_) => return,
                };

                // validate path
                let temp_res: [u8; ACK_SIZE] = [ACK_FLAG_BYTE, ACK_VALID_BYTE];
                current_stream.write(&temp_res).unwrap();
                current_stream.flush().unwrap();

                // get file size
                let mut file_size_bytes: [u8; 8] = [0; 8];
                let mut file_size: u64;
                match current_stream.read_exact(&mut file_size_bytes) {
                    Ok(_) => {
                        file_size = u64::from_be_bytes(file_size_bytes);
                    },
                    Err(_) => return,
                };
                println!("File size: {}", file_size);

                // get file
                let mut file_buf: Vec<u8> = vec![0; file_size as usize];
                match current_stream.read_exact(&mut file_buf) {
                    Ok(_) => (),
                    Err(_) => {
                        println!("Error: Error occurred during file transfer.");
                        return
                    },
                }
                println!("{:?}", file_buf);
            });
        }
    }
}