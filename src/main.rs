use std::thread::spawn;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::io;

mod engine;

fn main() {
	let (cin_tx, cin_rx): (Sender<String>, Receiver<String>) = mpsc::channel();

	spawn(move || {
		engine::start(cin_rx)
	});

	let mut input: String = String::new();

	loop {
		match io::stdin().read_line(&mut input) {
			Ok(_n) => {
				match cin_tx.send(input.trim().to_string()) {
					Ok(_n) => {}
					Err(error) => {
						println!("Channel error: {}", error);
						break;
					}
				}
			}
			Err(error) => {
				println!("Line read error: {}", error);
				break;
			}
		}
		input=String::new();
	}
}
