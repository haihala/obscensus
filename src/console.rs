use std::sync::mpsc::Receiver;


pub fn start(rx: Receiver<String>) {
	println!("start");
}