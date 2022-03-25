use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum LineType<'a> {
    File(&'a str),
    IncDir(Vec<&'a str>),
    Define(HashMap<&'a str, Option<&'a str>>),
    Filelist(&'a str),
    Comment,
    Unknown,
    Empty,
}

pub fn parse_line(line: &str) -> LineType {
    let line = line.trim();
    if line.starts_with("-f ") {
        let filelist_name = line.trim_start_matches("-f ");
        LineType::Filelist(filelist_name)
    } else if line.starts_with("+define+") {
        // remove +define+ from start and "+" from end
        let defines = line.trim_start_matches("+define+").trim_end_matches('+');
        let mut define_map = HashMap::new();
        for define in defines.split('+') {
            if let Some(pos) = define.find('=') {
                let (d, t) = define.split_at(pos);
                define_map.insert(d, Some(&t[1..]));
            } else {
                define_map.insert(define, None);
            }
        }
        LineType::Define(define_map)
    } else if line.starts_with("+incdir+") {
        // remove +incdir+ from start and "+" from end
        let incdirs = line.trim_start_matches("+incdir+").trim_end_matches('+');
        let incdir_vec: Vec<&str> = incdirs.split('+').collect();
        LineType::IncDir(incdir_vec)
    } else if line.starts_with("//") || line.starts_with("#") {
        LineType::Comment
    } else if line.starts_with('-') || line.starts_with('+') {
        LineType::Unknown
    } else if line.is_empty() {
        LineType::Empty
    } else {
        // Mark everything else as a File
        LineType::File(line)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_line_filelist() {
        let line = "-f sample/files.f\n";
        assert_eq!(parse_line(line), LineType::Filelist("sample/files.f"));
    }

    #[test]
    fn parse_line_define_single() {
        let line = "+define+CONST1=const1=23+\n";
        let mut define_map = HashMap::new();
        define_map.insert("CONST1", Some("const1=23"));
        assert_eq!(parse_line(line), LineType::Define(define_map));
    }

    #[test]
    fn parse_line_define_multiple() {
        let line = "+define+CONST1=const1+CONST2=const2+CONST3=const3=1+CONST4+CONST5+\n";
        let mut define_map = HashMap::new();
        define_map.insert("CONST1", Some("const1"));
        define_map.insert("CONST2", Some("const2"));
        define_map.insert("CONST3", Some("const3=1"));
        define_map.insert("CONST4", None);
        define_map.insert("CONST5", None);
        assert_eq!(parse_line(line), LineType::Define(define_map));
    }

    #[test]
    fn parse_line_incdir_single() {
        let line = "+incdir+../sample_dir1/sample_dir2\n";
        let incdir_vec = vec!["../sample_dir1/sample_dir2"];
        assert_eq!(parse_line(line), LineType::IncDir(incdir_vec));
    }

    #[test]
    fn parse_line_incdir_multiple() {
        let line = "+incdir+../sample_dir1/sample_dir2+../sample_dir2/sample_dir3+sample_dir4/sample_dir5+\n";
        let incdir_vec = vec![
            "../sample_dir1/sample_dir2",
            "../sample_dir2/sample_dir3",
            "sample_dir4/sample_dir5",
        ];
        assert_eq!(parse_line(line), LineType::IncDir(incdir_vec));
    }

    #[test]
    fn parse_line_comment() {
        let line = "//random_comment";
        assert_eq!(parse_line(line), LineType::Comment);
    }

    #[test]
    fn parse_line_unknown_hyphen() {
        let line = "-funcmd";
        assert_eq!(parse_line(line), LineType::Unknown);
    }

    #[test]
    fn parse_line_unknown_plus() {
        let line = "+funcmd";
        assert_eq!(parse_line(line), LineType::Unknown);
    }

    #[test]
    fn parse_line_empty() {
        let line = "";
        assert_eq!(parse_line(line), LineType::Empty);
        let line = " ";
        assert_eq!(parse_line(line), LineType::Empty);
        let line = "\t";
        assert_eq!(parse_line(line), LineType::Empty);
    }

    #[test]
    fn parse_line_file() {
        let line = "any_random_line_is_a_file";
        assert_eq!(
            parse_line(line),
            LineType::File("any_random_line_is_a_file")
        );
    }
}
