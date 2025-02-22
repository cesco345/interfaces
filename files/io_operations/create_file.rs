use std::fs::File;
use std::fs::OpenOptions;
use std::io;

fn main() -> io::Result<()> {
    // Method 1: Using File::create
    // This is the most straightforward equivalent to creat() in libc
    match File::create("file1.txt") {
        Ok(file) => println!("Successfully created file1.txt"),
        Err(e) => eprintln!("Failed to create file1.txt: {}", e),
    }

    // Method 2: Using OpenOptions
    // This gives you more control, equivalent to creat() with specific permissions
    match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("file2.txt")
    {
        Ok(file) => println!("Successfully created file2.txt"),
        Err(e) => eprintln!("Failed to create file2.txt: {}", e),
    }

    Ok(())
}