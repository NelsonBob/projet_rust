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

    let mut algorithm_is_good = false;
    let mut seed_output: u64 = 0;
    let mut hash_code: String = "".to_string();

    while(!algorithm_is_good) {
        let seed: u64 = rand::thread_rng().gen_range(0..10000000000000000);

        let concatenation_seed_and_message: String = seed.to_string() + &*message;

        let digest: Digest = md5::compute(concatenation_seed_and_message);

        let hex_hashcode = format_digest_to_hex(digest);
        let bin_hashcode = format_hex_to_binary(hex_hashcode);

        let size_message = complexity as usize;

        let mut slice = &bin_hashcode[0..size_message];

        for c in slice.chars() {
            if c != '0' {
                break;
            }
            slice = &slice[1..size_message];
        }

        if(slice.len() == 0){
            algorithm_is_good = true;
            seed_output = seed;
            hash_code = hex_hashcode;
        }
    }



    println!("{:?}", slice);

     return MD5HashCashOutput{ seed: seed_output, hashcode: hash_code };
}

fn format_digest_to_hex(digest: Digest) -> String {
    format!("{:032X}", digest)
}

fn format_hex_to_binary(hashcode: String) -> String {
    hashcode.chars().map(to_binary).collect()
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
    RecoverSecret(RecoverSecretOutput),
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
// fonction de recover secret
pub fn RecoverSecret(input: RecoverSecretInput) -> RecoverSecretOutput {
    return RecoverSecretOutput {
        secret_sentence: String::from(""),
    };
}

//struct imput
pub struct RecoverSecretInput {
    pub word_count: usize,
    pub letters: String,
    pub tuple_sizes: Vec<usize>,
}

// struct output

pub struct RecoverSecretOutput {
    pub secret_sentence: String,
}