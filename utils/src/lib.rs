use std::fs;
use std::env;

/// Read content of the file, which passed as first command line argument
pub fn load_data() -> std::io::Result<String> {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("Not enough command line arguments"); }

    let filename: &String = &args[1];
    println!("\nIncoming path: {}", filename);

    return fs::read_to_string(filename);
}
