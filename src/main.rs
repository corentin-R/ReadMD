extern crate rand;
extern crate getopts;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        print!("open file : ");
    }
    else{
        println!("Too few or too many arguments");   
    }

    let file_name = args[1].clone();
    println!("{}", file_name);


   // Create a path to the desired file
    let path = Path::new(&file_name);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) =>  panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed

}