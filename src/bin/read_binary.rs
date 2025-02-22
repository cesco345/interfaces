use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::os::unix::io::AsRawFd;
use libc::{self, iovec};
use std::mem;

const STR_SIZE: usize = 100;

#[derive(Debug)]
struct ReadVExample {
   my_struct: libc::stat,    // First buffer
   x: i32,                   // Second buffer
   str_buf: [u8; STR_SIZE],  // Third buffer
}

impl ReadVExample {
   fn new() -> Self {
       ReadVExample {
           my_struct: unsafe { mem::zeroed() },
           x: 0,
           str_buf: [0; STR_SIZE],
       }
   }

   fn prepare_iovec(&mut self) -> Vec<iovec> {
       vec![
           iovec {
               iov_base: &mut self.my_struct as *mut _ as *mut libc::c_void,
               iov_len: mem::size_of::<libc::stat>(),
           },
           iovec {
               iov_base: &mut self.x as *mut _ as *mut libc::c_void,
               iov_len: mem::size_of::<i32>(),
           },
           iovec {
               iov_base: self.str_buf.as_mut_ptr() as *mut libc::c_void,
               iov_len: STR_SIZE,
           },
       ]
   }

   fn total_bytes_required(&self) -> usize {
       mem::size_of::<libc::stat>() + mem::size_of::<i32>() + STR_SIZE
   }

   fn print_details(&self) {
       println!("\nDetailed Data Read:");
       println!("==================");
       
       // Format stat struct info
       println!("\n1. Stat Struct:");
       println!("   Size: {} bytes", mem::size_of::<libc::stat>());
       
       // Format integer
       println!("\n2. Integer Value:");
       println!("   Value: {}", self.x);
       println!("   Size: {} bytes", mem::size_of::<i32>());
       
       // Format string buffer
       println!("\n3. String Buffer:");
       println!("   Raw size: {} bytes", STR_SIZE);
       println!("   Content: \"{}\"", String::from_utf8_lossy(
           &self.str_buf.split(|&x| x == 0).next().unwrap_or(&[])
       ));
       
       // Calculate non-null content
       let content_len = self.str_buf.iter()
           .position(|&x| x == 0)
           .unwrap_or(STR_SIZE);
       println!("   Actual content length: {} bytes", content_len);
       println!("   Null padding: {} bytes", STR_SIZE - content_len);
   }
}

fn read_vectored(file: &File, iov: &mut [iovec]) -> io::Result<usize> {
   let fd = file.as_raw_fd();
   let result = unsafe {
       libc::readv(fd, iov.as_ptr(), iov.len() as i32)
   };

   if result == -1 {
       Err(io::Error::last_os_error())
   } else {
       Ok(result as usize)
   }
}

fn main() -> io::Result<()> {
   let args: Vec<String> = env::args().collect();

   if args.len() != 2 {
       eprintln!("Usage: {} <file>", args[0]);
       std::process::exit(1);
   }

   // Open the file
   let file = File::open(&args[1])?;
   
   // Initialize our data structure
   let mut example = ReadVExample::new();
   
   // Prepare the IO vectors
   let mut iov = example.prepare_iovec();
   
   // Calculate total bytes required
   let total_required = example.total_bytes_required();

   // Perform the vectored read
   match read_vectored(&file, &mut iov) {
       Ok(num_read) => {
           println!("Read Operation Summary:");
           println!("======================");
           println!("Total bytes requested: {}", total_required);
           println!("Bytes actually read: {}", num_read);

           if num_read < total_required {
               println!("\nWARNING: Partial read occurred!");
               println!("Missing bytes: {}", total_required - num_read);
           }

           example.print_details();
       }
       Err(e) => {
           eprintln!("Error during readv: {}", e);
           return Err(e);
       }
   }

   Ok(())
}