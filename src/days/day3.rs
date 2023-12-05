use std::{cell::RefCell};
mod util;

pub fn main(){
    const DAY: u8 = 3;

    println!("\n\n\nAdvent of Code day {DAY}");

    let contents = util::get_file_contents("data/day3/test.txt");

    let lines = contents.lines();

    let line = lines.clone().nth(0).unwrap();

    println!("first line:\n{line}");
}