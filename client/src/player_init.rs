use crate::server_communication::send_message;
use common::models::{Message, Subscribe, SubscribeResult, Welcome};
use std::net::TcpStream;

pub fn on_welcome(stream: &TcpStream, welcome: Welcome, name: &String) {
    println!("welcome: {welcome:?}");
    let subscribe = Subscribe { name: name.clone() };
    send_message(stream, Message::Subscribe(subscribe));
}

pub fn on_subscribe_result(subscribe_result: SubscribeResult) {
    println!("subscribe_result: {subscribe_result:?}");
}
