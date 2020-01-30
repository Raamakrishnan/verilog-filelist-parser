use sv_filelist_parser;
use std::collections::HashMap;

#[test]
fn simple_test() {
    let mut defines = HashMap::new();
    defines.insert("a".to_string(), Some("bad".to_string()));
    defines.insert("e".to_string(), Some("f".to_string()));
    defines.insert("c".to_string(), Some("d".to_string()));
    defines.insert("ENV_VAR1".to_string(), Some("var1".to_string()));
    defines.insert("ENV_VAR2".to_string(), Some("var2".to_string()));
    defines.insert("RTL".to_string(), None);

    let filelist_exp = sv_filelist_parser::Filelist {
        files : vec!["testcase/file1.sv".to_string(),
        "testcase/file2.sv".to_string(),
        "testcase/file3.sv".to_string(),
        "testcase/file4.sv".to_string(),],
        incdirs : vec!["testcase/".to_string()],
        defines : defines,
        comments_present : true
    };

    // Add env vars
    std::env::set_var("VAR1", "ENV_VAR1");
    std::env::set_var("VAR2", "ENV_VAR2");

    let filelist = sv_filelist_parser::parse("testcase/files.f")
        .expect("Error parsing");
    assert_eq!(filelist_exp, filelist);
}