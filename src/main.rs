use rand::Rng;
use std::{io, thread::sleep, time::Duration};

mod text;
use text::to_title;

fn again() {
	println!("Play Again? Y/N");
	let choice = loop {
		let mut choice = String::new();

		io::stdin()
			.read_line(&mut choice)
			.expect("Failed to read line");

		choice = choice.trim().to_ascii_lowercase();

		if choice == "y" || choice == "n" {
			break choice;
		} else if choice == "yes" {
			choice = "y".to_string();
			break choice;
		} else if choice == "no" {
			choice = "n".to_string();
			break choice;
		}
	};
	if choice == "y" {
		game();
	} else {
		println!("Press enter to close");
		let mut _enter = String::new();
		io::stdin().read_line(&mut _enter).expect("Failed");
	}
}

fn game() {
	let user = choice_input();
	println!("You chose {}!", to_title(&user));
	sleep(Duration::from_millis(500));
	let comp = computer_choice();
	println!("Computer chose {}!", to_title(&comp));
	sleep(Duration::from_millis(300));
	let mut result = logic(&user, &comp);
	let you = if result == "draw" {
		result = to_title(&result);
		None
	} else {
		Some("You ")
	};
	let exclaim = if result == "win" { Some("!") } else { None };
	println!(
		"{}{}{}",
		if let Some(i) = you { i } else { "" },
		result,
		if let Some(i) = exclaim { i } else { "" }
	);
	again();
}

fn main() {
	text::text();
	game();
}

fn logic(user: &str, comp: &str) -> String {
	let return_val = if user == comp {
		"draw"
	} else {
		match user {
			"rock" => {
				if comp == "paper" {
					"lose"
				} else {
					"win"
				}
			}
			"paper" => {
				if comp == "scissors" {
					"lose"
				} else {
					"win"
				}
			}
			"scissors" => {
				if comp == "rock" {
					"lose"
				} else {
					"win"
				}
			}
			_ => "",
		}
	};
	return_val.to_string()
}

fn computer_choice() -> String {
	let comp_num = rand::thread_rng().gen_range(1, 4);
	let _comp_val = match comp_num {
		1 => "rock",
		2 => "paper",
		3 => "scissors",
		_ => "broken",
	};
	_comp_val.to_string()
}

fn choice_input() -> String {
	loop {
		println!("Rock/Paper/Scissors?");

		let mut choice = String::new();

		io::stdin()
			.read_line(&mut choice)
			.expect("Failed to read line");

		choice = choice.trim().to_ascii_lowercase();

		if choice == "rock" || choice == "paper" || choice == "scissors" {
			break choice; //returns choice to loop which then returns to function call
		}
	} //no ; here means it returns loop value to where it was called
}
