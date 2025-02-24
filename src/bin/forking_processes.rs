use std::os::unix::process::CommandExt;
use nix::unistd::{fork, ForkResult};
use nix::sys::wait::wait;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match unsafe { fork()? } {
        ForkResult::Child => {
            // Child process
            Command::new("echo")
                .arg("Hello, World!")
                .spawn()?
                .wait()?;
            Ok(())
        }
        ForkResult::Parent { child } => {
            println!("Parent watching child process {}", child);
            
            // Wait for child to complete
            match wait() {
                Ok(status) => {
                    println!("Child process finished with status: {:?}", status);
                }
                Err(e) => {
                    println!("Error waiting for child: {}", e);
                }
            }
            Ok(())
        }
    }
}