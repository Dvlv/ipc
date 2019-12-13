use std::thread;
use std::time::Duration;

extern crate zmq;

fn reverse_word(word: &str) -> String {
    let rev: String = word.chars().rev().collect();

    rev
}

fn main() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5555").is_ok());

    let mut msg = zmq::Message::new();

    loop {
        responder.recv(&mut msg, 0).unwrap();
        println!("received: {}", msg.as_str().unwrap());
        thread::sleep(Duration::from_millis(1000));
        let response: String = reverse_word(msg.as_str().unwrap());
        responder.send(response.as_bytes(), 0).unwrap();
    }
}

