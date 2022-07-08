use std::{io, str};
use std::io::{Read, Write};
use std::net::TcpStream;
use serde::{Serialize, Deserialize};
use hashcash::{Stamp, check};
use serde_json::Error;
use shared::{Message, receive, send, Welcome, Subscribe, SubscribeResult, PublicLeaderBoard, Challenge, ChallengeResult, ChallengeAnswer, RoundSummary, EndOfGame, SubscribeError, MD5HashCashOutput, md5hash_cash, recover_secret};
use shared::Message::Hello;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:7878");
    match stream {
        Ok(mut stream) => {
            let hello = Message::Hello;
            send(&mut stream, hello);

            let array = [0; 4];

            loop {
                let challenge = receive(&mut stream, array);

                match challenge {
                    Ok(v) => {
                        if let Message::Welcome(..) = v {
                            let subscribe = Message::Subscribe(Subscribe { name: "lucas2".parse().unwrap() });
                            send(&mut stream, subscribe);
                        }
                        if let Message::PublicLeaderBoard(..) = v {
                            println!("public")
                        }
                        if let Message::EndOfGame(..) = v {
                            break;
                        }
                        if let Message::Challenge(challenge) = v {
                            println!("challenge a effectué : {:?}", challenge);
                            loop {
                                match challenge.clone() {
                                    Challenge::RecoverSecret(recover) => {
                                        let recover_secret_answer = recover_secret(recover);

                                        let challenge_result = Message::ChallengeResult(ChallengeResult { answer: ChallengeAnswer::RecoverSecret(recover_secret_answer), next_target: "lucas".parse().unwrap() });
                                        send(&mut stream, challenge_result);
                                    }

                                    Challenge::MD5HashCash(hashcash) => {
                                        let complexity = hashcash.complexity;
                                        let message = hashcash.message;

                                        let md5answer = md5hash_cash(complexity, message);

                                        println!("reponse du challenge {:?}", md5answer);

                                        let challenge_result = Message::ChallengeResult(ChallengeResult { answer: ChallengeAnswer::MD5HashCash(md5answer), next_target: "lucas".parse().unwrap() });
                                        send(&mut stream, challenge_result);

                                        break;
                                    }

                                    _ => {}
                                }
                            }
                        }
                    }
                    _ => {
                        println!("{:?}", challenge);
                        break;
                    }
                }
            }
        }
        Err(err) => panic!("Cannot connect: {}", err)
    }
}


