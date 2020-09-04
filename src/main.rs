use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};


fn handle_incoming(mut incoming: TcpStream) {
    let mut data = [0 as u8; 50];
    while match incoming.read(&mut data) {
        Ok(size) => { 
            incoming.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", incoming.peer_addr().unwrap());
            incoming.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();

    for incoming in listener.incoming() {
        match incoming {
            Ok(incoming) => {
                println!("New connection: {}", incoming.peer_addr().unwrap());
                thread::spawn(move|| {
                    handle_incoming(incoming)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }   
    drop(listener); // close
}
