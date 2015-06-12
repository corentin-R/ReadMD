extern crate rand;
extern crate getopts;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error;
use std::string::String;
use std::cmp::Ordering;
use std::process::exit;

fn main() {

    let args: Vec<String> = env::args().collect();

    match args.len().cmp(&2)  {
        Ordering::Less    => {
            println!("Too few arguments");
            exit(-1);
        }, 
        Ordering::Greater => {
            println!("Too many arguments");
            exit(-1);
        }, 
        Ordering::Equal => print!(""), //TODO: fix-->do nothing 
    }

    let file_name = args[1].clone();

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
        Ok(_) => print!("open file: {}\n{}", display, s),
    }

   //TODO: s line per lien

    let mut count = 7;
    while count>1{
        count -= 1;

        let mut test = String::new();
        for j in 0..count {
            test.push('#');
        }
        println!("-->{}", test);
    }

    //pick n fisrt char
    let slice = &"####EE"[..4];
    println!("{}", slice == "####");

    //chack if equal (or contains)
    let boolean = s.contains("ze");
    println!("{}", boolean);


}