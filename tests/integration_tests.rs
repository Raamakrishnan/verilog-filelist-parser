use std::collections::HashMap;
use std::path::PathBuf;
use verilog_filelist_parser;

#[test]
fn simple_test() {
    let mut defines = HashMap::new();
    defines.insert("a".to_string(), Some("bad".to_string()));
    defines.insert("e".to_string(), Some("f".to_string()));
    defines.insert("c".to_string(), Some("d".to_string()));
    defines.insert("ENV_VAR1".to_string(), Some("var1".to_string()));
    defines.insert("ENV_VAR2".to_string(), Some("var2".to_string()));
    defines.insert("ENV_VAR3".to_string(), Some("var3".to_string()));
    defines.insert("RTL".to_string(), None);

    let filelist_exp = verilog_filelist_parser::Filelist {
        files: vec![
            PathBuf::from("testcase/file1.sv"),
            PathBuf::from("testcase/file2.sv"),
            PathBuf::from("testcase/file3.sv"),
            PathBuf::from("testcase/file4.sv"),
        ],
        incdirs: vec![PathBuf::from("testcase/")],
        defines: defines,
        comments_present: true,
        unknowns_present: false,
    };

    // Add env vars
    std::env::set_var("VAR1", "ENV_VAR1");
    std::env::set_var("VAR2", "ENV_VAR2");
    std::env::set_var("VAR3", "ENV_VAR3");

    let filelist = verilog_filelist_parser::parse_file("testcase/files.f").expect("Error parsing");
    assert_eq!(filelist_exp, filelist);
}
