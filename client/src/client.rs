use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let stream = std::net::TcpStream::connect("127.0.0.1:7878");
    match stream {
        Ok(mut stream ) => {
            let message = "Hello";
            let serialized = serde_json::to_string(&message);
            let serialized = serialized.unwrap();
            let ch = (serialized.len()) as u32;
            stream.write_all(&ch.to_be_bytes()).unwrap();
            stream.write_all(&serialized.as_bytes()).unwrap();

            let mut v = [0; 4];

            stream.read( &mut v).unwrap();

            let back_to_u32: u32 = u32::from_be_bytes(v);
            let back_to_u32= back_to_u32 as usize;

            let mut a = vec![0; back_to_u32];

            stream.read(&mut a).unwrap();
            let finale = std::str::from_utf8(&*a).unwrap().to_string();
            println!("{}", finale);
        }
        Err(err) => panic!("Cannot connect: {err}")
    }

}