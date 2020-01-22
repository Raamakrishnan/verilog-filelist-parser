use std::collections::HashMap;

#[derive(PartialEq, Debug)]
enum LineType <'a> {
    File(&'a str),
    IncDir(Vec<&'a str>),
    Define(HashMap<&'a str, &'a str>),
    Filelist(&'a str),
    Unknown,
}

fn parse_line<'a>(line: &'a str) -> LineType<'a> {
    let line = line.trim();
    if line.starts_with("-f ") {
        let filelist_name = line.trim_start_matches("-f ");
        return LineType::Filelist(filelist_name);
    } else if line.starts_with("+define+") {
        // remove +define+ from start and "+" from end
        let defines = line.trim_start_matches("+define+").trim_end_matches("+");
        let mut define_map = HashMap::new();
        for define in defines.split("+") {
            let split: Vec<&str> = define.splitn(2, "=").collect();
            define_map.insert(split[0], split[1]);
        }
        return LineType::Define(define_map);
    } else if line.starts_with("+incdir+") {
        // remove +incdir+ from start and "+" from end
        let incdirs = line.trim_start_matches("+incdir+").trim_end_matches("+");
        let incdir_vec: Vec<&str> = incdirs.split("+").collect();
        return LineType::IncDir(incdir_vec);
    } else {
        return LineType::Unknown;
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
        let line = "+define+CONST1=const1=23\n";
        let mut define_map = HashMap::new();
        define_map.insert("CONST1", "const1=23");
        assert_eq!(parse_line(line), LineType::Define(define_map));
    }

    #[test]
    fn parse_line_define_multiple() {
        let line = "+define+CONST1=const1+CONST2=const2+CONST3=const3=1+\n";
        let mut define_map = HashMap::new();
        define_map.insert("CONST1", "const1");
        define_map.insert("CONST2", "const2");
        define_map.insert("CONST3", "const3=1");
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
        let incdir_vec = vec!["../sample_dir1/sample_dir2", "../sample_dir2/sample_dir3", 
            "sample_dir4/sample_dir5"];
        assert_eq!(parse_line(line), LineType::IncDir(incdir_vec));
    }
}