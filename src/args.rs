use clap::Parser;

/// Quick JNI | autogenerate JNI bindings for C and C++
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Target '.java' file
    pub file: String,

    /// Target native format ( '.c', '.cpp', etc )
    #[arg(short, long, value_name = "FORMAT")]
    pub format: Option<String>,

    /// Name of resulting file
    #[arg(short, long, value_name = "OUTPUT")]
    pub output_name: Option<String>,
}
