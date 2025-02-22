use std::fs::File;
use std::io::Write;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

fn create_test_file(filename: &str) -> std::io::Result<()> {
    let path = Path::new(filename);
    let mut file = File::create(path)?;

    // Create some sample data
    let mut data = Vec::new();

    // 1. First add a fake stat structure (just some random bytes matching the size)
    let stat_size = std::mem::size_of::<libc::stat>();
    data.extend_from_slice(&vec![0xAA; stat_size]);

    // 2. Add an integer
    let x: i32 = 12345;
    data.extend_from_slice(&x.to_ne_bytes());

    // 3. Add a string
    let test_str = "Hello, this is a test string!";
    let mut str_buffer = vec![0; 100];  // STR_SIZE from previous example
    str_buffer[..test_str.len()].copy_from_slice(test_str.as_bytes());
    data.extend_from_slice(&str_buffer);

    // Write all data at once
    file.write_all(&data)?;
    
    // Print information about what we wrote
    println!("Created test file: {}", filename);
    println!("Total bytes written: {}", data.len());
    println!("Structure:");
    println!("  - stat struct size: {} bytes", stat_size);
    println!("  - integer: {} ({} bytes)", x, std::mem::size_of::<i32>());
    println!("  - string buffer: {} bytes", str_buffer.len());
    
    // Get and print file metadata
    let metadata = path.metadata()?;
    println!("\nFile info:");
    println!("  - size: {} bytes", metadata.len());
    println!("  - permissions: {:o}", metadata.mode());
    println!("  - modified: {:?}", metadata.modified()?);

    Ok(())
}

fn main() -> std::io::Result<()> {
    create_test_file("test_readv.bin")?;
    Ok(())
}