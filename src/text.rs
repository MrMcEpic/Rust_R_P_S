use std::{thread::sleep, time::Duration};
use wincolor::{Color, Console, Intense};
use winconsole::console;

pub fn text() {
	let mut con = Console::stdout().unwrap();
	console::set_title("Rock Paper Scissors").unwrap();

	if let Some((w, _h)) = term_size::dimensions() {
		con.fg(Intense::Yes, Color::Cyan).unwrap();
		sleep(Duration::from_millis(500));

		println!(
			"{holder:^width$}",
			width = w,
			holder = "~-==Rock Paper Scissors==-~"
		);

		con.reset().unwrap();
		sleep(Duration::from_millis(1000));
		con.fg(Intense::Yes, Color::Green).unwrap();

		println!(
			"{holder:^width$}",
			width = w,
			holder = "~-==Now built in Rust!==-~"
		);

		sleep(Duration::from_millis(1000));
		con.reset().unwrap();
	} else {
		con.fg(Intense::Yes, Color::Cyan).unwrap();
		sleep(Duration::from_millis(500));

		println!(
			"{holder:^width$}",
			width = 110,
			holder = "~-==Rock Paper Scissors==-~"
		);

		con.reset().unwrap();
		sleep(Duration::from_millis(1000));
		con.fg(Intense::Yes, Color::Green).unwrap();

		println!(
			"{holder:^width$}",
			width = 110,
			holder = "~-==Now built in Rust!==-~"
		);

		sleep(Duration::from_millis(1000));
		con.reset().unwrap();
	}
}

pub fn to_title(s1: &str) -> String {
	let mut v: Vec<char> = s1.chars().collect();
	v[0] = v[0].to_uppercase().nth(0).unwrap();
	let s2: String = v.into_iter().collect();
	s2
}
