use std::env;
use std::fs;
use std::path::PathBuf;
use sv_filelist_parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filelist = sv_filelist_parser::parse(&args[1]).expect("Error parsing");
    println!("{:#?}", filelist);
}
