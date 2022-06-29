use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
use serde::{Serialize, Deserialize};

fn main() {
    let stream = std::net::TcpStream::connect("127.0.0.1:7878");
    match stream {
        Ok(mut stream ) => {

            let message_to_send = "Hello";
            send(&mut stream, &message_to_send);

            let array = [0; 4];
            receive(&mut stream, array);

            let subscribe = Message::Subscribe(Subscribe { name: "Lucas237".parse().unwrap() });
            let serialized = serde_json::to_string(&subscribe).unwrap();
            print!("{}" , serialized);
            let serialized_length_to_u32 = (serialized.len()) as u32;
            stream.write_all(&serialized_length_to_u32.to_be_bytes()).unwrap();
            stream.write_all(&serialized.as_bytes()).unwrap();

            let array_2 = [0; 4];
            receive(&mut stream, array_2);

        }
        Err(err) => panic!("Cannot connect: {err}")
    }

}

fn receive(stream: &mut TcpStream, mut array: [u8; 4]) {
    stream.read( &mut array).unwrap();

    let size_message: u32 = u32::from_be_bytes(array);
    let size_message = size_message as usize;

    let mut vector = vec![0; size_message];

    stream.read(&mut vector).unwrap();

    let message_received = std::str::from_utf8(&*vector).unwrap();

    let welcome_serialized = serde_json::to_string(&message_received).unwrap();

    let a = welcome_serialized.replace("\\", "");


    let first_last_off: &str = &a[1..a.len() - 1];

    let message: Result<Message, _> = serde_json::from_str(&first_last_off);

    match message {
        Ok(m) => println!("message={m:?}"),
        Err(err) => println!("error={err:?}")
    }

}

fn send(stream: &mut TcpStream, message_to_send: &str) {
    let message_to_serialized = serde_json::to_string(&message_to_send);
    let message_to_serialized = message_to_serialized.unwrap();
    let serialized_message_length_to_u32 = (message_to_serialized.len()) as u32;
    stream.write_all(&serialized_message_length_to_u32.to_be_bytes()).unwrap();
    stream.write_all(&message_to_serialized.as_bytes()).unwrap();
}





#[derive(Serialize, Deserialize, Debug)]
struct Welcome{
    version: i32
}
#[derive(Debug, Serialize, Deserialize)]
struct Subscribe {
    name: String
}
#[derive(Debug, Serialize, Deserialize)]
enum SubscribeResult {
    Ok,
    Err
}



#[derive(Debug, Serialize, Deserialize)]
enum Message {
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubscribeResult)
}