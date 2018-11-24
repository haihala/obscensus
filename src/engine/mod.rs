use std::sync::mpsc::Receiver;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
struct Button {	// String is probably the wrong type. investigate later
	pub action: String,
}

struct Command {
	pub key: String,
	pub value: String,
}


fn prep() -> state::GameState {
	let player: state::Player = state::Player {};
	let ret: state::GameState = state::GameState {player};
	ret
}

fn run(gs: state::GameState, rx: Receiver<String>) {
	// Registry of buttons currently being pressed.
	let mut button_registry: HashSet<Button> = HashSet::new();
	loop {
		match read_input(rx.recv().unwrap()) {
			Some(x) => {
				let ((buttons_down, buttons_up), commands): ((Vec<Button>, Vec<Button>), Vec<Command>) = x;
				for button in buttons_down {
					// Insert returns true if set didn't and false if set contained the element.
					// Maybe do something with that.
					button_registry.insert(button);
				}

				for button in buttons_up {
					// Might not work, maybe does. We'll see.
					button_registry.remove(&button);
				}

				for command in commands {

				}
			}
			None => {}
		}
	}
}

fn read_input(raw_input: String) -> Option<((Vec<Button>, Vec<Button>), Vec<Command>)> {
	let (mut buttons_down, mut buttons_up): (Vec<Button>, Vec<Button>) = (Vec::new(), Vec::new());
	let mut commands: Vec<Command> = Vec::new();

	for cmd in raw_input.split(';') {
		let pressed: bool = cmd.starts_with('+');
		let action: String = cmd.trim_matches(char::is_numeric).to_string();
		if pressed || cmd.starts_with('-') {	// Certified +-<something>, a button press if you will.
			if cmd.contains(' '){
				()
			}
			if pressed {
				buttons_down.push(
					Button {action: action}
				);
			} else {
				buttons_up.push(
					Button {action: action}
				);
			}
		} else {
			// This might fail if splitn 1 doesn't return a 2 long vector for strings without spaces
			// If that happens, should be an easy fix.
			let kv_pair: Vec<&str> = cmd.splitn(1, ' ').collect();
			let key: String = String::from(kv_pair[0]);
			let value: String = String::from(kv_pair[1]);

			commands.push (
				Command {key: key, value: value}
			);
		}
	}

	Some(((buttons_down, buttons_up), commands))
}

pub fn start(rx: Receiver<String>) {
	let gs: state::GameState = prep();
	run(gs, rx);
}

mod state;