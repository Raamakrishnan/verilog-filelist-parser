use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::line_parser;
use crate::line_parser::LineType;

#[derive(PartialEq, Debug, Default)]
pub struct Filelist {
    pub files: Vec<PathBuf>,
    pub incdirs: Vec<PathBuf>,
    pub defines: HashMap<String, Option<String>>,
    pub comments_present: bool,
    pub unknowns_present: bool,
}

impl Filelist {
    pub fn new() -> Filelist {
        Filelist {
            files: Vec::new(),
            incdirs: Vec::new(),
            defines: HashMap::new(),
            comments_present: false,
            unknowns_present: false,
        }
    }

    pub fn extend(&mut self, other: Filelist) {
        self.files.extend(other.files);
        self.incdirs.extend(other.incdirs);
        self.defines.extend(other.defines);
        self.comments_present |= other.comments_present;
        self.unknowns_present |= other.unknowns_present;
    }
}

pub fn parse_file(path: &str) -> Result<Filelist, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;

    let mut filelist = Filelist::new();

    for line in contents.lines() {
        let line = replace_env_vars(&line);
        match line_parser::parse_line(&line) {
            LineType::File(file) => filelist.files.push(PathBuf::from(file)),
            LineType::Define(define_map) => {
                for (d, t) in define_map.into_iter() {
                    match t {
                        Some(text) => filelist
                            .defines
                            .insert(d.to_string(), Some(text.to_string())),
                        None => filelist.defines.insert(d.to_string(), None),
                    };
                }
            }
            LineType::IncDir(incdirs) => {
                for dir in incdirs {
                    filelist.incdirs.push(PathBuf::from(dir));
                }
            }
            LineType::Comment => filelist.comments_present = true,
            LineType::Unknown => filelist.unknowns_present = true,
            LineType::Filelist(path) => {
                filelist.extend(parse_file(path)?);
            }
        }
    }
    Ok(filelist)
}

fn replace_env_vars(line: &str) -> String {
    let re_env_brace = Regex::new(r"\$\{(?P<env>[^}]+)\}").unwrap();
    let re_env_paren = Regex::new(r"\$\((?P<env>[^)]+)\)").unwrap();

    let mut expanded_line = String::from(line);
    for caps in re_env_brace.captures_iter(&line) {
        let env = &caps["env"];
        if let Ok(env_var) = std::env::var(env) {
            expanded_line = expanded_line.replace(&format!("${{{}}}", env), &env_var);
        }
    }
    for caps in re_env_paren.captures_iter(&line) {
        let env = &caps["env"];
        if let Ok(env_var) = std::env::var(env) {
            expanded_line = expanded_line.replace(&format!("$({})", env), &env_var);
        }
    }
    expanded_line
}
