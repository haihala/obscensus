use std::thread::spawn;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::io;

mod console;

fn main() {
	let (cin_tx, cin_rx): (Sender<String>, Receiver<String>) = mpsc::channel();

	spawn(move || {
		console::start(cin_rx)
	});

	let mut input: String = String::new();

	while input != "quit" {
		match io::stdin().read_line(&mut input) {
			Ok(n) => {
				match cin_tx.send(input.trim().to_string()) {
					Ok(_result) => {}
					Err(error) => println!("error: {}", error)
				}
			}
			Err(error) => println!("error: {}", error)
		}
		input=String::new();
	}
}
