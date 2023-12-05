use std::{collections::HashMap, fs::File, io::Read};

pub fn get_file_contents(path: &str) -> String {
    let mut file = File::open(path).expect("Failed to open file");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    contents
}
