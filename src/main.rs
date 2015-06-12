extern crate rand;
extern crate getopts;

#[allow(dead_code)]

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error;
use std::string::String;
use std::cmp::Ordering;
use std::process::exit;

const H1 : i32 = 1;
const H2 : i32 = 2;
const H3 : i32 = 3;
const H4 : i32 = 4;
const H5 : i32 = 5;
const H6 : i32 = 6;

const BOLD : i32 = 10;
const ITALIC : i32 = 11;
const UNDERLINE : i32 = 12;

//src: http://stackoverflow.com/questions/5412761/using-colors-with-printf
const NRM  : &'static str = "\x1B[0m";
const RED  : &'static str = "\x1B[31m";
const GRN  : &'static str = "\x1B[32m";
const YEL  : &'static str = "\x1B[33m";
const BLU  : &'static str = "\x1B[34m";
const MAG  : &'static str = "\x1B[35m";
const CYN  : &'static str = "\x1B[36m";
const WHT  : &'static str = "\x1B[37m";

const B : &'static str = "\033[1m";
//TODO: S, U, HYPERLINK...


fn check_title(line: String) -> usize {

    //TODO: if ">xx##"

    if line.is_empty() {
        return 0;
    }

    let mut count = 7;

    while count>1{
        count -= 1;
        let mut slice = String::new();
        slice = line.clone();
        while slice.len() < count {
            count -= 1;
        }
        slice.truncate(count);
        //print!("slice: {},  ", slice);
        let mut test = String::new();
        for _ in 0..count {
            test.push('#');
        }

        //println!("test: {}", test);
        //check if equal (or contains)
        let boolean = slice == test;
        //println!("{} res: {}", BLU,boolean); 
        if boolean {
            return count;
        }
    }
    return 0;
}

fn open_file(args: Vec<String>) -> String {

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
        Ok(_) => print!("open file: {}\n", display),
    }
    return s;
}


fn main() {

    let args: Vec<String> = env::args().collect();

    let plain_text = open_file(args);

    //split s line per line
    let s2  = plain_text.clone();

    let lines_vector: Vec<&str> = s2.split("\n").collect();
    //println!("{:?}", lines_vector);

    for one_line in lines_vector {
        let res = check_title(one_line.to_string().clone());
        if res < 10 && res > 0 {
            println!("{}", one_line);
        }

    }

}