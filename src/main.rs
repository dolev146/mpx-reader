#![allow(unused)] // silence unused warnings while exploring (to comment out)

use std::{fs};

use walkdir::WalkDir;
const DIR: &str = "./"; 

fn main() {
    println!("Hello, world!!!!");
    
    let mut total_size: u64 = 0;
    let mut total_numbers: u32 = 0;

    for entry in WalkDir::new(DIR).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && !entry.path_is_symlink() {
            total_numbers += 1;
            let metadata = fs::metadata(entry.path()).unwrap();
            total_size += metadata.len();
        }
        // println!("{}", entry.path().display());
    }
    println!("{} , {}", total_numbers , total_size);
}


// used this youtube to start this project
// https://www.youtube.com/watch?v=FzbxAhTqK9s&ab_channel=JeremyChone