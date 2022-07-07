use std::io::{Read, Write};
use std::net::TcpStream;
use std::{io, str};
use serde::{Serialize, Deserialize};
use hashcash::{Stamp, check};
use md5::Digest;
use serde_json::Error;
use rand::Rng;


pub fn receive(stream: &mut TcpStream, mut array: [u8; 4]) -> Result<Message, Error> {
    stream.read( &mut array).unwrap();

    let size_message: u32 = u32::from_be_bytes(array);
    let size_message = size_message as usize;

    let mut vector = vec![0; size_message];

    stream.read(&mut vector).unwrap();

    let message_received = std::str::from_utf8(&*vector).unwrap();

    let welcome_serialized = serde_json::to_string(&message_received).unwrap();
    let a = welcome_serialized.replace("\\", "");


    let first_last_off: &str = &a[1..a.len() - 1];
    let message: Result<Message, serde_json::Error> = serde_json::from_str(&first_last_off);

    println!("{:?}", &message);
   return message
}

pub fn send(stream: &mut TcpStream, message_to_send: Message) {
    let message_to_serialized = serde_json::to_string(&message_to_send);
    let message_to_serialized = message_to_serialized.unwrap();
    let serialized_message_length_to_u32 = (message_to_serialized.len()) as u32;

    stream.write_all(&serialized_message_length_to_u32.to_be_bytes()).unwrap();

    stream.write_all(&message_to_serialized.as_bytes()).expect("Broken Pipe");
}

pub fn md5hash_cash(complexity: u32, message: String) -> MD5HashCashOutput {

    let mut finish = false;
    let mut seed: u64 = 0;
    let mut hash_code = "".to_string();

    while finish == false {
        let hex_seed = format_dec_to_hex(seed as i32);
        let concat_seed = concat_string(hex_seed.to_string(), &message);
        let digest = md5::compute(concat_seed);
        hash_code = format_digest_to_hex(digest);
        let mut binary_hash: String = format_to_binary(&hash_code);
        finish = check_seed(binary_hash, complexity);
        seed += 1;
    }

     return MD5HashCashOutput{ seed, hashcode: hash_code.parse().unwrap() };
}

fn concat_string(seed: String, message: &String) -> String {
    format!("{}{}\n", seed, message)
}

fn format_dec_to_hex(seed: i32) -> String {
    format!("{:016X}", seed)
}

fn format_digest_to_hex(digest: Digest) -> String {
    format!("{:032X}", digest)
}

fn format_to_binary(hashcode: &String) -> String {
    hashcode.chars().map(to_binary).collect()
}

fn check_seed(binaryHash: String, complexity: u32) -> bool {
    let mut index = 0;
    for character in binaryHash.chars() {
        if character == '1' && index < complexity {
            print!("false\n");
            return false;
        } else if index >= complexity {
            print!("good ");//envoie du resultat au server
            return true;
        }
        index += 1;
    }
    return false;
}


fn to_binary(c: char) -> String {
    let b = match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    };
    return String::from(b);
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
    MD5HashCashInput(MD5HashCashInput),
    ChallengeResult(ChallengeResult),
    ChallengeAnswer(ChallengeAnswer),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicLeaderBoard(pub Vec<PublicPlayer>);

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicPlayer {
    pub name: String,
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
    pub complexity: u32,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MD5HashCashOutput {
    pub seed: u64,
    pub hashcode: String,
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