use std::io::{Read, Write};
use std::net::TcpStream;
use std::{io, str};
use serde::{Serialize, Deserialize};
use hashcash::{Stamp, check};


pub fn receive(stream: &mut TcpStream, mut array: [u8; 4]) {
    stream.read( &mut array).unwrap();

    let size_message: u32 = u32::from_be_bytes(array);
    let size_message = size_message as usize;

    println!("{}", size_message);

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

pub fn send(stream: &mut TcpStream, message_to_send: Message) {
    let message_to_serialized = serde_json::to_string(&message_to_send);
    let message_to_serialized = message_to_serialized.unwrap();
    let serialized_message_length_to_u32 = (message_to_serialized.len()) as u32;

    stream.write_all(&serialized_message_length_to_u32.to_be_bytes()).unwrap();

    stream.write_all(&message_to_serialized.as_bytes()).expect("Broken Pipe");
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Welcome{
    version: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscribe {
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscribeResult {
    Ok,
    Err(SubscribeError)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(PublicLeaderBoard),
    Challenge(Challenge),
    ChallengeResult(ChallengeResult),
    ChallengeAnswer(ChallengeAnswer),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicLeaderBoard(Vec<PublicPlayer>);

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicPlayer {
    name: String,
    stream_id: String,
    score: i32,
    steps: u32,
    is_active: bool,
    total_used_time: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Challenge {
    MD5HashCash(MD5HashCashInput),
}



#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeResult {
    pub answer: ChallengeAnswer,
    pub next_target: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeValue {
    Unreachable,
    Timeout,
    BadResult(BadResult),
    OK(Ok)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BadResult {
    used_time: f64,
    next_target: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ok {
    used_time: f64,
    next_target: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MD5HashCashInput {
    complexity: u32,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MD5HashCashOutput {
    seed: u64,
    hashcode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundSummary {
    challenge: String,
    chain: Vec<ReportedChallengeResult>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportedChallengeResult {
    name: String,
    value: ChallengeValue
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeAnswer {
    MD5HashCash(MD5HashCashOutput)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndOfGame {
    leader_board: PublicLeaderBoard
}