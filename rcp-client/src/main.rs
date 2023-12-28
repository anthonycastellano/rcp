use std::env;
use std::process::exit;
use std::path::Path;
use std::fs::File;

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

    // read file into memory
    let mut file = File::open(src_file).unwrap();
    let mut file_buffer: Vec<u8> = Vec::new();

    println!("Sending {}...", file_path)
}
