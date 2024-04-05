use std::env;
use std::process::exit;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::net::TcpStream;

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
    println!("{:?}", target_host);
    
    // create connection
    // let mut stream = TcpStream::connect("127.0.0.1:34254").unwrap();

    println!("Sending {}...", file_path);
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