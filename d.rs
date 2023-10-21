use std::net::UdpSocket;
use std::thread;
use std::fs::File;
use std::io::{Read, Result};

fn main() -> Result<()> {
    // Bind the UDP socket to the local address and port
    let socket = UdpSocket::bind("0.0.0.0:9999").unwrap();
    
    // Read the TS file
    let mut file = File::open("test2.ts")?;
    
    // Create a buffer to hold each chunk
    let mut buf = vec![0u8; 60]; // Adjust the chunk size as needed
    
    loop {
        // Read a chunk from the file
        let bytes_read = file.read(&mut buf)?;

        println!("{:?}",bytes_read);
        println!("{:?}",&buf[0..bytes_read]);
        
        // If we've reached the end of the file, break the loop
        if bytes_read == 0 {
            break;
        }
        
        // Send the chunk over UDP to the multicast address and port
        socket.send_to(&buf[0..bytes_read], "234.2.2.2:8888").unwrap();
        
        // Sleep for a while (adjust as needed)
        // thread::sleep(std::time::Duration::from_millis(125));
    }
    
    Ok(())
}
