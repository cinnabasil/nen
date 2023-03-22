extern crate nenc;

use std::{ env, path::PathBuf, process::exit, fs::File };

const RED: &str = "\u{001b}[91m";
const RESET: &str = "\u{001b}[0m";

#[derive(Default)]
struct CliOptions {
    input_file: String
}

fn parse_arguments() -> CliOptions {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let mut idx = 0;
    let mut options = CliOptions::default();

    while idx < args.len() {
        let arg = &args[idx];
        idx += 1;

        if arg.starts_with("-") {
            todo!("Flags");
        } else {
            options.input_file = arg.to_string();
        }
    }

    options
}

fn main() {
    let options = parse_arguments();

    if options.input_file.is_empty() {
        eprintln!("{RED}ERROR{RESET} No input file was provided.");
        exit(1);
    }
    
    let path = PathBuf::from(&options.input_file);
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{RED}ERROR{RESET} Could not open file '{}': {e}.", &options.input_file);
            exit(1);
        }
    };

    nenc::compile(file, nenc::CompilerOptions {});
}
