use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::process;

const BUF_SIZE: usize = 1024;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Check command line arguments
    if args.len() != 3 || args[1] == "--help" {
        eprintln!("Usage: {} old-file new-file", args[0]);
        process::exit(1);
    }

    // Open input file for reading
    let mut input_file = match File::open(&args[1]) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening input file {}: {}", args[1], err);
            process::exit(1);
        }
    };

    // Open output file for writing (create, truncate if exists)
    let mut output_file = match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&args[2])
    {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening output file {}: {}", args[2], err);
            process::exit(1);
        }
    };

    let mut buffer = [0u8; BUF_SIZE];

    // Copy data in chunks
    loop {
        match input_file.read(&mut buffer) {
            Ok(0) => break,  // End of file
            Ok(n) => {
                if let Err(err) = output_file.write_all(&buffer[..n]) {
                    eprintln!("Error writing to output file: {}", err);
                    process::exit(1);
                }
            }
            Err(err) => {
                eprintln!("Error reading from input file: {}", err);
                process::exit(1);
            }
        }
    }

    // Files are automatically closed when they go out of scope
    Ok(())
}