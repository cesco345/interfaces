use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::process;

const BUFFER_SIZE: usize = 8192; // 8KB buffer for reading

#[derive(Debug)]
enum CopyError {
    IoError(io::Error),
    Usage(String),
}

impl From<io::Error> for CopyError {
    fn from(err: io::Error) -> Self {
        CopyError::IoError(err)
    }
}

fn print_usage() {
    eprintln!("Usage: sparse_cp <source_file> <target_file>");
    process::exit(1);
}

fn copy_sparse_file(source_path: &str, target_path: &str) -> Result<(), CopyError> {
    // Open source file for reading
    let mut source = File::open(source_path)?;
    
    // Create target file with appropriate permissions
    let mut target = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(target_path)?;

    let mut buffer = vec![0u8; BUFFER_SIZE];
    let mut current_pos: u64 = 0;

    loop {
        // Read a chunk from source
        let bytes_read = source.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // End of file
        }

        // Check if this chunk is all zeros (a hole)
        let is_hole = buffer[..bytes_read].iter().all(|&byte| byte == 0);

        if is_hole {
            // For a hole, seek past it instead of writing zeros
            current_pos += bytes_read as u64;
            target.seek(SeekFrom::Start(current_pos))?;
        } else {
            // For non-zero data, write the chunk
            target.write_all(&buffer[..bytes_read])?;
            current_pos += bytes_read as u64;
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        print_usage();
    }

    let source_path = &args[1];
    let target_path = &args[2];

    if let Err(err) = copy_sparse_file(source_path, target_path) {
        eprintln!("Error: {:?}", err);
        process::exit(1);
    }

    println!("File copied successfully with holes preserved!");
}