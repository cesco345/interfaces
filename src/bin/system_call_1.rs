use libc::{self, pid_t};
use std::io::Error;

fn get_parent_pid() -> Result<pid_t, Error> {
    unsafe {
        let ppid = libc::getppid();
        if ppid < 0 {
            return Err(Error::last_os_error());
        }
        Ok(ppid)
    }
}

fn main() {
    // Get current process ID
    let pid = unsafe { libc::getpid() };
    println!("Current Process ID: {}", pid);

    // Get parent process ID
    match get_parent_pid() {
        Ok(ppid) => println!("Parent Process ID: {}", ppid),
        Err(e) => eprintln!("Error getting parent PID: {}", e),
    }
}