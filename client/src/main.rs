mod challenge;
mod player_init;
mod server_communication;

use rand::Rng;
use std::io::Read;
use std::net::TcpStream;

use crate::challenge::md5_hash_cash::md5_challenge_resolver;
use crate::challenge::monstrous_maze::maze_challenge_resolver;
use crate::challenge::recover_secret::secret_challenge_resolver;
use crate::player_init::{on_subscribe_result, on_welcome};
use crate::server_communication::send_message;
use common::models::{
    Challenge, ChallengeAnswer, ChallengeResult, EndOfGame, Message, PublicPlayer, RoundSummary,
};

struct GameState {
    pub name: String,
    pub players: Vec<PublicPlayer>,
    pub md5_thread: u64,
}

fn on_leader_board(leader_board: &Vec<PublicPlayer>) {
   // println!("LeaderBoard: {leader_board:?}");
}

fn on_challenge(stream: &TcpStream, challenge: Challenge, game_state: &GameState) {
    let challenge_answer: ChallengeAnswer;
    let mut rng = rand::thread_rng();

    match challenge {
        Challenge::MD5HashCash(input) => {
            challenge_answer = ChallengeAnswer::MD5HashCash(md5_challenge_resolver(input, game_state.md5_thread));
        }
        Challenge::MonstrousMaze(input) => {
            challenge_answer = ChallengeAnswer::MonstrousMaze(maze_challenge_resolver(input));
        }
        Challenge::RecoverSecret(input) => {
            challenge_answer = ChallengeAnswer::RecoverSecret(secret_challenge_resolver(input));
        }
    }

    let mut index = rng.gen_range(0..game_state.players.len());
    while game_state.players[index].name == game_state.name {
        index = rng.gen_range(0..game_state.players.len());
    }

    let next_target = game_state.players[index].name.clone();

    let challenge_result = ChallengeResult {
        answer: challenge_answer,
        next_target,
    };
    send_message(stream, Message::ChallengeResult(challenge_result));
}

fn on_round_summary(stream: &TcpStream, summary: RoundSummary) {
  //  println!("RoundSummary: {summary:?}");
}

fn on_end_of_game(end_of_game: EndOfGame) {
   // println!("EndOfGame: {end_of_game:?}");
}

fn main_loop(mut stream: &TcpStream, game_state: &mut GameState) {
    let mut buf = [0; 4];

    send_message(stream, Message::Hello);

    println!("Listening");

    loop {
        match stream.read_exact(&mut buf) {
            Ok(_) => {}
            Err(_) => {
                continue;
            }
        }
        //  println!("receiving message");

        let message_size = u32::from_be_bytes(buf);
        //     println!("message_size: {message_size:?}");

        let mut message_buf = vec![0; message_size as usize];
        stream
            .read_exact(&mut message_buf)
            .expect("failed to read message");

        let record = buffer_to_object(&mut message_buf);

        match record {
            Message::Hello => {}
            Message::Welcome(welcome) => on_welcome(stream, welcome, &game_state.name),
            Message::Subscribe(_) => {}
            Message::SubscribeResult(subscribe_result) => {
                on_subscribe_result(subscribe_result);
            }
            Message::PublicLeaderBoard(leader_board) => {
                game_state.players = leader_board;
                on_leader_board(&game_state.players);
            }
            Message::Challenge(challenge) => {
                on_challenge(stream, challenge, &game_state);
            }
            Message::RoundSummary(summary) => {
                on_round_summary(stream, summary);
            }
            Message::EndOfGame(end_of_game) => {
                on_end_of_game(end_of_game);
                break;
            }
            Message::ChallengeResult(_) => {}
        }
    }
}

fn buffer_to_object(message_buf: &mut Vec<u8>) -> Message {
    let message = std::str::from_utf8(&message_buf).expect("failed to parse message");
    //  println!("message: {message:?}");

    let record: Message = serde_json::from_str(&message).expect("failed to serialize message");
    //  println!("message: {record:?}");
    record
}

fn main() {
    let name = std::env::args().nth(1).expect("no name given");
    let mut md5_nb_thread: u64 = 10;

    for mut i in 2..std::env::args().len() {
        if std::env::args().nth(i).unwrap() == "--md5t" {
            i += 1;
            md5_nb_thread = std::env::args().nth(i).unwrap().parse().unwrap();
        }
    }

    println!("test: {md5_nb_thread}");
    let mut game_state = GameState {
        players: vec![],
        name: name.clone(),
        md5_thread: md5_nb_thread,
    };

    let stream = TcpStream::connect("localhost:7878");


    match stream {
        Ok(stream) => {
            main_loop(&stream, &mut game_state);
        }
        Err(err) => panic!("Cannot connect: {err}"),
    }
}
