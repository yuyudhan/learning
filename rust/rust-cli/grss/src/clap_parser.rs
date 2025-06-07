use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The patterhn to loonk for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    println!(" Pattern is {:?}, path is {:?}");
}
