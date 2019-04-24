use std::io::prelude::*;
use std::io::{self, Read};
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

// let listener_thread = thread::spawn(move || {
//     let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

//     for stream in listener.incoming() {
//         let mut stream = stream.unwrap();
//         stream.set_read_timeout(Some(Duration::from_millis(100))).unwrap();
//         let mut buf = [0];

//         let response = "HTTP/1.1 200 OK\r\n\r\nsdfuhgsjdfghsdfjk";
//         stream.write(response.as_bytes()).unwrap();
//         stream.flush().unwrap();
//     }
// });

fn main() {
    let print_thread = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        println!("nonono")
    });

    thread::sleep(Duration::from_secs(5));
    drop(print_thread);
    thread::sleep(Duration::from_secs(5));
}

// fn main() {
//     let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

//     for stream in listener.incoming() {
//         let mut stream = stream.unwrap();
//         stream.set_read_timeout(Some(Duration::from_millis(100))).unwrap();
//         let mut buf = [0];

//         loop {
//         let _ = match stream.read(&mut buf) {
//             Err(e) => {
//                 match e.kind() {
//                     io::ErrorKind::WouldBlock => {
//                         println!("would have blocked");
//                         break;
//                     },
//                     _ => panic!("Got an error: {}", e),
//                 }
//             },
//             Ok(m) => {
//                 println!("Received {:?}, {:?}", m, buf);
//                 if m == 0 {
//                     // doesn't reach here.
//                     break;
//                 }
//                 m
//             },
//         };
//     }

//         // let response = "HTTP/1.1 200 OK\r\n\r\nsdfuhgsjdfghsdfjk";
//         // stream.write(response.as_bytes()).unwrap();
//         // stream.flush().unwrap();
//     }
// }
