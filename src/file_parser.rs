use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use crate::line_parser;
use crate::line_parser::LineType;

/// Represents a Verilog Filelist
#[derive(PartialEq, Debug, Default)]
pub struct Filelist {
    /// List of all files
    pub files: Vec<PathBuf>,
    /// List of all Include Directories
    pub incdirs: Vec<PathBuf>,
    /// HashMap of all Defines
    pub defines: HashMap<String, Option<String>>,
    /// True if comments are present in the filelist
    pub comments_present: bool,
    /// True if unknown arguments are present in the filelist
    pub unknowns_present: bool,
}

impl Filelist {
    /// Returns an empty Filelist
    pub fn new() -> Filelist {
        Filelist {
            files: Vec::new(),
            incdirs: Vec::new(),
            defines: HashMap::new(),
            comments_present: false,
            unknowns_present: false,
        }
    }

    /// Adds the elements of the other filelist to the current filelist
    pub fn extend(&mut self, other: Filelist) {
        self.files.extend(other.files);
        self.incdirs.extend(other.incdirs);
        self.defines.extend(other.defines);
        self.comments_present |= other.comments_present;
        self.unknowns_present |= other.unknowns_present;
    }
}

/// Parses a filelist file.
///
/// Environment variables represented with paranthesis or
/// curly braces (i.e. `$()` or `${}`) will be automatically
/// substituted.
///
/// # Arguments
///
/// * `path` - The path to the filelist
///
/// # Errors
///
/// Returns an error if the filelist in `path` cannot be read. Also returns
/// error if any of the nested filelists cannot be read.
pub fn parse_file(path: impl AsRef<Path>) -> Result<Filelist, Box<dyn Error>> {
    let path = path.as_ref();
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
            LineType::Empty => (),
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
    let re_env_bare = Regex::new(r"\$(?P<env>[a-zA-Z_][a-zA-Z0-9_]*)").unwrap();

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
    for caps in re_env_bare.captures_iter(&line) {
        let env = &caps["env"];
        if let Ok(env_var) = std::env::var(env) {
            expanded_line = expanded_line.replace(&format!("${}", env), &env_var);
        }
    }
    expanded_line
}
