use std::env;
use std::fs::OpenOptions;
use std::io::{self, ErrorKind};
use std::path::PathBuf;

// Custom error type for our application
#[derive(Debug)]
enum AppError {
    Usage,
    IoError(io::Error),
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::IoError(error)
    }
}

struct Config {
    filename: PathBuf,
}

impl Config {
    fn from_args() -> Result<Self, AppError> {
        let args: Vec<String> = env::args().collect();
        
        if args.len() < 2 || args[1] == "--help" {
            return Err(AppError::Usage);
        }

        Ok(Config {
            filename: PathBuf::from(&args[1]),
        })
    }
}

fn create_exclusive_file(filename: &PathBuf) -> io::Result<()> {
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)
    {
        Ok(_file) => {
            println!(
                "[PID {}] Successfully created file \"{}\" exclusively",
                std::process::id(),
                filename.display()
            );
            Ok(())
        }
        Err(e) if e.kind() == ErrorKind::AlreadyExists => {
            println!(
                "[PID {}] File \"{}\" already exists",
                std::process::id(),
                filename.display()
            );
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn run() -> Result<(), AppError> {
    let config = Config::from_args()?;
    create_exclusive_file(&config.filename)?;
    Ok(())
}

fn main() {
    match run() {
        Ok(()) => (),
        Err(AppError::Usage) => {
            eprintln!("Usage: {} <filename>", env::args().next().unwrap_or_default());
            std::process::exit(1);
        }
        Err(AppError::IoError(e)) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}