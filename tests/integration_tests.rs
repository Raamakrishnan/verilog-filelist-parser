extern crate sv_filelist_parser;
use std::collections::HashMap;

#[test]
fn simple_test() {
    let mut defines = HashMap::new();
    defines.insert("a".to_string(), "bad".to_string());
    defines.insert("e".to_string(), "f".to_string());
    defines.insert("c".to_string(), "d".to_string());

    let filelist_exp = sv_filelist_parser::file_parser::Filelist {
        files : vec!["testcase/file1.sv".to_string(),
        "testcase/file2.sv".to_string(),
        "testcase/file3.sv".to_string(),
        "testcase/file4.sv".to_string(),],
        incdirs : vec!["testcase/".to_string()],
        defines : defines,
        comments_present : true
    };
    let filelist = sv_filelist_parser::parse("testcase/files.f")
        .expect("Error parsing");
    assert_eq!(filelist_exp, filelist);
}