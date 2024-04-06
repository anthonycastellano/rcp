use std::process::exit;

#[derive(Debug)]
pub struct CopyTarget<'a> {
    pub host: &'a str,
    pub path: &'a str, 
}

impl<'a> CopyTarget<'a> {
    pub fn new(target_string: &String) -> CopyTarget {
        let split_target_string: Vec<&str> = target_string.split(':').collect();
        if split_target_string.len() != 2 {
            println!("Error: Malformed destination");
            exit(1);
        }

        CopyTarget { host: split_target_string[0], path: split_target_string[1] }
    }
}