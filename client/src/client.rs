use std::{io, str};
use std::io::{Read, Write};
use std::net::TcpStream;
use serde::{Serialize, Deserialize};
use hashcash::{Stamp, check};
use shared::{Message, receive, send, Hello, Welcome, Subscribe, SubscribeResult, PublicLeaderBoard, Challenge, ChallengeResult, ChallengeAnswer, RoundSummary, EndOfGame, SubscribeError, MD5HashCash, MD5HashCashOutput};


fn main() {
    let stream = TcpStream::connect("127.0.0.1:7878");
    match stream {
        Ok(mut stream) => {
            let hello = Message::Hello;
            send(&mut stream, hello);

            let subscribe = Message::Subscribe(Subscribe { name: "l".parse().unwrap() });
            send(&mut stream, subscribe);

            let array = [0; 4];

            // welcome
            receive(&mut stream, array);

            // subscribe result
            receive(&mut stream, array);

            // leaderBoard
            receive(&mut stream, array);

            // challenge
            let challenge_result = Message::ChallengeResult(ChallengeResult { answer: ChallengeAnswer::MD5HashCash(MD5HashCashOutput { seed: 0, hashcode: "".to_string() }),  next_target: "".to_string() });
            send(&mut stream, challenge_result);

            // challenge result
            receive(&mut stream, array);

            // Round Summary
            receive(&mut stream, array);

            // leaderBoard
            receive(&mut stream, array);
        }
        Err(err) => panic!("Cannot connect: {}",err)
    }
}


