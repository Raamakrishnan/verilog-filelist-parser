#[derive(PartialEq, Debug)]
pub enum LineType {
    File,
    Define,
    Filelist,
    Unknown,
}

pub fn parse_line(line: &str) -> LineType {
    if line.starts_with("-f ") {
        return LineType::Filelist;
    } else if line.starts_with("+define+") {
        return LineType::Define;
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
        assert_eq!(parse_line(line), LineType::Filelist);
    }

    #[test]
    fn parse_line_define() {
        let line = "+define+CONST1=const1+CONST2=const2";
        assert_eq!(parse_line(line), LineType::Define);
    }
}