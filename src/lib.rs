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
        let defines = line.trim_start_matches("+define+");
        let mut define_map = HashMap::new();
        for define in defines.split("+") {
            let split: Vec<&str> = define.splitn(2, "=").collect();
            define_map.insert(split[0], split[1]);
        }
        return LineType::Define(define_map);
    } else {
        return LineType::Unknown;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_line_filelist() {
        let line = "-f sample/files.f";
        assert_eq!(parse_line(line), LineType::Filelist("sample/files.f"));
    }

    #[test]
    fn parse_line_define() {
        let line = "+define+CONST1=const1+CONST2=const2+CONST3=const3=1";
        let mut define_map = HashMap::new();
        define_map.insert("CONST1", "const1");
        define_map.insert("CONST2", "const2");
        define_map.insert("CONST3", "const3=1");
        assert_eq!(parse_line(line), LineType::Define(define_map));
    }
}