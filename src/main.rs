#![allow(non_snake_case)]

use std::{env::args, fs::read_to_string, process::exit};

fn main() {

    let args: Vec<String> = args().collect();
    dbg!(&args);

    let target_file_result = args.get(1);
    if target_file_result.is_none() {
        displayHelp();
    }
    let target_file = target_file_result.unwrap().clone();

    if !target_file.contains(".java") {
        println!("it is probably NOT a java file");
    }

    // todo: check if the target file is actually a java file

    let file_contents_result = read_to_string(&target_file);

    if file_contents_result.is_err() {
        println!("File '{target_file}' does not exit");
        exit(1);
    }

    let file_contents = file_contents_result.unwrap();

    for line in file_contents.lines() {
        if line.contains("loadLibrary(\"") {
            println!("entry point found");
        }
        if line.contains("native") {
            println!("a match");
        }
    }
}

fn _buildFromJava() {
    todo!()
}

fn _buildFromC() {
    todo!()
}

fn displayHelp() {
    println!(
        "
    Quick JNI help goes here
    "
    );
    todo!();
    exit(0);
}
