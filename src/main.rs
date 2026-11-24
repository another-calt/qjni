#![allow(non_snake_case)]

use clap::Parser;
use std::process::exit;

mod args;

fn main() {

    let args = args::Args::parse(); // theyre args, if you couldnt tell

    // locate source file
    let target_file = args.file;
    if !target_file.contains(".java") {
        println!("\x1b[1;31merror\x1b[0m: file must be '.java' file");
        exit(1);
    }

    // locate build file
    let native_target = args.format.unwrap_or(".c".to_string());
    let sperator = if native_target.contains('.') {""} else {"."};
    let file_name = args.output_name.unwrap_or(target_file.split(".java").collect()) + sperator + &native_target;
    dbg!(file_name);

    if std::fs::read(target_file).is_err() {
        println!("\x1b[1;31merror\x1b[0m: file does not exist");
        exit(1);
    }

}
