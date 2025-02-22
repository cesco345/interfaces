use std::process;

fn main() {
    // Method 1: Using Rust's standard library (which wraps the system call)
    let pid = process::id();
    println!("Process ID (using std): {}", pid);

    // Method 2: Direct system call using libc
    unsafe {
        let raw_pid = libc::getpid();
        println!("Process ID (using syscall): {}", raw_pid);
    }
}
 
 