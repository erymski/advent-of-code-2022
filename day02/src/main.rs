use std::fs::File;
use std::env;
use std::io::{BufReader, BufRead, Seek};

fn first_half(file: &File) -> std::io::Result<()> {

    let reader = BufReader::new(file);

    for line in reader.lines() {

    }
    Ok(())
}


fn second_half(file: &File) -> std::io::Result<()> {

    let reader = BufReader::new(file);

    for line in reader.lines() {

    }
    Ok(())
}

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    println!("\nIncoming path: {}", filename);

    let mut file = File::open(filename)?;
    first_half(&file)?;

    file.rewind()?;
    second_half(&file)?;

    Ok(())
}