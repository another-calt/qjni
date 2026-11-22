use std::{env::args, process::exit};
#[allow(non_snake_case)]

fn main() {

   let args: Vec<String> = args().collect();
   dbg!(&args);

   let target_file_result = args.get(1);
   if target_file_result.is_none() {
      displayHelp();
   } 
   let target_file = target_file_result.unwrap().clone();
    
}

fn displayHelp() {
   println!("help text goes here");
   todo!();
   exit(0);
}
