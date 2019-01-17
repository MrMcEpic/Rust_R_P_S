use wincolor::{Console, Color, Intense};
use winconsole::console;
use std::{thread::sleep, time::Duration};
 
pub fn text(){
	let mut con = Console::stdout().unwrap();
	console::set_title("Rock Paper Scissors").unwrap();

	if let Some((w, _h)) = term_size::dimensions() {
		con.fg(Intense::Yes, Color::Cyan).unwrap();
		sleep(Duration::from_millis(500));

		println!("{holder:^width$}", width=w, holder="~-==Rock Paper Scissors==-~");

		con.reset().unwrap();
		sleep(Duration::from_millis(1000));
		con.fg(Intense::Yes, Color::Green).unwrap();

		println!("{holder:^width$}", width=w, holder="~-==Now built in Rust!==-~");

		sleep(Duration::from_millis(1000));
		con.reset().unwrap();
	} else {
		con.fg(Intense::Yes, Color::Cyan).unwrap();
		sleep(Duration::from_millis(500));

		println!("{holder:^width$}", width=110, holder="~-==Rock Paper Scissors==-~");

		con.reset().unwrap();
		sleep(Duration::from_millis(1000));
		con.fg(Intense::Yes, Color::Green).unwrap();

		println!("{holder:^width$}", width=110, holder="~-==Now built in Rust!==-~");

		sleep(Duration::from_millis(1000));
		con.reset().unwrap();
	}	
}