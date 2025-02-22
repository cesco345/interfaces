use std::env;
use std::fs::OpenOptions;
use std::io::{self, Error, ErrorKind};
use std::process;
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "--help" {
        eprintln!("Usage: {} file [delay]", args[0]);
        process::exit(1);
    }

    // Use create_new which is equivalent to O_EXCL|O_CREAT
    match OpenOptions::new()
        .write(true)
        .create_new(true)  // This is the key improvement!
        .open(&args[1])
    {
        Ok(_file) => {
            println!(
                "[PID {}] Successfully created file \"{}\" exclusively",
                process::id(),
                args[1]
            );
        }
        Err(e) if e.kind() == ErrorKind::AlreadyExists => {
            println!(
                "[PID {}] File \"{}\" already exists",
                process::id(),
                args[1]
            );
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(e);
        }
    }

    Ok(())
}