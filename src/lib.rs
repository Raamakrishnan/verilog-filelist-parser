pub mod line_parser;
pub mod file_parser;

use file_parser::Filelist;
use std::error::Error;

pub fn parse(path: &str) -> Result<Filelist, Box<dyn Error>> {
    file_parser::parse_file(path)
}
