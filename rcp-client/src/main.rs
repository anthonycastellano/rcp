use std::env;
use std::process::exit;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

const PORT: u16 = 5050;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Error: A source and destination must be specified.");
        exit(1);
    }

    let file_path = &args[1];
    let dest = &args[2];

    // validate source file
    let src_file = Path::new(file_path);
    if !src_file.exists() {
        println!("Error: The specified source file does not exist.");
        exit(1);
    }

    // set up file
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            println!("Error: Failed to open the file.");
            exit(1);
        },
    };

    // read file into memory
    let mut file_buffer = Vec::new();
    match file.read_to_end(&mut file_buffer) {
        Ok(_) => {
            println!("File size: {}", file_buffer.len());
        }
        Err(_) => {
            println!("Error: Failed to read the file.");
            exit(1);
        }
    }

    // parse host, path
    let target_host: CopyTarget = CopyTarget::new(dest);
    
    // create connection
    let mut stream = match TcpStream::connect(format!("{}:{}", target_host.host, PORT)) {
        Ok(stream) => stream,
        Err(_) => {
            println!("Error: Could not connect to target host");
            exit(1);
        },
    };

    println!("Sending {}...", file_path);
    match stream.write(&file_buffer) {
        Ok(_) => println!("File transfer complete."),
        Err(_) => {
            println!("Error: File transfer failed");
            exit(1);
        },
    };

}

// TOOD: put this in separate file
#[derive(Debug)]
struct CopyTarget<'a> {
    host: &'a str,
    path: &'a str, 
}

impl<'a> CopyTarget<'a> {
    fn new(target_string: &String) -> CopyTarget {
        let split_target_string: Vec<&str> = target_string.split(':').collect();
        if split_target_string.len() != 2 {
            println!("Error: Malformed destination");
            exit(1);
        }

        CopyTarget { host: split_target_string[0], path: split_target_string[1] }
    }
}