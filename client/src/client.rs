use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let stream = std::net::TcpStream::connect("127.0.0.1:7878");
    match stream {
        Ok(mut stream ) => {

            let message_to_send = "Hello";

            send(&mut stream, &message_to_send);

            let mut array = [0; 4];

            stream.read( &mut array).unwrap();

            receive(&mut stream, array);
        }
        Err(err) => panic!("Cannot connect: {err}")
    }

}

fn receive(stream: &mut TcpStream, mut array: [u8; 4]) {
    let size_message: u32 = u32::from_be_bytes(array);
    let size_message = size_message as usize;

    let mut vector = vec![0; size_message];

    stream.read(&mut vector).unwrap();
    let message_received = std::str::from_utf8(&*vector).unwrap().to_string();
    println!("{}", message_received);
}

fn send(stream: &mut TcpStream, message_to_send: &str) {
    let message_to_serialized = serde_json::to_string(&message_to_send);
    let message_to_serialized = message_to_serialized.unwrap();
    let serialized_message_length_to_u32 = (message_to_serialized.len()) as u32;

    stream.write_all(&serialized_message_length_to_u32.to_be_bytes()).unwrap();

    stream.write_all(&message_to_serialized.as_bytes()).unwrap();
}