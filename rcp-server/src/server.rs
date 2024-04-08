use std::net::{TcpListener, TcpStream};
use std::process::exit;
use std::io::{Read, Write};
use std::fs;
use std::fs::File;
use std::thread;
use std::io::{BufReader, BufRead};

const DEFAULT_IFACE: &str = "0.0.0.0";
const ACK_SIZE: usize = 2;
const ACK_FLAG_BYTE: u8 = 0x69;
const ACK_VALID_BYTE: u8 = 0x01;
const ACK_INVALID_BYTE: u8 = 0x00;
const NEWLINE_BYTE: u8 = 0x03;
const INVALID_PATH_FIRST_CHARS: [&str; 3] = ["/", "~", "."];

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
                target_path.remove(target_path.len() - 1); // remove stop char

                // validate path
                let mut valid_path: bool = true;
                match target_path.chars().nth(0) {
                    Some(c) => {
                        if INVALID_PATH_FIRST_CHARS.contains(&&c.to_string()[0..1]) {
                            println!("Error: Invalid target path.");
                            valid_path = false;
                        }
                    },
                    None => {
                        println!("Error: Target path is empty (somehow).");
                        valid_path = false;
                    },
                };
                let mut ack_bytes: [u8; ACK_SIZE];
                if valid_path {
                    ack_bytes = [ACK_FLAG_BYTE, ACK_VALID_BYTE];
                } else {
                    ack_bytes = [ACK_FLAG_BYTE, ACK_INVALID_BYTE];
                }
                current_stream.write(&ack_bytes).unwrap();
                current_stream.flush().unwrap();
                if !valid_path {
                    return
                }

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

                // write file
                let mut file: File = match fs::OpenOptions::new().create(true).write(true).open(&target_path) {
                    Ok(f) => f,
                    Err(e) => {
                        println!("Error while opening file for writing: {}", e);
                        return
                    },
                };
                match file.write_all(&file_buf) {
                    Ok(_) => {
                        println!("File created at {}", target_path);
                    },
                    Err(e) => {
                        println!("Error while writing file: {}", e);
                        return
                    },
                };
            });
        }
    }
}