use clap::Parser;
use std::path::PathBuf;
use verilog_filelist_parser;

#[derive(Debug, Parser)]
#[clap(name = "verilog-filelist-parser")]
#[clap(long_version(option_env!("LONG_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"))))]
pub struct Opt {
    #[clap()]
    pub filelist: Vec<PathBuf>,
}

pub fn main() {
    let opt: Opt = Parser::parse();

    for filename in &opt.filelist {
        println!("{:?}:", filename);
        let filelist = verilog_filelist_parser::parse_file(filename).expect("Cannot read filelist");

        println!("  files:");
        for file in filelist.files {
            println!("    - {:?}", file);
        }

        println!("  incdirs:");
        for incdir in filelist.incdirs {
            println!("    - {:?}", incdir);
        }

        println!("  defines:");
        for (d, t) in filelist.defines {
            match t {
                None => println!("    {:?}:", d),
                Some(te) => println!("    {:?}: {:?}", d, te),
            };
        }
    }
}
