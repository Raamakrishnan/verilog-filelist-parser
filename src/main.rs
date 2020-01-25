use std::env;
use std::path::PathBuf;
use std::fs;
use sv_filelist_parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filelist = sv_filelist_parser::parse(&args[1])
        .expect("Error parsing");
    println!("{:#?}", filelist);
}
