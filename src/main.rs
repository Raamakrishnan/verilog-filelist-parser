use std::env;
use std::path::PathBuf;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // get .f file
    let dot_f_file = PathBuf::from(&args[1]);
    println!("pathbuf {:?}", dot_f_file);

    // check if .f file exists
    if dot_f_file.is_file() {
        println!("File exists", )
    } else {
        println!("File does not exist", )
    }

    // read .f file
    let contents = fs::read_to_string(&dot_f_file)
        .expect("Error while reading file");

    println!("contents:\n{}", contents);
    // check if all the files exist
    for line in contents.lines() {
        if PathBuf::from(line).is_file() {
            println!("{} exists", line)
        }
        else {
            println!("{} does not exist", line)
        }
    }
}
