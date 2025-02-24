use std::env;
use std::fs::OpenOptions;
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::process;

fn print_usage(program: &str) {
    eprintln!("Usage: {} file {{r<length>|R<length>|w<string>|s<offset>}}...", program);
    process::exit(1);
}

#[derive(Debug)]
enum FileError {
    Io(io::Error),
    Parse(String),
}

impl From<io::Error> for FileError {
    fn from(err: io::Error) -> FileError {
        FileError::Io(err)
    }
}

fn parse_number(s: &str) -> Result<u64, FileError> {
    s.parse::<u64>()
        .map_err(|_| FileError::Parse(format!("Invalid number: {}", s)))
}

fn process_file(filename: &str, commands: &[String]) -> Result<(), FileError> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filename)?;

    for cmd in commands {
        if cmd.is_empty() {
            continue;
        }

        match cmd.chars().next().unwrap() {
            'r' | 'R' => {
                let len = parse_number(&cmd[1..])?;
                let mut buffer = vec![0u8; len as usize];
                
                match file.read(&mut buffer) {
                    Ok(0) => println!("{}: end-of-file", cmd),
                    Ok(n) => {
                        print!("{}: ", cmd);
                        if cmd.starts_with('r') {
                            for byte in &buffer[..n] {
                                let c = *byte as char;
                                print!("{}", if c.is_ascii_graphic() { c } else { '?' });
                            }
                        } else {
                            for byte in &buffer[..n] {
                                print!("{:02x} ", byte);
                            }
                        }
                        println!();
                    }
                    Err(e) => return Err(e.into()),
                }
            }

            'w' => {
                let data = &cmd[1..];
                let bytes_written = file.write(data.as_bytes())?;
                println!("{}: wrote {} bytes", cmd, bytes_written);
            }

            's' => {
                let offset = parse_number(&cmd[1..])?;
                file.seek(SeekFrom::Start(offset))?;
                println!("{}: seek succeeded", cmd);
            }

            _ => return Err(FileError::Parse(
                format!("Invalid command (must start with r, R, w, or s): {}", cmd)
            )),
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args[1] == "--help" {
        print_usage(&args[0]);
    }

    let filename = &args[1];
    let commands = &args[2..];

    if let Err(err) = process_file(filename, commands) {
        eprintln!("Error: {:?}", err);
        process::exit(1);
    }
}