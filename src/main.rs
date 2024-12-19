#![feature(new_range_api)]

mod parser;

use std::io;
use std::io::Write;

fn main() {
    loop {
        print!("Please enter a string: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match parser::parse(&input) {
            Ok(num) => println!("{}", num),
            Err(err) => println!("{}", err),
        }
    }
}
